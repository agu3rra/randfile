use assert_cmd::Command;
use predicates::prelude::predicate;
use clap;

fn setup() -> Command {
    Command::cargo_bin(clap::crate_name!()).unwrap()
}

#[test]
fn die_no_args() {
    let mut cmd = setup();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
}

#[test]
fn directory_404() {
    let mut cmd = setup();
    cmd.arg("iAmANonExistentDirectory");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));
}

#[test]
fn no_files_in_directory() {
    let mut cmd = setup();
    cmd.arg("empty");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("The provided directory has no files to select."));
}

#[test]
fn file_selected() {
    let mut cmd = setup();
    cmd.arg("withfiles");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("File selected at random: "));
}
