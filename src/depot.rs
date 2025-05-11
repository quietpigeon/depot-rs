use versions::SemVer;

#[derive(Debug)]
pub struct Depot {
    pub store: Krates,
    pub crate_count: i64,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Krates(pub Vec<Krate>);

#[derive(Debug, PartialEq, Eq)]
pub struct Krate {
    pub name: String,
    pub version: SemVer,
    pub binaries: Vec<String>,
}
