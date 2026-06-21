use crate::version::CLI_VERSION;

pub fn help(output: &mut impl std::io::Write) {
	writeln!(
        output,
        "bx v{version} help page:
------------------------

bx is a command aliasing tool that supports extremely simple and extremely advanced syntax. For the full reference, please see the documentation at https://github.com/arctic-hen7/bonnie/wiki.
This just summarizes the functionality of this command, not the syntax of bx configuration files!

-h, --help                                      prints this help page
-v, --version                                   prints the current version of bx
-i, --init [-t, --template <template-file>]     creates a new config file (or whatever's set in `BX_CONF`/`BONNIE_CONF`), using the specified template file if provided
-c, --cache                                     caches the bx configuration file to `.bx.cache.json` for performance (this cache must be MANUALLY updated by re-running this command!)

help [command-name]                             prints the help page for the current bx configuration or for the given command

The config file location can be set via `BX_CONF` or `BONNIE_CONF` environment variables. Without these, `./bx.toml` is checked first, then `./bonnie.toml`.
The expected location of a bx cache file can be changed from the default `./.bx.cache.json` by setting the `BX_CACHE` or `BONNIE_CACHE` environment variable.
The expected location of your default template can be changed from the default `~/.bx/template.toml` by setting the `BX_TEMPLATE` or `BONNIE_TEMPLATE` environment variable.

Further information can be found at https://github.com/arctic-hen7/bonnie/wiki.",
        version = CLI_VERSION
    )
    .expect("Failed to write help page.")
}
