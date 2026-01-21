// This file contains logic to get the actual configuration itself

use std::env;
use std::fs;
use std::path::Path;

pub const DEFAULT_BX_CFG_PATH: &str = "./bx.toml";
pub const DEFAULT_BONNIE_CFG_PATH: &str = "./bonnie.toml";

pub fn get_cfg() -> Result<String, String> {
    let path = resolve_cfg_path(
        env::var("BX_CONF").ok().as_deref(),
        env::var("BONNIE_CONF").ok().as_deref(),
        Path::new(DEFAULT_BX_CFG_PATH).exists(),
    );
    let cfg_string = fs::read_to_string(&path);
    match cfg_string {
		Ok(cfg_string) => Ok(cfg_string),
		Err(_) => Err(format!("Error reading configuration file at '{}', make sure the file is present in this directory and you have the permissions to read it.", path))
	}
}

pub fn resolve_cfg_path(
    bx_conf: Option<&str>,
    bonnie_conf: Option<&str>,
    bx_toml_exists: bool,
) -> String {
    if let Some(path) = bx_conf {
        return path.to_string();
    }
    if let Some(path) = bonnie_conf {
        return path.to_string();
    }
    if bx_toml_exists {
        DEFAULT_BX_CFG_PATH.to_string()
    } else {
        DEFAULT_BONNIE_CFG_PATH.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bx_conf_takes_priority() {
        let result = resolve_cfg_path(Some("/bx/path"), Some("/bonnie/path"), true);
        assert_eq!(result, "/bx/path");
    }

    #[test]
    fn bonnie_conf_used_when_bx_conf_not_set() {
        let result = resolve_cfg_path(None, Some("/bonnie/path"), true);
        assert_eq!(result, "/bonnie/path");
    }

    #[test]
    fn bx_toml_used_when_no_env_vars_and_exists() {
        let result = resolve_cfg_path(None, None, true);
        assert_eq!(result, DEFAULT_BX_CFG_PATH);
    }

    #[test]
    fn bonnie_toml_used_when_no_env_vars_and_bx_toml_missing() {
        let result = resolve_cfg_path(None, None, false);
        assert_eq!(result, DEFAULT_BONNIE_CFG_PATH);
    }
}
