// This file contains logic to get the actual configuration itself

use std::env;
use std::fs;
use std::path::Path;

// Default config file names (bx.toml takes priority over bonnie.toml)
pub const DEFAULT_BX_CFG_PATH: &str = "./bx.toml";
pub const DEFAULT_BONNIE_CFG_PATH: &str = "./bonnie.toml";

// Extracts the config from the TOML file at the given path
pub fn get_cfg() -> Result<String, String> {
    // Get the path of the config
    let path = get_cfg_path()?;
    let cfg_string = fs::read_to_string(&path);
    match cfg_string {
		Ok(cfg_string) => Ok(cfg_string),
		Err(_) => Err(format!("Error reading configuration file at '{}', make sure the file is present in this directory and you have the permissions to read it.", path))
	}
}

// Gets the path to the config file based on given environment variables
// Priority: BX_CONF > BONNIE_CONF > ./bx.toml (if exists) > ./bonnie.toml
// This will return an error if an environment variable is set but contains invalid characters
pub fn get_cfg_path() -> Result<String, String> {
    // First check BX_CONF environment variable
    match env::var("BX_CONF") {
        Ok(path) => return Ok(path),
        Err(env::VarError::NotUnicode(_)) => return Err(String::from("The path to your configuration file given in the 'BX_CONF' environment variable contained invalid characters. Please make sure it only contains valid Unicode.")),
        Err(env::VarError::NotPresent) => {} // Continue to check BONNIE_CONF
    }

    // Then check BONNIE_CONF environment variable
    match env::var("BONNIE_CONF") {
        Ok(path) => return Ok(path),
        Err(env::VarError::NotUnicode(_)) => return Err(String::from("The path to your configuration file given in the 'BONNIE_CONF' environment variable contained invalid characters. Please make sure it only contains valid Unicode.")),
        Err(env::VarError::NotPresent) => {} // Continue to check default paths
    }

    // No env var set, check for bx.toml first, then bonnie.toml
    if Path::new(DEFAULT_BX_CFG_PATH).exists() {
        Ok(DEFAULT_BX_CFG_PATH.to_string())
    } else {
        Ok(DEFAULT_BONNIE_CFG_PATH.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bx_conf_env_takes_priority() {
        let custom_path = "/custom/path/config.toml";
        env::set_var("BX_CONF", custom_path);
        env::remove_var("BONNIE_CONF");

        let result = get_cfg_path().unwrap();
        assert_eq!(result, custom_path);

        env::remove_var("BX_CONF");
    }

    #[test]
    fn bonnie_conf_env_used_when_bx_conf_not_set() {
        env::remove_var("BX_CONF");
        let custom_path = "/custom/bonnie/config.toml";
        env::set_var("BONNIE_CONF", custom_path);

        let result = get_cfg_path().unwrap();
        assert_eq!(result, custom_path);

        env::remove_var("BONNIE_CONF");
    }

    #[test]
    fn bx_conf_takes_priority_over_bonnie_conf() {
        let bx_path = "/bx/config.toml";
        let bonnie_path = "/bonnie/config.toml";
        env::set_var("BX_CONF", bx_path);
        env::set_var("BONNIE_CONF", bonnie_path);

        let result = get_cfg_path().unwrap();
        assert_eq!(result, bx_path);

        env::remove_var("BX_CONF");
        env::remove_var("BONNIE_CONF");
    }

    #[test]
    fn defaults_to_bonnie_toml_when_no_env_vars() {
        env::remove_var("BX_CONF");
        env::remove_var("BONNIE_CONF");

        let result = get_cfg_path().unwrap();
        assert!(
            result == DEFAULT_BX_CFG_PATH || result == DEFAULT_BONNIE_CFG_PATH,
            "Expected either {} or {}, got {}",
            DEFAULT_BX_CFG_PATH,
            DEFAULT_BONNIE_CFG_PATH,
            result
        );
    }
}
