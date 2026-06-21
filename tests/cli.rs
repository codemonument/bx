use assert_cmd::Command;
use assert_fs::prelude::*;
use predicates::prelude::*;

#[test]
fn test_version_flag_short() {
	Command::cargo_bin("bx")
		.unwrap()
		.arg("-v")
		.assert()
		.success()
		.stdout(predicate::str::contains("bx v"));
}

#[test]
fn test_version_flag_long() {
	Command::cargo_bin("bx")
		.unwrap()
		.arg("--version")
		.assert()
		.success()
		.stdout(predicate::str::contains("bx v"));
}

#[test]
fn test_help_flag_short() {
	Command::cargo_bin("bx")
		.unwrap()
		.arg("-h")
		.assert()
		.success()
		.stdout(predicate::str::contains("help page"));
}

#[test]
fn test_help_flag_long() {
	Command::cargo_bin("bx")
		.unwrap()
		.arg("--help")
		.assert()
		.success()
		.stdout(predicate::str::contains("help page"));
}

#[test]
fn test_missing_config_file() {
	let temp = assert_fs::TempDir::new().unwrap();
	Command::cargo_bin("bx")
		.unwrap()
		.current_dir(temp.path())
		.arg("build")
		.assert()
		.failure()
		.stderr(predicate::str::contains("Error"));
}

#[test]
fn test_init_creates_config() {
	let temp = assert_fs::TempDir::new().unwrap();
	Command::cargo_bin("bx")
		.unwrap()
		.current_dir(temp.path())
		.arg("-i")
		.assert()
		.success()
		.stdout(predicate::str::contains("initialized"));

	temp.child("bx.toml").assert(predicate::path::exists());
}

#[test]
fn test_cache_creates_cache_file() {
	let temp = assert_fs::TempDir::new().unwrap();
	let config = r#"
version = "0.3.2"

[scripts]
build = "echo building"
"#;
	temp.child("bx.toml").write_str(config).unwrap();

	Command::cargo_bin("bx")
		.unwrap()
		.current_dir(temp.path())
		.arg("-c")
		.assert()
		.success();

	temp.child(".bx.cache.json")
		.assert(predicate::path::exists());
}

#[test]
fn test_debug_mode() {
	let temp = assert_fs::TempDir::new().unwrap();
	let config = r#"
version = "0.3.2"

[scripts]
build = "echo building"
"#;
	temp.child("bx.toml").write_str(config).unwrap();

	Command::cargo_bin("bx")
		.unwrap()
		.current_dir(temp.path())
		.args(["-d", "build"])
		.assert()
		.success()
		.stdout(predicate::str::contains("Running command"));
}
