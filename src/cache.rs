use crate::{raw_schema, schema};
use anyhow::{bail, Context, Result};
use std::env;
use std::fs;

pub const DEFAULT_BX_CACHE_PATH: &str = "./.bx.cache.json";

fn get_cache_path() -> Result<String> {
	match env::var("BX_CACHE") {
		Ok(path) => return Ok(path),
		Err(env::VarError::NotUnicode(_)) => {
			bail!("The cache file path in 'BX_CACHE' environment variable contained invalid characters. Please make sure it only contains valid Unicode.")
		}
		Err(env::VarError::NotPresent) => {}
	}
	match env::var("BONNIE_CACHE") {
		Ok(path) => Ok(path),
		Err(env::VarError::NotUnicode(_)) => {
			bail!("The cache file path in 'BONNIE_CACHE' environment variable contained invalid characters. Please make sure it only contains valid Unicode.")
		}
		Err(env::VarError::NotPresent) => Ok(DEFAULT_BX_CACHE_PATH.to_string()),
	}
}

// Serializes the given parsed configuration into a JSON string and write it to disk to speed up future execution
// This takes around 100ms on an old i7 for the testing file
// We extract the path for testing (which needs to use a temporary file)
pub fn cache(
	cfg: &schema::Config,
	output: &mut impl std::io::Write,
	raw_cache_path: Option<&str>,
) -> Result<()> {
	let cache_path = match raw_cache_path {
		Some(cache_path) => cache_path.to_string(),
		None => get_cache_path()?,
	};
	let cache_str = serde_json::to_string(cfg).context(
		"The following error occurred while attempting to cache your parsed bx configuration",
	)?;
	fs::write(&cache_path, cache_str)
		.with_context(|| format!("The following error occurred while attempting to write your cached bx configuration to '{}'", &cache_path))?;

	writeln!(
        output,
        "Your bx configuration has been successfully cached to '{}'! This will be used to speed up future execution. Please note that this cache will NOT be updated until you explicitly run `bx -c` again.",
        cache_path
    ).expect("Failed to write caching message.");
	Ok(())
}

pub fn cache_exists() -> Result<bool> {
	let exists = fs::metadata(get_cache_path()?).is_ok();
	Ok(exists)
}

// This does NOT attempt to check if the cache is out of date for performance
// The user must manually recache
pub fn load_from_cache(
	output: &mut impl std::io::Write,
	raw_cache_path: Option<&str>,
) -> Result<schema::Config> {
	let cache_path = match raw_cache_path {
		Some(cache_path) => cache_path.to_string(),
		None => get_cache_path()?,
	};
	let cfg_str = fs::read_to_string(&cache_path)
		.with_context(|| format!("The following error occurred while attempting to read your cached bx configuration at '{}'", &cache_path))?;

	let cfg = serde_json::from_str::<schema::Config>(&cfg_str)
		.with_context(|| format!("The following error occurred while attempting to parse your cached bx configuration at '{}'. If this persists, you can recache with `bx -c`.", &cache_path))?;
	raw_schema::Config::check_config_version(&cfg.version, output)?;
	// Load the environment variable files
	raw_schema::Config::load_env_files(Some(cfg.env_files.clone()))?;

	Ok(cfg)
}
