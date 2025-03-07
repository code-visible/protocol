use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Pkg {
    pub id: String,
    pub name: String,
    pub path: String,
    pub imports: Vec<String>,
    pub exports: Vec<String>,
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
    pub exports: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Dep {
    pub id: String,
    pub name: String,
    pub typ: String,
    pub rf: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Repo {
    pub name: String,
    pub lang: String,
    pub parser: String,
    pub timestamp: String,
    pub pkgs: Vec<Pkg>,
    pub files: Vec<File>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Callable {
    pub id: String,
    pub name: String,
    pub pos: String,
    pub file: String,
    pub comment: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Call {
    pub id: String,
    pub name: String,
    pub pos: String,
    pub file: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Abstract {
    pub id: String,
    pub name: String,
    pub pos: String,
    pub file: String,
    pub comment: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Reference {
    pub id: String,
    pub name: String,
    pub pos: String,
    pub file: String,
}
