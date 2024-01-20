/// These tests check that the app can be run from the command line (cli)
/// with the required combination of arguments.

use anyhow::Result;
use assert_cmd::prelude::*;
// use predicates::prelude::*;
use std::process::Command;
// use assert_fs::prelude::*;

#[test]
fn can_run_from_cli_with_one_string_arg() -> Result<()> {
	
	// load the main function in the binary file and run the
	// built version of it
	let mut cmd = Command::cargo_bin("cctr")?;

	// add the arg c
	cmd.arg("c");

	// make sure the function is successful and returns an ok exit code
	cmd.assert()
		.success()
		.code(0);

	Ok(())
	
}

#[test]
fn can_run_from_cli_with_two_string_args() -> Result<()> {
	
	// load the main function in the binary file and run the
	// built version of it
	let mut cmd = Command::cargo_bin("cctr")?;

	// add the arg c
	cmd.arg("c").arg("C");

	// make sure the function is successful and returns an ok exit code
	cmd.assert()
		.success()
		.code(0);

	Ok(())
	
}

#[test]
fn can_run_from_cli_with_one_flag() -> Result<()> {
	
	// load the main function in the binary file and run the
	// built version of it
	let mut cmd = Command::cargo_bin("cctr")?;

	// add the arg c and the second arg C
	cmd.arg("-C").arg("c");

	// make sure the function is successful and returns an ok exit code
	cmd.assert()
		.success()
		.code(0);

	Ok(())
	
}

#[test]
fn can_run_from_cli_with_all_flags() -> Result<()> {
	
	// load the main function in the binary file and run the
	// built version of it
	let mut cmd = Command::cargo_bin("cctr")?;

	// add the arg c plus all five flags
	cmd.arg("-Ccsud").arg("c");

	// make sure the function is successful and returns an ok exit code
	cmd.assert()
		.success()
		.code(0);

	Ok(())
	
}

#[test]
fn running_with_three_string_args_returns_error() -> Result<()> {
	
	// load the main function in the binary file and run the
	// built version of it
	let mut cmd = Command::cargo_bin("cctr")?;

	// add the arg c
	cmd.arg("c").arg("C").arg("a");

	// make sure the function is a failure and returns a misuse of shell exit code
	cmd.assert()
		.failure()
		.code(2);

	Ok(())
	
}

#[test]
fn running_with_unknown_flag_returns_error() -> Result<()> {
	
	// load the main function in the binary file and run the
	// built version of it
	let mut cmd = Command::cargo_bin("cctr")?;

	// add the arg c
	cmd.arg("-a").arg("C");

	// make sure the function is a failure and returns a misuse of shell exit code
	cmd.assert()
		.failure()
		.code(2);

	Ok(())
	
}

#[test]
fn running_with_an_empty_string_returns_error() -> Result<()> {
	
	// load the main function in the binary file and run the
	// built version of it
	let mut cmd = Command::cargo_bin("cctr")?;

	// add the empty string arg
	cmd.arg("");

	// make sure the function is a failure and returns a misuse of shell exit code
	cmd.assert()
		.failure()
		.code(2);

	Ok(())
	
}