use std::{fs::FileType, path::PathBuf};

use serde::Deserialize;

const DEFAULT_TOML: &str = include_str!("../default_config.toml");

#[derive(Debug, Deserialize)]
struct File {
    config: Config,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub dest: PathBuf,
    pub src: Option<PathBuf>,
    #[serde(default = "default_dry_run")]
    pub dry_run: bool,
    pub image_types: Vec<String>,
}

fn default_dry_run() -> bool {
    true
}

impl Config { 
    pub fn default_from_toml() -> Self {
        let file: File = toml::from_str(DEFAULT_TOML).expect("default_config.toml is invalid");
        let mut cfg = file.config;
        cfg.dest = expand_tilde(cfg.dest);
        cfg.src = cfg.src.map(expand_tilde);
        cfg.image_types = cfg
            .image_types
            .into_iter()
            .map(|s| s.to_ascii_lowercase())
            .collect();
        cfg
    }

    pub fn allows_ext(&self, ext: &str) -> bool {
        self.image_types
            .iter()
            .any(|t| t.eq_ignore_ascii_case(ext))
    }
}

fn expand_tilde(path: PathBuf) -> PathBuf {
    let raw = path.to_string_lossy();
    if let Some(rest) = raw.strip_prefix("~/") && let Some(home) = std::env::var_os("HOME") { 
        return PathBuf::from(home).join(rest); 
    }
    path
}
