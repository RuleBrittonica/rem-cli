use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Programs {
    pub aeneas: String,
    pub charon: String,
}

#[derive(Debug, Deserialize)]
pub struct Files {
    pub primitives: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub programs: Programs,
    pub files: Files,
}