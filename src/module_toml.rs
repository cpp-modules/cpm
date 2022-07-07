use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Module {
    pub project: Project,
    pub dependencies: Deps,
    pub flags: Flags,
    pub sources: Sources,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub version: String,
    pub module_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sources {
    pub source: Vec<String>,
    pub header: Vec<String>,
}

pub type Deps = BTreeMap<String, String>;
pub type Flags = BTreeMap<String, String>;
