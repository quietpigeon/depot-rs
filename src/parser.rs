use crate::depot::{Krate, Krates};
use nom::bytes::complete::take_until;
use nom::character::complete::alphanumeric1;
use nom::character::complete::char;
use nom::character::complete::{multispace1, newline, space1, u32};
use nom::combinator::{map, recognize};
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
        let (s, name) = map(alphanumeric1_with_hyphen, |x| String::from(x)).parse(s)?;
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
        };

        Ok((s, k))
    }
}

/// > \tdepot-rs
fn parse_binary(s: &str) -> IResult<&str, String> {
    let (s, _) = space1(s)?;
    let (s, b) = map(alphanumeric1_with_hyphen, |x| String::from(x)).parse(s)?;

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

#[cfg(test)]
mod tests {
    use crate::{
        depot::{Krate, Krates},
        parser::{Parsable, alphanumeric1_with_hyphen, parse_binary},
    };
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
                    binaries: vec!["depot".to_string(), "depot-rs".to_string()]
                },
                Krate {
                    name: "foo".to_string(),
                    version: SemVer::parse("0.1.0").unwrap().1,
                    binaries: vec!["foo".to_string()]
                }
            ])
        );

        assert_eq!(
            Krates::parse(s2).unwrap().1,
            Krates(vec![Krate {
                name: "depot-rs".to_string(),
                version: SemVer::parse("0.1.0").unwrap().1,
                binaries: vec!["depot".to_string(), "depot-rs".to_string()]
            },])
        );

        assert_eq!(
            Krates::parse(s3).unwrap().1,
            Krates(vec![Krate {
                name: "uv".to_string(),
                version: SemVer::parse("0.6.16").unwrap().1,
                binaries: vec!["uv".to_string(), "uvx".to_string()]
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
                binaries: vec!["depot".to_string(), "depot-rs".to_string()]
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
