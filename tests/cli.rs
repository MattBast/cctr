/// These tests check that the app can be run from the command line (cli)
/// with the required combination of arguments.
use anyhow::Result;
use assert_cmd::prelude::*;
use std::process::{Command, Stdio};

// ************************************************************************
// translate mode tests
// ************************************************************************

#[test]
fn can_run_in_translate_mode() -> Result<()> {
    // load the main function in the binary file and run the
    // built version of it
    let mut cmd = Command::cargo_bin("cctr")?;

    // add the arg c
    cmd.arg("c").arg("C");

    // make sure the function is successful and returns an ok exit code
    cmd.assert().success().code(0);

    Ok(())
}

#[test]
fn can_use_the_capital_c_flag_in_translate_mode() -> Result<()> {
    // load the main function in the binary file and run the
    // built version of it
    let mut cmd = Command::cargo_bin("cctr")?;

    // add two args and the s flag
    cmd.arg("-C").arg("c").arg("C");

    // make sure the function is successful and returns an ok exit code
    cmd.assert().success().code(0);

    Ok(())
}

#[test]
fn can_use_the_c_flag_in_translate_mode() -> Result<()> {
    // load the main function in the binary file and run the
    // built version of it
    let mut cmd = Command::cargo_bin("cctr")?;

    // add two args and the s flag
    cmd.arg("-c").arg("c").arg("C");

    // make sure the function is successful and returns an ok exit code
    cmd.assert().success().code(0);

    Ok(())
}

#[test]
fn can_use_the_s_flag_in_translate_mode() -> Result<()> {
    // load the main function in the binary file and run the
    // built version of it
    let mut cmd = Command::cargo_bin("cctr")?;

    // add two args and the s flag
    cmd.arg("-s").arg("c").arg("C");

    // make sure the function is successful and returns an ok exit code
    cmd.assert().success().code(0);

    Ok(())
}

#[test]
fn can_use_the_u_flag_in_translate_mode() -> Result<()> {
    // load the main function in the binary file and run the
    // built version of it
    let mut cmd = Command::cargo_bin("cctr")?;

    // add two args and the s flag
    cmd.arg("-u").arg("c").arg("C");

    // make sure the function is successful and returns an ok exit code
    cmd.assert().success().code(0);

    Ok(())
}

#[test]
fn can_use_the_ccsu_flags_in_translate_mode() -> Result<()> {
    // load the main function in the binary file and run the
    // built version of it
    let mut cmd = Command::cargo_bin("cctr")?;

    // add two args and the s flag
    cmd.arg("-Ccsu").arg("c").arg("C");

    // make sure the function is successful and returns an ok exit code
    cmd.assert().success().code(0);

    Ok(())
}

// ************************************************************************
// delete mode tests
// ************************************************************************

#[test]
fn can_run_in_delete_mode() -> Result<()> {
    // load the main function in the binary file and run the
    // built version of it
    let mut cmd = Command::cargo_bin("cctr")?;

    // add the arg c plus the d flag
    cmd.arg("-d").arg("c");

    // make sure the function is successful and returns an ok exit code
    cmd.assert().success().code(0);

    Ok(())
}

#[test]
fn can_run_in_delete_mode_with_ccu_flags() -> Result<()> {
    // load the main function in the binary file and run the
    // built version of it
    let mut cmd = Command::cargo_bin("cctr")?;

    // add the arg c plus the d flag
    cmd.arg("-Ccud").arg("c");

    // make sure the function is successful and returns an ok exit code
    cmd.assert().success().code(0);

    Ok(())
}

#[test]
fn two_string_with_just_the_delete_flag_return_error() -> Result<()> {
    // load the main function in the binary file and run the
    // built version of it
    let mut cmd = Command::cargo_bin("cctr")?;

    // add two args and the d flag
    cmd.arg("-d").arg("c").arg("C");

    // make sure the function is a failure and returns a misuse of shell exit code
    cmd.assert().failure().code(2);

    Ok(())
}

#[test]
fn one_string_and_no_d_or_s_flag_returns_error() -> Result<()> {
    // load the main function in the binary file and run the
    // built version of it
    let mut cmd = Command::cargo_bin("cctr")?;

    // add the arg c
    cmd.arg("c");

    // make sure the function is a failure and returns a misuse of shell exit code
    cmd.assert().failure().code(2);

    Ok(())
}

#[test]
fn one_string_and_flag_that_is_not_d_or_s_returns_error() -> Result<()> {
    // load the main function in the binary file and run the
    // built version of it
    let mut cmd = Command::cargo_bin("cctr")?;

    // add the arg c and the second arg C
    cmd.arg("-C").arg("c");

    // make sure the function is a failure and returns a misuse of shell exit code
    cmd.assert().failure().code(2);

    Ok(())
}

