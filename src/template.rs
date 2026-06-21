use home::home_dir;

use crate::LATEST_CONFIG_VERSION;
use anyhow::{Context, Result};
use std::env;
use std::fs;
use std::path::PathBuf;

// Gets the pre-programmed default template
fn get_inbuilt_default_template() -> String {
	format!(
		"version=\"{version}\"

[scripts]
start = \"echo \\\"No start script yet!\\\"\"",
		version = LATEST_CONFIG_VERSION
	)
}

fn get_template_path() -> Option<PathBuf> {
	if let Ok(path) = env::var("BX_TEMPLATE") {
		return Some(PathBuf::from(path));
	}
	if let Ok(path) = env::var("BONNIE_TEMPLATE") {
		return Some(PathBuf::from(path));
	}
	if let Some(home) = home_dir() {
		let bx_template = home.join(".bx").join("template.toml");
		if bx_template.exists() {
			return Some(bx_template);
		}
		let bonnie_template = home.join(".bonnie").join("template.toml");
		if bonnie_template.exists() {
			return Some(bonnie_template);
		}
	}
	None
}

pub fn get_default_template() -> Result<String> {
	let path = get_template_path();
	if let Some(path) = path {
		fs::read_to_string(&path)
			.with_context(|| format!("Failed to get default template file at '{}'. Please make sure any path in 'BX_TEMPLATE' or 'BONNIE_TEMPLATE' definitely exists.", path.display()))
	} else {
		Ok(get_inbuilt_default_template())
	}
}
