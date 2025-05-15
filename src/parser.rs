use crate::depot::{Krate, Krates};
use crate::depot::{KrateInfo, Tags};
use nom::bytes::complete::take_until;
use nom::bytes::take_till1;
use nom::character::complete::{alphanumeric1, multispace0};
use nom::character::complete::{char, space0};
use nom::character::complete::{multispace1, newline, space1, u32};
use nom::combinator::{map, opt, recognize};
use nom::error::ParseError;
use nom::multi::separated_list0;
use nom::sequence::delimited;
use nom::{IResult, Parser, multi::separated_list1};
use versions::SemVer;

pub trait Parsable {
    fn parse(s: &str) -> IResult<&str, Self>
    where
        Self: Sized;
}

impl Parsable for Krates {
    fn parse(s: &str) -> IResult<&str, Self> {
        let (s, krates) = separated_list1(newline, Krate::parse).parse(s)?;
        let k = Self(krates);

        Ok((s, k))
    }
}

impl Parsable for Krate {
    fn parse(s: &str) -> IResult<&str, Self> {
        let (s, name) = map(alphanumeric1_with_hyphen, String::from).parse(s)?;
        let (s, _) = multispace1(s)?;
        let (s, _) = char('v')(s)?;
        let (s, version) = SemVer::parse(s)?;
        let (s, _) = take_until("\n")(s)?;
        let (s, _) = newline(s)?;
        let (s, binaries) = separated_list1(newline, parse_binary).parse(s)?;

        let k = Self {
            name,
            version,
            binaries,
            ..Default::default()
        };

        Ok((s, k))
    }
}

impl Parsable for KrateInfo {
    fn parse(s: &str) -> IResult<&str, Self> {
        let (s, _) = alphanumeric1_with_hyphen(s)?;
        let (s, _) = multispace0(s)?;
        let (s, tags) = map(opt(Tags::parse), |t| t.unwrap_or_default()).parse(s)?;
        let (s, _) = multispace0(s)?;
        let (s, description) = map(take_till1(|c| c == '\n'), String::from).parse(s)?;

        let k = Self { tags, description };

        Ok((s, k))
    }
}

impl Parsable for Tags {
    /// > #foo #baz #bar
    fn parse(s: &str) -> IResult<&str, Self> {
        let (s, _) = char('#')(s)?;
        let (s, t) = map(
            separated_list0(char('#'), ws2(alphanumeric1_with_hyphen)),
            |v| v.into_iter().map(String::from).collect(),
        )
        .parse(s)?;

        Ok((s, Tags(t)))
    }
}

/// > \tdepot-rs
fn parse_binary(s: &str) -> IResult<&str, String> {
    let (s, _) = space1(s)?;
    let (s, b) = map(alphanumeric1_with_hyphen, String::from).parse(s)?;

    Ok((s, b))
}

impl Parsable for SemVer {
    /// > 3.12.0
    fn parse(s: &str) -> IResult<&str, Self> {
        let (s, major) = u32(s)?;
        let (s, _) = char('.')(s)?;
        let (s, minor) = u32(s)?;
        let (s, _) = char('.')(s)?;
        let (s, patch) = u32(s)?;

        Ok((
            s,
            SemVer {
                major,
                minor,
                patch,
                ..Default::default()
            },
        ))
    }
}

/// A parser that recognizes strings that contain `_` as a word.
fn alphanumeric1_with_hyphen(s: &str) -> IResult<&str, &str> {
    recognize(separated_list1(char('-'), alphanumeric1)).parse(s)
}

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace (excluding `\n`), returning the output of `inner`.
fn ws2<'a, O, E: ParseError<&'a str>, F>(inner: F) -> impl Parser<&'a str, Output = O, Error = E>
where
    F: Parser<&'a str, Output = O, Error = E>,
{
    delimited(space0, inner, space0)
}

#[cfg(test)]
mod tests {
    use crate::depot::{Krate, KrateInfo, Krates, Tags};
    use crate::parser::{Parsable, alphanumeric1_with_hyphen, parse_binary};
    use pretty_assertions::assert_eq;
    use versions::SemVer;