#[test]
fn adding_all_flags_and_only_one_string_returns_error() -> Result<()> {
    // load the main function in the binary file and run the
    // built version of it
    let mut cmd = Command::cargo_bin("cctr")?;

    // add the arg c plus all five flags
    cmd.arg("-Ccsud").arg("c");

    // make sure the function is a failure and returns a misuse of shell exit code
    cmd.assert().failure().code(2);

    Ok(())
}

// ************************************************************************
// compress mode tests
// ************************************************************************

#[test]
fn can_run_in_compress_mode() -> Result<()> {
    // load the main function in the binary file and run the
    // built version of it
    let mut cmd = Command::cargo_bin("cctr")?;

    // add the arg c plus the s flag
    cmd.arg("-s").arg("c");

    // make sure the function is successful and returns an ok exit code
    cmd.assert().success().code(0);

    Ok(())
}

// ************************************************************************
// delete and compress mode tests
// ************************************************************************

#[test]
fn can_run_in_delete_and_compress_mode() -> Result<()> {
    // load the main function in the binary file and run the
    // built version of it
    let mut cmd = Command::cargo_bin("cctr")?;

    // add the arg c plus all five flags
    cmd.arg("-Ccsud").arg("c").arg("C");

    // make sure the function is successful and returns an ok exit code
    cmd.assert().success().code(0);

    Ok(())
}

// ************************************************************************
// other bad input tests
// ************************************************************************

#[test]
fn running_with_three_string_args_returns_error() -> Result<()> {
    // load the main function in the binary file and run the
    // built version of it
    let mut cmd = Command::cargo_bin("cctr")?;

    // add the arg c
    cmd.arg("c").arg("C").arg("a");

    // make sure the function is a failure and returns a misuse of shell exit code
    cmd.assert().failure().code(2);

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
    cmd.assert().failure().code(2);

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
    cmd.assert().failure().code(2);

    Ok(())
}

// ************************************************************************
// delete flag tests
// ************************************************************************

#[test]
fn can_delete_single_characters() -> Result<()> {
    // mock an echo command that then gets piped to the cctr stdin
    let echo_out = Command::new("echo")
        .arg("Coding challenge")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start echo process")
        .stdout
        .expect("Failed to open echo stdout");

    // run the cctr cli tool
    let output = Command::cargo_bin("cctr")?
        .stdin(Stdio::from(echo_out))
        .arg("-d")
        .arg("C")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start cctr process")
        .wait_with_output()
        .expect("Failed to wait on cctr");

    assert_eq!(b"oding challenge\n", output.stdout.as_slice());

    Ok(())
}

#[test]
fn can_delete_many_characters() -> Result<()> {
    // mock an echo command that then gets piped to the cctr stdin
    let echo_out = Command::new("echo")
        .arg("Coding challenge")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start echo process")
        .stdout
        .expect("Failed to open echo stdout");

    // run the cctr cli tool
    let output = Command::cargo_bin("cctr")?
        .stdin(Stdio::from(echo_out))
        .arg("-d")
        .arg("Cdg")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start cctr process")
        .wait_with_output()
        .expect("Failed to wait on cctr");

    assert_eq!(b"oin challene\n", output.stdout.as_slice());

    Ok(())
}

#[test]
fn can_delete_everything() -> Result<()> {
    // mock an echo command that then gets piped to the cctr stdin
    let echo_out = Command::new("echo")
        .arg("Coding challenge")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start echo process")
        .stdout
        .expect("Failed to open echo stdout");

    // run the cctr cli tool
    let output = Command::cargo_bin("cctr")?
        .stdin(Stdio::from(echo_out))
        .arg("-d")
        .arg("Coding challenge")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start cctr process")
        .wait_with_output()
        .expect("Failed to wait on cctr");

    assert_eq!(b"\n", output.stdout.as_slice());

    Ok(())
}

#[test]
fn can_delete_digits() -> Result<()> {
    // mock an echo command that then gets piped to the cctr stdin
    let echo_out = Command::new("echo")
        .arg("123 challenge")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start echo process")
        .stdout
        .expect("Failed to open echo stdout");

    // run the cctr cli tool
    let output = Command::cargo_bin("cctr")?
        .stdin(Stdio::from(echo_out))
        .arg("-d")
        .arg("123")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start cctr process")
        .wait_with_output()
        .expect("Failed to wait on cctr");

    assert_eq!(b" challenge\n", output.stdout.as_slice());

    Ok(())
}

#[test]
fn can_delete_whitespace() -> Result<()> {
    // mock an echo command that then gets piped to the cctr stdin
    let echo_out = Command::new("echo")
        .arg("Coding challenge")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start echo process")
        .stdout
        .expect("Failed to open echo stdout");

    // run the cctr cli tool
    let output = Command::cargo_bin("cctr")?
        .stdin(Stdio::from(echo_out))
        .arg("-d")
        .arg(" ")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start cctr process")
        .wait_with_output()
        .expect("Failed to wait on cctr");

    assert_eq!(b"Codingchallenge\n", output.stdout.as_slice());

    Ok(())
}

