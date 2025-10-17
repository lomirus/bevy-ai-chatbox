use std::{fs, io::ErrorKind};

use bevy::prelude::*;
use ron::ser::PrettyConfig;
use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize, Resource)]
pub(crate) struct Config {
    pub(crate) api_key: String,
}

impl Config {
    pub(crate) fn get_or_init() -> Config {
        match fs::read("config.ron") {
            Ok(file) => {
                let config_str = String::from_utf8(file).unwrap();
                let config: Config = ron::from_str(&config_str).unwrap();
                config
            }
            Err(err) => match err.kind() {
                ErrorKind::NotFound => {
                    let config: Config = Config::default();
                    let config_str =
                        ron::ser::to_string_pretty(&config, PrettyConfig::default()).unwrap();
                    fs::write("config.ron", config_str).unwrap();
                    config
                }
                _ => panic!("{err}"),
            },
        }
    }
}