    #[test]
    fn parse_krates() {
        let s1 = r#"depot-rs v0.1.0:
        depot
        depot-rs
foo v0.1.0:
    foo"#;

        let s2 = r#"depot-rs v0.1.0:
        depot
        depot-rs"#;

        let s3 = r#"uv v0.6.16 (https://github.com/astral-sh/uv#43e5a6ef):
    uv
    uvx"#;

        assert_eq!(
            Krates::parse(s1).unwrap().1,
            Krates(vec![
                Krate {
                    name: "depot-rs".to_string(),
                    version: SemVer::parse("0.1.0").unwrap().1,
                    binaries: vec!["depot".to_string(), "depot-rs".to_string()],
                    ..Default::default()
                },
                Krate {
                    name: "foo".to_string(),
                    version: SemVer::parse("0.1.0").unwrap().1,
                    binaries: vec!["foo".to_string()],
                    ..Default::default()
                }
            ])
        );

        assert_eq!(
            Krates::parse(s2).unwrap().1,
            Krates(vec![Krate {
                name: "depot-rs".to_string(),
                version: SemVer::parse("0.1.0").unwrap().1,
                binaries: vec!["depot".to_string(), "depot-rs".to_string()],
                ..Default::default()
            },])
        );

        assert_eq!(
            Krates::parse(s3).unwrap().1,
            Krates(vec![Krate {
                name: "uv".to_string(),
                version: SemVer::parse("0.6.16").unwrap().1,
                binaries: vec!["uv".to_string(), "uvx".to_string()],
                ..Default::default()
            },])
        );
    }

    #[test]
    fn parse_krate() {
        let s1 = r#"depot-rs v0.1.0:
        depot
        depot-rs
            "#;

        let s2 = r#"depot-rs 0.1.0:
        depot
            "#;
        let s3 = "depot-rs v0.1.0:";

        let s4 = r#"
        depot-rs v0.1.0:
        depot
        depot-rs
            "#;

        assert_eq!(
            Krate::parse(s1).unwrap().1,
            Krate {
                name: "depot-rs".to_string(),
                version: SemVer::parse("0.1.0").unwrap().1,
                binaries: vec!["depot".to_string(), "depot-rs".to_string()],
                ..Default::default()
            }
        );

        assert!(Krate::parse(s2).is_err(),);
        assert!(Krate::parse(s3).is_err(),);
        assert!(Krate::parse(s4).is_err(),);
    }

    #[test]
    fn parse_single_binary() {
        assert_eq!(parse_binary("\tdepot-rs").unwrap().1, "depot-rs")
    }

    #[test]
    fn parse_tags() {
        assert_eq!(
            Tags::parse("#foo #bar #baz").unwrap().1,
            Tags(vec![
                "foo".to_string(),
                "bar".to_string(),
                "baz".to_string()
            ])
        );
    }

    #[test]
    fn parse_krate_info() {
        let output = r#"cargo-thesaurust #ratatui #dictionary #tui #terminal #thesaurus
        A terminal-based dictionary app.
        version: 0.1.2
        license: MIT
        rust-version: unknown
        documentation: https://docs.rs/cargo-thesaurust/0.1.2
        homepage: https://moreenh.me/pages/projects/cargo-thesaurust
        repository: https://github.com/quietpigeon/cargo-thesaurust
        crates.io: https://crates.io/crates/cargo-thesaurust/0.1.2
        "#;

        assert_eq!(
            KrateInfo::parse(output).unwrap().1,
            KrateInfo {
                tags: Tags(vec![
                    "ratatui".to_string(),
                    "dictionary".to_string(),
                    "tui".to_string(),
                    "terminal".to_string(),
                    "thesaurus".to_string()
                ]),
                description: "A terminal-based dictionary app.".to_string()
            }
        )
    }

    #[test]
    fn parse_krate_info_no_tags() {
        let output = r#"cargo-thesaurust
        A terminal-based dictionary app.
        version: 0.1.2
        license: MIT
        rust-version: unknown
        documentation: https://docs.rs/cargo-thesaurust/0.1.2
        homepage: https://moreenh.me/pages/projects/cargo-thesaurust
        repository: https://github.com/quietpigeon/cargo-thesaurust
        crates.io: https://crates.io/crates/cargo-thesaurust/0.1.2
        "#;

        assert_eq!(
            KrateInfo::parse(output).unwrap().1,
            KrateInfo {
                tags: Tags(vec![]),
                description: "A terminal-based dictionary app.".to_string()
            }
        )
    }

    #[test]
    fn parse_semver() {
        let v = SemVer::parse("1.12.0").unwrap().1;
        let semver = SemVer {
            major: 1,
            minor: 12,
            patch: 0,
            ..Default::default()
        };

        assert_eq!(v, semver);
        assert!(SemVer::parse("1").is_err());
        assert!(SemVer::parse("1.1").is_err());
    }

    #[test]
    fn parse_hyphenated() {
        assert_eq!(alphanumeric1_with_hyphen("depot-rs").unwrap().1, "depot-rs");
        assert_eq!(alphanumeric1_with_hyphen("depot").unwrap().1, "depot")
    }
}
