use std::path::{PathBuf};
use serde::{Deserialize};

const DEFAULT: &str = include_str!("../default_config.toml");

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct Config {
    dest: Option<PathBuf>,
    src: Option<PathBuf>,
    dry_run: Option<bool>,
    image_types: Option<Vec<String>>,
}

impl Default for Config {
    fn default() -> Self {
        let path = PathBuf::from(DEFAULT);
        let raw = std::fs::read_to_string(&path).expect("failed to read config file"); 
        let cfg: Config = toml::from_str(&raw).expect("Failed to parse config file");        

        Self {
            dest: Some(cfg.dest.unwrap()),
            src: None,
            dry_run: Some(cfg.dry_run.unwrap_or(true)),
            image_types: Some(cfg.image_types.unwrap()),
        }
    }
}



