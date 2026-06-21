use crate::template::get_default_template;

use anyhow::{bail, Context, Result};
use std::fs;

pub fn init(template: Option<String>, cfg_path: &str) -> Result<()> {
	if fs::metadata(cfg_path).is_ok() {
		bail!("A bx configuration file already exists in this directory. If you want to create a new one, please delete the old one first.");
	}

	// Check if a template has been given
	if let Some(template_path) = template {
		if fs::metadata(&template_path).is_ok() {
			// We have a valid template file
			let contents = fs::read_to_string(&template_path)
				.with_context(|| format!("An error occurred while attempting to read the given template file '{}'. Please make sure the file exists and you have the permissions necessary to read from it.", &template_path))?;
			fs::write(cfg_path, contents)
				.with_context(|| format!("Error creating new {}, make sure you have the permissions to write to this directory.", cfg_path))?;
		} else {
			// We have a template file that doesn't exist
			bail!("The given template file at '{}' does not exist or can't be read. Please make sure the file exists and you have the permissions necessary to read from it.", &template_path);
		}
	} else {
		// Try to get the default template file from `~/.bonnie/template.toml`
		// If it's not available, we'll use a pre-programmed default
		let template = get_default_template()?;
		fs::write(cfg_path, template)
			.with_context(|| format!("Error creating new {}, make sure you have the permissions to write to this directory.", cfg_path))?;
	}

	Ok(())
}
