use crate::commands::{install_crate, list_crates, search_crate, uninstall_crate};
use crate::errors::{ChannelError, Error};
use crate::parser::{alphanumeric1_with_hyphen, ws, ws2};
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::char;
use nom::character::complete::multispace0;
use nom::character::complete::{multispace1, newline, space1};
use nom::combinator::{map, opt};
use nom::multi::separated_list0;
use nom::sequence::preceded;
use nom::{IResult, Parser, multi::separated_list1};
use ratatui::widgets::ListState;
use std::collections::HashSet;
use std::fmt::Display;
use throbber_widgets_tui::ThrobberState;
use versions::SemVer;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DepotMessage {
    FetchKrateInfo(Vec<KrateMetadata>),
    UpdateKrate { krate: String },
    UninstallKrate,
    DepotError(ChannelError),
}

impl DepotMessage {
    pub fn handle(self, state: &mut DepotState) -> Result<(), Error> {
        match self {
            DepotMessage::FetchKrateInfo(r) => state.sync(r)?,
            DepotMessage::UpdateKrate { krate } => state.update_krate(&krate)?,
            DepotMessage::UninstallKrate => {}
            DepotMessage::DepotError(e) => return Err(Error::HandleKrate(e)),
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct DepotState {
    pub depot: Depot,
    pub list_state: ListState,
    pub update_list_state: ListState,
    pub throbber_state: ThrobberState,
    update_queue: HashSet<String>,
}

impl Default for DepotState {
    fn default() -> Self {
        let depot = Depot::get().expect("failed to initialize `DepotState`");
        let list_state = ListState::default();
        let update_list_state = ListState::default();
        let throbber_state = throbber_widgets_tui::ThrobberState::default();
        let update_queue: HashSet<String> = HashSet::new();

        Self {
            depot,
            list_state,
            update_list_state,
            throbber_state,
            update_queue,
        }
    }
}

impl DepotState {
    pub fn is_all_synced(&self) -> bool {
        self.depot.store.0.iter().all(|k| k.is_metadata_synced())
    }

    pub fn update_krate(&mut self, name: &str) -> Result<(), Error> {
        if let Some(k) = self.depot.store.0.iter_mut().find(|k| k.name == name) {
            k.update_version()?;
            self.update_queue.remove(name);
        }

        Ok(())
    }

    pub fn get_update_items(&self) -> HashSet<String> {
        self.update_queue.clone()
    }

    pub fn append_to_update_queue(&mut self, krate: &str) {
        self.update_queue.insert(krate.to_string());
    }

    fn sync(&mut self, info: Vec<KrateMetadata>) -> Result<(), Error> {
        for krate in &mut self.depot.store.0 {
            if let Some(ki) = info.iter().find(|&i| krate.name == i.name) {
                krate.metadata = ki.clone();
            } else {
                return Err(Error::Unexpected("unmatched name".to_string()));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct Depot {
    pub store: Krates,
}

impl Depot {
    pub fn crate_count(&self) -> i64 {
        self.store.0.len() as i64
    }

    /// Obtain the list of installed crates.
    pub fn get() -> Result<Self, Error> {
        let output = list_crates()?;
        let store = Krates::parse(&output)?.1;

        Ok(Self { store })
    }

    pub fn get_outdated_krates(&self) -> Result<Krates, Error> {
        let k = self
            .store
            .0
            .clone()
            .into_iter()
            .filter(|k| !k.is_latest())
            .collect();

        Ok(Krates(k))
    }

    /// Shorthand for getting the number of outdated krates.
    pub fn outdated_krate_count(&self) -> Result<usize, Error> {
        Ok(self.get_outdated_krates()?.0.len())
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Krates(pub Vec<Krate>);

impl Krates {
    fn parse(s: &str) -> IResult<&str, Krates> {
        let (s, krates) = separated_list0(newline, Krate::parse).parse(s)?;
        let k = Krates(krates);

        Ok((s, k))
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Krate {
    pub name: String,
    pub version: SemVer,
    pub binaries: Vec<String>,
    metadata: KrateMetadata,
}

impl Krate {
    pub fn description(&self) -> String {
        if let Some(description) = &self.metadata.info.description {
            description.clone()
        } else {
            "not available".to_string()
        }
    }

    pub fn tags_str(&self) -> String {
        if let Some(tags) = &self.metadata.info.tags
            && !tags.0.is_empty()
        {
            tags.to_string()
        } else {
            "".to_string()
        }
    }

    pub fn latest_version(&self) -> SemVer {
        if let Some(latest_version) = &self.metadata.info.latest_version {
            latest_version.clone()
        } else {
            self.version.clone()
        }
    }

    pub fn license(&self) -> String {
        if let Some(license) = &self.metadata.info.license {
            license.clone()
        } else {
            "not found".to_string()
        }
    }

    pub fn rust_version_str(&self) -> String {
        if let Some(rv) = &self.metadata.info.rust_version {
            rv.to_string()
        } else {
            "unknown".to_string()
        }
    }

    pub fn documentation(&self) -> String {
        if let Some(documentation) = &self.metadata.info.documentation {
            documentation.clone()
        } else {
            "".to_string()
        }
    }

    pub fn homepage(&self) -> String {
        if let Some(hp) = &self.metadata.info.homepage {
            hp.clone()
        } else {
            "".to_string()
        }
    }

    pub fn repository(&self) -> String {
        if let Some(repository) = &self.metadata.info.repository {
            repository.clone()
        } else {
            "".to_string()
        }
    }

    pub fn is_latest(&self) -> bool {
        if let Some(latest_version) = &self.metadata.info.latest_version {
            latest_version == &self.version
        } else {
            true
        }
    }

    pub fn is_metadata_synced(&self) -> bool {
        self.metadata.info.synced
    }

    pub fn update_version(&mut self) -> Result<(), Error> {
        let name = &self.name.clone();
        let s = list_crates()?;
        let v = parse_ver(&s, name)?.1;
        self.version = v;

        Ok(())
    }

    pub async fn update(&self) -> Result<(), Error> {
        install_crate(&self.name).await?;
        Ok(())
    }

    pub async fn uninstall(&self) -> Result<(), Error> {
        uninstall_crate(&self.name).await
    }

    /// Retrieves information about the crate.
    /// Does not contain latest information from crates.io.
    fn parse(s: &str) -> IResult<&str, Krate> {
        let (s, name) = map(alphanumeric1_with_hyphen, String::from).parse(s)?;
        let (s, _) = multispace1(s)?;
        let (s, _) = char('v')(s)?;
        let (s, version) = SemVer::parse(s)?;
        let (s, _) = take_until("\n")(s)?;
        let (s, _) = newline(s)?;
        let (s, binaries) = separated_list1(newline, parse_binary).parse(s)?;

        let k = Krate {
            name,
            version,
            binaries,
            ..Default::default()
        };

        Ok((s, k))
    }
}

fn parse_ver<'a>(s: &'a str, n: &'a str) -> IResult<&'a str, SemVer> {
    let (s, _) = take_until(n)(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = tag(n)(s)?;
    let (s, _) = multispace1(s)?;
    let (s, _) = char('v')(s)?;
    let (s, v) = SemVer::parse(s)?;

    Ok((s, v))
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct KrateMetadata {
    name: String,
    info: KrateInfo,
}

impl KrateMetadata {
    /// Get the info of the given crate.
    pub fn get(name: &str) -> Result<Self, Error> {
        let s = search_crate(name)?;
        let info = KrateInfo::parse(&s)?.1;
        let ki = Self {
            name: name.to_string(),
            info,
        };

        Ok(ki)
    }
}

/// Contains latest information about the crate from crates.io.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct KrateInfo {
    description: Option<String>,
    tags: Option<Tags>,
    latest_version: Option<SemVer>,
    license: Option<String>,
    // Some crates have "unknown" as their Rust version.
    rust_version: Option<SemVer>,
    // NOTE: Let's keep the urls as strings for now as it's easier to parse and display.
    documentation: Option<String>,
    homepage: Option<String>,
    repository: Option<String>,
    crates_io: Option<String>,
    synced: bool,
}

impl KrateInfo {
    fn parse(s: &str) -> IResult<&str, Self> {
        let (s, _) = alphanumeric1_with_hyphen(s)?;
        let (s, _) = multispace0(s)?;
        let (s, tags) = opt(map(opt(Tags::parse), |t| t.unwrap_or_default())).parse(s)?;
        let (s, _) = multispace0(s)?;
        let (s, description) = opt(map(take_until("version"), String::from)).parse(s)?;
        let (s, _) = ws(tag("version:")).parse(s)?;
        let (s, latest_version) = opt(SemVer::parse).parse(s)?;
        let (s, _) = newline(s)?;
        let (s, _) = ws(tag("license:")).parse(s)?;
        let (s, license) = opt(map(take_until("\n"), String::from)).parse(s)?;
        let (s, _) = newline(s)?;
        let (s, _) = ws(tag("rust-version:")).parse(s)?;
        let (s, rust_version) = opt(SemVer::parse).parse(s)?;
        // When the Rust version is "unknown".
        let (s, _) = take_until("\n")(s)?;
        let (s, _) = opt(ws(tag("documentation:"))).parse(s)?;
        let (s, documentation) = opt(map(take_until("\n"), String::from)).parse(s)?;
        let (s, homepage) = opt(map(
            opt(preceded(ws(tag("homepage:")), take_until("\n"))),
            |d| d.unwrap_or("not available").to_string(),
        ))
        .parse(s)?;
        let (s, _) = multispace0(s)?;
        let (s, repository) = opt(map(
            opt(preceded(ws(tag("repository:")), take_until("\n"))),
            |d| d.unwrap_or("not available").to_string(),
        ))
        .parse(s)?;
        let (s, _) = multispace0(s)?;
        let (s, _) = ws(tag("crates.io:")).parse(s)?;
        let (s, crates_io) = opt(map(take_until("\n"), String::from)).parse(s)?;
        let synced = true;

        let description = if let Some(d) = description {
            Some(d.trim_end().to_string())
        } else {
            description
        };

        let k = Self {
            description,
            tags,
            latest_version,
            license,
            rust_version,
            documentation,
            homepage,
            repository,
            crates_io,
            synced,
        };

        Ok((s, k))
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Tags(Vec<String>);

impl Display for Tags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v: &Vec<String> = &self.0.iter().map(|s| format!("#{s}")).collect();
        let s = v.join(" ");
        write!(f, "{s}")
    }
}

impl Tags {
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

#[cfg(test)]
mod parser_tests {
    use super::{Krate, KrateInfo, Krates, Tags, parse_binary};
    use crate::{depot::parse_ver, parser::alphanumeric1_with_hyphen};
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
    fn parse_empty_krates() {
        let s = "";
        assert_eq!(Krates::parse(s).unwrap().1, Krates(vec![]))
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
    fn parse_krate_version() {
        let s = " fooo foooo fooooo cargo-thesaurust v0.1.2:";

        assert_eq!(
            parse_ver(s, "cargo-thesaurust").unwrap().1,
            SemVer::parse("0.1.2").unwrap().1
        )
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
                description: Some("A terminal-based dictionary app.".to_string()),
                tags: Some(Tags(vec![
                    "ratatui".to_string(),
                    "dictionary".to_string(),
                    "tui".to_string(),
                    "terminal".to_string(),
                    "thesaurus".to_string()
                ])),
                latest_version: SemVer::new("0.1.2"),
                license: Some("MIT".to_string()),
                rust_version: None,
                documentation: Some("https://docs.rs/cargo-thesaurust/0.1.2".to_string()),
                homepage: Some("https://moreenh.me/pages/projects/cargo-thesaurust".to_string()),
                repository: Some("https://github.com/quietpigeon/cargo-thesaurust".to_string()),
                crates_io: Some("https://crates.io/crates/cargo-thesaurust/0.1.2".to_string()),
                synced: true
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
                tags: Some(Tags(vec![])),
                description: Some("A terminal-based dictionary app.".to_string()),
                latest_version: SemVer::new("0.1.2"),
                license: Some("MIT".to_string()),
                rust_version: None,
                documentation: Some("https://docs.rs/cargo-thesaurust/0.1.2".to_string()),
                homepage: Some("https://moreenh.me/pages/projects/cargo-thesaurust".to_string()),
                repository: Some("https://github.com/quietpigeon/cargo-thesaurust".to_string()),
                crates_io: Some("https://crates.io/crates/cargo-thesaurust/0.1.2".to_string()),
                synced: true
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
