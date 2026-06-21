// This file contains the schema that all bx configuration files are deserialised with
// They will then be parsed into the schema defined in `schema.rs` using the logic in the methods on this schema
// The use of `#[serde(untagged)]` on all `enum`s simply ensures that Serde doesn't require them to be labelled as to their variant
// This raw schema will also derive the `Arbitrary` trait for fuzzing when that feature is enabled

use crate::bones::parse_directive_str;
use crate::default_shells::get_default_shells;
use crate::schema;
use crate::version::{
	is_config_version_supported, LATEST_CONFIG_VERSION, SUPPORTED_CONFIG_VERSIONS,
};
use anyhow::{bail, Result};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
	version: String,                // This will be used to confirm compatibility
	env_files: Option<Vec<String>>, // Files specified here have their environment variables loaded into bx
	default_shell: Option<DefaultShell>,
	scripts: Scripts,
}
impl Config {
	pub fn new(cfg_string: &str) -> Result<Self> {
		let cfg: Result<Self, toml::de::Error> = toml::from_str(cfg_string);
		let cfg = match cfg {
            Ok(cfg) => cfg,
            Err(err) if err.to_string().starts_with("missing field `version`") => bail!("Your configuration file appears to be missing a 'version' key. Please add `version = \"{}\"` to the top of your configuration file.", LATEST_CONFIG_VERSION),
            Err(err) => bail!("Invalid configuration file. Error: '{}'", err)
        };

		Ok(cfg)
	}
	pub fn to_final(&self, output: &mut impl std::io::Write) -> Result<schema::Config> {
		Self::check_config_version(&self.version, output)?;
		Self::load_env_files(self.env_files.clone())?;
		let cfg = self.parse()?;
		Ok(cfg)
	}

	pub fn check_config_version(
		cfg_version_str: &str,
		_output: &mut impl std::io::Write,
	) -> Result<()> {
		if is_config_version_supported(cfg_version_str) {
			Ok(())
		} else {
			bail!(
				"Unsupported config file version '{}'. Supported versions: {}. \
				Please update your configuration file's version field.",
				cfg_version_str,
				SUPPORTED_CONFIG_VERSIONS.join(", ")
			)
		}
	}

