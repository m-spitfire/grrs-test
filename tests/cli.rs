use assert_cmd::prelude::*;
use predicates::prelude::*;
use assert_fs::prelude::*;
use std::process::Command;

const TEST_FILE_CONTENT: &str = "A test\nAactual content\nMore content\nAnother test";
#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.arg("foobar").arg("test/file/doesn/t/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Couldn't read file"));
    Ok(())
}

#[test]
fn find_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("test.txt")?;
    file.write_str(TEST_FILE_CONTENT)?;

    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.arg("test").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("test\nAnother test"));

    Ok(())
}

#[test]
fn empty_pattern() -> Result<(), Box<dyn std::error::Error>> {

    let file = assert_fs::NamedTempFile::new("test.txt")?;
    file.write_str(TEST_FILE_CONTENT)?;

    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.arg("").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(TEST_FILE_CONTENT));

    Ok(())
}
