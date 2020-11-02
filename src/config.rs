use rocket::figment::{Figment, providers::{Format, Toml, Serialized, Env}};
use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
pub struct Config {}

impl Default for Config {
    fn default() -> Self {
        Config {}
    }
}

impl Config {
    pub fn figment() -> Figment {
        rocket::Config::figment()
            .merge(Serialized::defaults(Config::default()))
            .merge(Toml::file(Env::var_or("ARA_CONFIG", "Ara.toml")).nested())
            .merge(Env::prefixed("ARA_").ignore(&["PROFILE"]).global())
    }
}
