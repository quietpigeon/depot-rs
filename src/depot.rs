use versions::SemVer;

use crate::{commands::list_crates, errors::Error, parser::Parsable};

#[derive(Debug, Default)]
pub struct DepotState {
    pub depot: Depot,
}

#[derive(Debug, Default)]
pub struct Depot {
    pub store: Krates,
    pub crate_count: i64,
}

impl DepotState {
    pub fn sync(&mut self) -> Result<(), Error> {
        self.depot = Depot::get()?;
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
}
