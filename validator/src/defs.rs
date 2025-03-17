use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Pkg {
    pub id: String,
    pub name: String,
    pub path: String,
    pub imports: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct File {
    pub id: String,
    pub name: String,
    pub path: String,
    pub pkg: String,
    pub callables: Vec<Callable>,
    pub calls: Vec<Call>,
    pub abstracts: Vec<Abstract>,
    pub refs: Vec<Reference>,
    pub imports: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Dep {
    pub id: String,
    pub name: String,
    pub typ: String,
    pub refer: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Repo {
    pub name: String,
    pub lang: String,
    pub parser: String,
    pub timestamp: String,
    pub repository: String,
    pub version: String,
    pub pkgs: Vec<Pkg>,
    pub files: Vec<File>,
    pub fns: Vec<Callable>,
    pub calls: Vec<Call>,
    pub absts: Vec<Abstract>,
    pub refs: Vec<Reference>,
    pub deps: Vec<Dep>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Callable {
    pub id: String,
    pub name: String,
    pub pos: String,
    pub file: String,
    pub signature: String,
    pub abst: String,
    pub parameters: Vec<String>,
    pub results: Vec<String>,
    pub comment: String,
    pub method: bool,
    pub private: bool,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Call {
    pub id: String,
    pub signature: String,
    pub pos: String,
    pub caller: String,
    pub callee: String,
    pub typ: String,
    pub file: String,
    pub dep: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Abstract {
    pub id: String,
    pub name: String,
    pub pos: String,
    pub file: String,
    pub comment: String,
    pub fields: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Reference {
    pub id: String,
    pub name: String,
    pub pos: String,
    pub file: String,
}