	#[deprecated(note = "Use check_config_version instead")]
	pub fn parse_version_against_current(
		cfg_version_str: &str,
		_bonnie_version_str: &str,
		output: &mut impl std::io::Write,
	) -> Result<()> {
		Self::check_config_version(cfg_version_str, output)
	}
	// Loads the environment variable files requested in the config
	// This is generic because it's called in caching as well
	pub fn load_env_files(env_files: Option<Vec<String>>) -> Result<()> {
		let env_files = env_files.unwrap_or_default();
		// Parse each of the requested environment variable files
		for env_file in env_files.iter() {
			// Load the file
			// This will be loaded for the bx program, which allows us to interpolate them into commands
			let res = dotenv::from_filename(env_file);
			if res.is_err() {
				bail!("Requested environment variable file '{}' could not be loaded. Either the file doesn't exist, bx doesn't have the permissions necessary to access it, or something inside it can't be processed.", &env_file);
			}
		}

		Ok(())
	}
	// Parses the rest of the config into the final form, consuming `self`
	// A very large portion of bx's logic lives here or is called here (spec transformation)
	fn parse(&self) -> Result<schema::Config> {
		// Parse the default shell
		let default_shell = match &self.default_shell {
			// If we're just given a shell string, use it as the generic shell
			Some(DefaultShell::Simple(generic)) => schema::DefaultShell {
				generic: generic.parse(),
				targets: HashMap::new(),
			},
			// If we have all the information we need, just transform it
			Some(DefaultShell::Complex { generic, targets }) => schema::DefaultShell {
				generic: generic.parse(),
				targets: match targets {
					Some(raw_targets) => {
						// This is just transformation logic
						let mut targets = HashMap::new();
						for (target_name, shell) in raw_targets.iter() {
							targets.insert(target_name.to_string(), shell.parse());
						}
						targets
					}
					None => HashMap::new(), // We'll just use the generic if we don't have anything else
				},
			},
			// If no default shell is provided, we'll use the default paradigm (see `default_shells.rs`)
			None => get_default_shells(),
		};
		// Parse the scripts (brace yourself!)
		// We do this inside a function because it's recursive
		// Unfortunately we can't define methods on type aliases, so this goes here
		// This involves validation logic to ensure invalid property combinations aren't specified, so we need to know whether or not `order` is specified if this is parsing subcommands
		fn parse_scripts(raw_scripts: &Scripts, is_order_defined: bool) -> Result<schema::Scripts> {
			let mut scripts: schema::Scripts = HashMap::new();
			for (script_name, raw_command) in raw_scripts.iter() {
				let command = match raw_command {
                    Command::Simple(raw_command_wrapper) => schema::Command {
                        args: Vec::new(),
                        env_vars: Vec::new(),
                        subcommands: None,
                        order: None,
                        cmd: Some(raw_command_wrapper.parse()), // In the simple form, a command must be given (no subcommands can be specified)
                        description: None
                    },
                    Command::Complex {
                        args,
                        env_vars,
                        subcommands,
                        order,
                        cmd,
                        desc
                    } => schema::Command {
                        // If `order` is defined at the level above, we can't interpolate environment variables from here (has to be done at the level `order` was specified)
                        args: match is_order_defined {
                            // Unordered subcommands can't take arguments in any case of upper-level `order` definition
                            _ if subcommands.is_some() && order.is_none() && args.is_some() => bail!("Error in parsing bx configuration file: if `subcommands` is specified without `order`, `args` cannot be specified. This error occurred in in the '{}' script/subscript.", script_name),
                            // If it was and `args` is specified, return an error
                            true if args.is_some() => bail!("Error in parsing bx configuration file: if `order` is specified, subscripts cannot specify `args`, as no environment variables can be provided to them. Environment variables to be interpolated in ordered subcommands must be set at the top-level. This error occurred in the '{}' script/subscript.", script_name),
                            // If it was but args` isn't specified, it doesn't matter and we just give an empty vector instead
                            true => Vec::new(),
                            // If it wasn't, no validation needed
                            false => args.as_ref().unwrap_or(&Vec::new()).to_vec()
                        },
                        // This doesn't need any transformation, just a simple alternative if it's `None`
                        env_vars: env_vars.as_ref().unwrap_or(&Vec::new()).to_vec(),
                        // The subcommands are parsed recursively as scripts using this very function
                        // We parse through whether or not `order` is defined (has validation implications)
                        subcommands: match subcommands {
                            // We can't use `.map()` for this because we need support for `?`
                            Some(subcommands) => Some(
                                parse_scripts(subcommands, order.is_some())?
                            ),
                            None => None
                        },
                        // If `order` is defined at the level above and `subcommands` is defined here, `order` must be defined here too
                        order: match is_order_defined {
                            true if subcommands.is_some() => match order {
                                // If it was required and was given, no problem
                                Some(order) => Some(parse_directive_str(order)?),
                                // If it was required but not given, return an error
                                None => bail!("Error in parsing bx configuration file: if `order` is specified, all further nested subsubcommands must also specify `order`. This occurred in the '{}' script/subscript.", script_name)
                            }
                            // If it wasn't required, no validation needed
                            true | false => match order {
                                Some(order) => Some(parse_directive_str(order)?),
                                None => None
                            }
                        },
                        // If subcommands were specified, this is optional, otherwise we return an error
                        cmd: match cmd {
                            // It was given, but there are also ordered subcommands here, so execution will be ambiguous, return an error
                            Some(_) if order.is_some() => bail!("Error in parsing bx configuration file: both `cmd` and `order` were specified. This would lead to problems of ambiguous execution, so commands can have either the top-level `cmd` property or ordered subcommands, the two are mutually exclusive. This error occurred in in the '{}' script/subscript.", script_name),
                            // It's optional
                            _ if subcommands.is_some() => cmd.as_ref().map(|cmd| cmd.parse()),
                            // It's mandatory and given
                            Some(cmd) => Some(cmd.parse()),
                            // It's mandatory and not given
                            None => bail!("Error in parsing bx configuration file: if `subcommands` is not specified, `cmd` is mandatory. This error occurred in in the '{}' script/subscript.", script_name)
                        },
                        description: desc.clone()
                    },
                };
				scripts.insert(script_name.to_string(), command);
			}

			Ok(scripts)
		}

		let scripts = parse_scripts(&self.scripts, false)?;

		Ok(schema::Config {
			default_shell,
			scripts,
			// Copy these last two in case the final config is cached and needs to be revalidated on load
			env_files: match &self.env_files {
				Some(env_files) => env_files.to_vec(),
				None => Vec::new(),
			},
			version: self.version.clone(),
		})
	}
}
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum DefaultShell {
	Simple(Shell), // Just a generic shell
	Complex {
		generic: Shell, // A generic shell must be given
		targets: Option<HashMap<String, Shell>>,
	},
}
// A vector of the executable followed by raw arguments thereto, the location for command interpolation is specified with '{COMMAND}'
// A custom delimiter can also be specified (the default is ` && `), this should include spaces if necessary
// Note that the default for PowerShell uses `;` instead
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum Shell {
	Simple(Vec<String>),
	WithDelimiter {
		parts: Vec<String>,
		delimiter: String,
	},
}
impl Shell {
	fn parse(&self) -> schema::Shell {
		match self {
			Shell::Simple(parts) => schema::Shell {
				parts: parts.to_vec(),
				// The default delimiter is ` && ` in all cases (supported everywhere except Windows PowerShell)
				delimiter: " && ".to_string(),
			},
			Shell::WithDelimiter { parts, delimiter } => schema::Shell {
				parts: parts.to_vec(),
				// The default delimiter is `&&` in all cases (supported everywhere except Windows PowerShell)
				delimiter: delimiter.to_string(),
			},
		}
	}
}
type TargetString = String; // A target like `linux` or `x86_64-unknown-linux-musl` (see `rustup` targets)
type Scripts = HashMap<String, Command>;

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum Command {
	Simple(CommandWrapper), // Might be just a string command to run on the default generic shell
	Complex {
		args: Option<Vec<String>>,
		env_vars: Option<Vec<String>>,
		subcommands: Option<Scripts>, // Subcommands are fully-fledged commands (mostly)
		order: Option<OrderString>, // If this is specified, subcomands must not specify the `args` property, it may be specified at the top-level of this script as a sibling of `order`
		cmd: Option<CommandWrapper>, // This is optional if subcommands are specified
		desc: Option<String>, // This will be rendered in the config's help page ('description' is overly verbose)
	},
}
type OrderString = String; // A string of as yet undefined syntax that defines the progression between subcommands
						   // This wraps the complexities of having different shell logic for each command in a multi-stage context
						   // subcommands are specified above this level (see `Command::Complex`)
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum CommandWrapper {
	Universal(CommandCore), // Just a given command
	Specific {
		generic: CommandCore,
		targets: Option<HashMap<TargetString, CommandCore>>,
	},
}
impl CommandWrapper {
	// Parses `self` into its final form (`schema::CommandWrapper`)
	fn parse(&self) -> schema::CommandWrapper {
		match self {
			// If it's universal to all targets, just provide a generic
			CommandWrapper::Universal(raw_command_core) => schema::CommandWrapper {
				generic: raw_command_core.parse(),
				targets: HashMap::new(),
			},
			// If no targets were given in specific form, the expansion is basically the same as if it were universal
			CommandWrapper::Specific {
				generic,
				targets: None,
			} => schema::CommandWrapper {
				generic: generic.parse(),
				targets: HashMap::new(),
			},
			CommandWrapper::Specific {
				generic,
				targets: Some(targets),
			} => {
				let parsed_generic = generic.parse();
				let mut parsed_targets: HashMap<schema::TargetString, schema::CommandCore> =
					HashMap::new();
				for (target_name, raw_command_core) in targets.iter() {
					parsed_targets.insert(target_name.to_string(), raw_command_core.parse());
				}
				schema::CommandWrapper {
					generic: parsed_generic,
					targets: parsed_targets,
				}
			}
		}
	}
}
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum CommandCore {
	Simple(CommandBox), // No shell configuration
	WithShell {
		exec: CommandBox, // We can't call this `cmd` because otherwise we'd have a collision with the higher-level `cmd`, which leads to misinterpretation
		shell: Option<Shell>,
	},
}
impl CommandCore {
	// Parses `self` into its final form (`schema::CommandCore`)
	fn parse(&self) -> schema::CommandCore {
		match self {
			CommandCore::Simple(exec) => schema::CommandCore {
				exec: exec.parse(),
				shell: None,
			},
			CommandCore::WithShell {
				exec,
				shell: Some(shell),
			} => schema::CommandCore {
				exec: exec.parse(),
				shell: Some(shell.parse()),
			},
			// If no shell was given in the complex form, the expansion is the same as the simple form
			CommandCore::WithShell { exec, shell: None } => schema::CommandCore {
				exec: exec.parse(),
				shell: None,
			},
		}
	}
}
// This represents the possibility of a vector or string at the lowest level
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum CommandBox {
	Simple(String),
	MultiStage(Vec<String>),
}
impl CommandBox {
	// Parses `self` into its final form (`Vec<schema::CommandWrapper>`)
	fn parse(&self) -> Vec<String> {
		match self {
			// In fully parsed form, all command wrappers are inside vectors for simplicity
			CommandBox::Simple(cmd_str) => vec![cmd_str.to_string()],
			CommandBox::MultiStage(cmd_strs) => {
				cmd_strs.iter().map(|cmd_str| cmd_str.to_string()).collect()
			}
		}
	}
}
