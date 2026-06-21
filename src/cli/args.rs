use clap::{Parser, Subcommand};

/// Simple, cross-platform, and fast command aliases with superpowers.
#[derive(Parser, Debug)]
#[command(name = "bx", version, about, long_about = None)]
#[command(args_conflicts_with_subcommands = true)]
pub struct Cli {
	/// Enable debug mode (show commands without running them)
	#[arg(short = 'd', long = "debug", global = true)]
	pub debug: bool,

	#[command(subcommand)]
	pub command: Option<Commands>,

	/// Command name and arguments to run from config
	#[arg(trailing_var_arg = true, allow_hyphen_values = true)]
	pub run_args: Vec<String>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
	/// Print version information
	#[command(name = "-v", visible_alias = "--version")]
	Version,

	/// Initialize a new configuration file
	#[command(name = "-i", visible_alias = "--init")]
	Init {
		/// Use a template file
		#[arg(short = 't', long = "template")]
		template: Option<String>,
	},

	/// Cache the configuration file for faster loading
	#[command(name = "-c", visible_alias = "--cache")]
	Cache,

	/// Show help information
	#[command(name = "-h", visible_alias = "--help")]
	Help,
}
