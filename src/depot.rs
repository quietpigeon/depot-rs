use crate::{
    commands::{list_crates, search_crate},
    errors::Error,
    parser::Parsable,
};
use ratatui::widgets::ListState;
use std::fmt::{self, Display};
use versions::SemVer;

#[derive(Debug)]
pub struct DepotState {
    pub depot: Depot,
    pub list_state: ListState,
}

#[derive(Debug, Default)]
pub struct Depot {
    pub store: Krates,
    pub crate_count: i64,
}

impl Default for DepotState {
    fn default() -> Self {
        let depot = Depot::get().expect("failed to initialize `DepotState`");
        let list_state = ListState::default();
        Self { depot, list_state }
    }
}

impl DepotState {
    pub fn sync_krate(&mut self, name: &str) -> Result<(), Error> {
        if let Some(idx) = self.depot.store.0.iter().position(|k| k.name == name) {
            self.depot.store.0[idx].info = KrateInfo::get(name)?;
        } else {
            return Err(Error::KrateNotFound);
        }

        Ok(())
    }
}

impl Depot {
    pub fn get() -> Result<Self, Error> {
        let output = list_crates()?;
        let store = Krates::parse(&output)?.1;
        let crate_count = store.0.len() as i64;

        Ok(Self { store, crate_count })
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Krates(pub Vec<Krate>);

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Krate {
    pub name: String,
    pub version: SemVer,
    pub binaries: Vec<String>,
    pub info: KrateInfo,
}

impl KrateInfo {
    fn get(name: &str) -> Result<Self, Error> {
        let s = search_crate(name)?;
        let info = KrateInfo::parse(&s)?.1;

        Ok(info)
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct KrateInfo {
    pub description: String,
    pub tags: Tags,
    pub latest_version: SemVer,
    pub license: String,
    pub rust_version: Option<SemVer>,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Tags(pub Vec<String>);

impl Display for Tags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let v: &Vec<String> = &self.0.iter().map(|s| format!("#{s}")).collect();
        let s = v.join(" ");
        write!(f, "{s}")
    }
}
