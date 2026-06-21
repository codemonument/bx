use lib::{cache, load_from_cache, Config, FinalConfig, LATEST_CONFIG_VERSION};
use std::env;
use tempfile::tempdir;

const CFG_STR: &str = r#"
env_files = ["src/.env"]
default_env.generic = ["sh", "-c", "{COMMAND}"]
default_env.targets.linux = ["bash", "-c", "{COMMAND}"]
[scripts]
basic.subcommands.test.cmd.generic = "exit 5"
basic.subcommands.test.cmd.targets.linux.exec = [
    "echo %SHORTGREETING %%",
    "echo %name && exit 1"
]
basic.subcommands.test.env_vars = ["SHORTGREETING"]
basic.subcommands.test.cmd.targets.linux.shell = ["sh", "-c", "{COMMAND}"]
basic.subcommands.nested.subcommands.test = "exit 2"
basic.subcommands.nested.subcommands.other = "exit 3"
basic.subcommands.nested.order = """
test {
    Any => other
}
"""
basic.args = ["name"]
basic.order = """
test {
    Any => nested {
        Any => test
    }
}
"""
"#;

#[cfg(test)]
fn get_cfg(version: &str) -> FinalConfig {
	let cfg_str = "version = \"".to_string() + version + "\"\n" + CFG_STR;
	Config::new(&cfg_str)
		.unwrap()
		.to_final(&mut Vec::new())
		.unwrap()
}

#[test]
fn cache_works() {
	let dir = tempdir().unwrap();
	let tmp_path = dir.path().join("cache.json").to_string_lossy().to_string();
	let cfg = get_cfg(LATEST_CONFIG_VERSION);
	let mut output = Vec::new();
	cache(&cfg, &mut output, Some(&tmp_path)).expect("cache should succeed");
	let cfg_extracted =
		load_from_cache(&mut output, Some(&tmp_path)).expect("load_from_cache should succeed");
	assert_eq!(cfg_extracted, cfg);
}

#[test]
fn loads_env_files() {
	let dir = tempdir().unwrap();
	let tmp_path = dir.path().join("cache.json").to_string_lossy().to_string();
	let cfg = get_cfg(LATEST_CONFIG_VERSION);
	env::remove_var("SHORTGREETING");
	let mut output = Vec::new();
	cache(&cfg, &mut output, Some(&tmp_path)).unwrap();
	load_from_cache(&mut output, Some(&tmp_path)).unwrap();
	assert_eq!(env::var("SHORTGREETING"), Ok("Hello".to_string()))
}

#[test]
fn returns_error_on_bad_version() {
	let dir = tempdir().unwrap();
	let tmp_path = dir.path().join("cache.json").to_string_lossy().to_string();
	let mut cfg = get_cfg(LATEST_CONFIG_VERSION);
	let mut output = Vec::new();
	cfg.version = "0.1.0".to_string();
	cache(&cfg, &mut output, Some(&tmp_path)).unwrap();
	let cfg_extracted = load_from_cache(&mut output, Some(&tmp_path));
	assert!(cfg_extracted.is_err());
}
