use std::{collections::HashMap, fs::read_to_string, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::config::Config;

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    pub projector: HashMap<PathBuf, HashMap<String, String>>,
}

pub struct Projector {
    config: Config,
    data: Data,
}

fn default_data() -> Data {
    return Data {
        projector: HashMap::new(),
    };
}

impl Projector {
    fn from_config(config: Config) -> Self {
        // if std::fs::try_exists(config.config).is_ok() {
        // }
        if std::fs::metadata(config.config).is_ok() {
            let contents = std::fs::read_to_string(config.config);
            let contents = contents.unwrap_or(String::from("{\"projector\":{}}"));
            data = serde_json::from_str(&contents);
            let data = data.unwrap_or(default_data());

            return Projector { config, data };
        }

        return Projector {
            config,
            data: default_data(),
        };
    }
}