#[test]
fn can_delete_special_chars() -> Result<()> {
    // mock an echo command that then gets piped to the cctr stdin
    let echo_out = Command::new("echo")
        .arg("Coding@challenge")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start echo process")
        .stdout
        .expect("Failed to open echo stdout");

    // run the cctr cli tool
    let output = Command::cargo_bin("cctr")?
        .stdin(Stdio::from(echo_out))
        .arg("-d")
        .arg("@")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start cctr process")
        .wait_with_output()
        .expect("Failed to wait on cctr");

    assert_eq!(b"Codingchallenge\n", output.stdout.as_slice());

    Ok(())
}

#[test]
fn can_delete_upper_class() -> Result<()> {
    // mock an echo command that then gets piped to the cctr stdin
    let echo_out = Command::new("echo")
        .arg("CoDinG challenge")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start echo process")
        .stdout
        .expect("Failed to open echo stdout");

    // run the cctr cli tool
    let output = Command::cargo_bin("cctr")?
        .stdin(Stdio::from(echo_out))
        .arg("-d")
        .arg("[:upper:]")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start cctr process")
        .wait_with_output()
        .expect("Failed to wait on cctr");

    assert_eq!(b"oin challenge\n", output.stdout.as_slice());

    Ok(())
}

#[test]
fn can_delete_lower_class() -> Result<()> {
    // mock an echo command that then gets piped to the cctr stdin
    let echo_out = Command::new("echo")
        .arg("CoDinG challenge")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start echo process")
        .stdout
        .expect("Failed to open echo stdout");

    // run the cctr cli tool
    let output = Command::cargo_bin("cctr")?
        .stdin(Stdio::from(echo_out))
        .arg("-d")
        .arg("[:lower:]")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start cctr process")
        .wait_with_output()
        .expect("Failed to wait on cctr");

    assert_eq!(b"CDG \n", output.stdout.as_slice());

    Ok(())
}

#[test]
fn can_delete_alphabetic_chars() -> Result<()> {
    // mock an echo command that then gets piped to the cctr stdin
    let echo_out = Command::new("echo")
        .arg("123 challenge")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start echo process")
        .stdout
        .expect("Failed to open echo stdout");

    // run the cctr cli tool
    let output = Command::cargo_bin("cctr")?
        .stdin(Stdio::from(echo_out))
        .arg("-d")
        .arg("[:alpha:]")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start cctr process")
        .wait_with_output()
        .expect("Failed to wait on cctr");

    assert_eq!(b"123 \n", output.stdout.as_slice());

    Ok(())
}

#[test]
fn can_delete_alphanumeric_chars() -> Result<()> {
    // mock an echo command that then gets piped to the cctr stdin
    let echo_out = Command::new("echo")
        .arg("123@challenge")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start echo process")
        .stdout
        .expect("Failed to open echo stdout");

    // run the cctr cli tool
    let output = Command::cargo_bin("cctr")?
        .stdin(Stdio::from(echo_out))
        .arg("-d")
        .arg("[:alnum:]")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start cctr process")
        .wait_with_output()
        .expect("Failed to wait on cctr");

    assert_eq!(b"@\n", output.stdout.as_slice());

    Ok(())
}

#[test]
fn can_delete_blanks() -> Result<()> {
    // mock an echo command that then gets piped to the cctr stdin
    let echo_out = Command::new("echo")
        .arg("123 challenge")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start echo process")
        .stdout
        .expect("Failed to open echo stdout");

    // run the cctr cli tool
    let output = Command::cargo_bin("cctr")?
        .stdin(Stdio::from(echo_out))
        .arg("-d")
        .arg("[:blank:]")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start cctr process")
        .wait_with_output()
        .expect("Failed to wait on cctr");

    assert_eq!(b"123challenge\n", output.stdout.as_slice());

    Ok(())
}

#[test]
fn can_delete_control_characters() -> Result<()> {
    // mock an echo command that then gets piped to the cctr stdin
    let echo_out = Command::new("echo")
        .arg("123\tchallenge")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start echo process")
        .stdout
        .expect("Failed to open echo stdout");

    // run the cctr cli tool
    let output = Command::cargo_bin("cctr")?
        .stdin(Stdio::from(echo_out))
        .arg("-d")
        .arg("[:cntrl:]")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start cctr process")
        .wait_with_output()
        .expect("Failed to wait on cctr");

    assert_eq!(b"123challenge\n", output.stdout.as_slice());

    Ok(())
}

#[test]
fn can_delete_digit_characters() -> Result<()> {
    // mock an echo command that then gets piped to the cctr stdin
    let echo_out = Command::new("echo")
        .arg("٣7৬¾ challenge")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start echo process")
        .stdout
        .expect("Failed to open echo stdout");

    // run the cctr cli tool
    let output = Command::cargo_bin("cctr")?
        .stdin(Stdio::from(echo_out))
        .arg("-d")
        .arg("[:digit:]")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start cctr process")
        .wait_with_output()
        .expect("Failed to wait on cctr");

    assert_eq!(b" challenge\n", output.stdout.as_slice());

    Ok(())
}
