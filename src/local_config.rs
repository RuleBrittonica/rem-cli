//! Local configuration file. Allows for the deserializtion of Config.toml

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Programs {
    pub aeneas: String,
    pub charon: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings{
    pub programs: Programs,
}