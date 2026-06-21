use clap::Parser;
use lib::{
	anyhow::Result,
	cache, cache_exists,
	cli::{Cli, Commands},
	get_cfg, help, init, load_from_cache, resolve_init_cfg_path, Config, CLI_VERSION,
};
use std::env;
use std::io::Write;

fn main() {
	let exit_code = real_main();
	std::process::exit(exit_code)
}

fn real_main() -> i32 {
	let res = core();
	match res {
		Ok(exit_code) => exit_code,
		Err(err) => {
			eprintln!("Error: {:#}", err);
			1
		}
	}
}

fn core() -> Result<i32> {
	let stdout = &mut std::io::stdout();
	let cli = Cli::parse();

	match cli.command {
		Some(Commands::Version) => {
			writeln!(
				stdout,
				"You are currently running bx v{}! You can see the latest release at https://github.com/codemonument/bx/releases.",
				CLI_VERSION
			).expect("Failed to write version.");
			return Ok(0);
		}
		Some(Commands::Init { template }) => {
			let init_path = resolve_init_cfg_path(
				env::var("BX_CONF").ok().as_deref(),
				env::var("BONNIE_CONF").ok().as_deref(),
			);
			init(template, &init_path)?;
			println!(
				"A new bx configuration file has been initialized at {}!",
				&init_path
			);
			return Ok(0);
		}
		Some(Commands::Help) => {
			help(stdout);
			return Ok(0);
		}
		Some(Commands::Cache) => {
			let cfg_str = get_cfg()?;
			let cfg = Config::new(&cfg_str)?.to_final(stdout)?;
			cache(&cfg, stdout, None)?;
			return Ok(0);
		}
		None => {}
	}

	let prog_args = cli.run_args;
	let verbose = cli.debug;

	let mut document = false;
	if !prog_args.is_empty() && prog_args[0] == "help" {
		document = true;
	}

	let cfg = if cache_exists()? {
		load_from_cache(stdout, None)?
	} else {
		let cfg_str = get_cfg()?;
		Config::new(&cfg_str)?.to_final(stdout)?
	};

	if document {
		let msg = cfg.document(prog_args.get(1).cloned())?;
		writeln!(stdout, "{}", msg).expect("Failed to write configuration help.");
		return Ok(0);
	}

	let (command_to_run, command_name, relevant_args) = cfg.get_command_for_args(&prog_args)?;
	let bone = command_to_run.prepare(&command_name, &relevant_args, &cfg.default_shell)?;
	let exit_code = bone.run(&command_name, verbose, stdout)?;

	Ok(exit_code)
}
