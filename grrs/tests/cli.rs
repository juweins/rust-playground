use assert_cmd::{prelude::*, assert}; // Add methods on commands
use assert_fs::prelude::*; use grrs::search_file;
// Used for writing files
use predicates::prelude::*; // Used for writing assertions
use std::{process::Command}; // Run programs


// Test if the program exits with an error if no arguments are provided
// - Create a new Command
// - Run the program
// - Assert that the program exits with an error
// - Assert that the error message contains the expected message
#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("grrs")?;

    cmd.arg("foobar").arg("test/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));

    Ok(())
}

// Test if the program exits with an error if no arguments are provided
// - Create a new Command
// - Run the program
// - Assert that the program exits successfully
// - Assert that the error message contains the expected message
// TODO: Find a way to make this test more readable without formatting the output
#[test]
fn find_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("A test\nActual content\nMore content\nAnother test")?;

    // test if the file contains the expected content
    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.arg("test").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("A \x1b[1;31mtest\x1b[0m"))
        .stdout(predicate::str::contains("Another \x1b[1;31mtest\x1b[0m"));

    Ok(())
}

// Test if the program exits with an error if no arguments are provided
// - Create a new Command
// - Run the program
// - Assert that the program exits with an error
// - Assert that the error message contains the expected error-message
#[test]
fn find_empty_string_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("A test\nActual content\nMore content\nAnother test")?;

    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.arg("").arg(file.path());
    cmd.assert()
        .success()
        .stderr(predicate::str::contains("No pattern provided!"));

    Ok(())
}