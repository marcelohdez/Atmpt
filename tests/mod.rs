use std::{env, path::PathBuf};

use assert_cmd::Command;
use atmpt::{ALWAYS_DELETE_KEY, EDITOR_KEY, TEMPLATE_DIR_KEY};

const PROJECT_DIR: &str = env!("CARGO_MANIFEST_DIR");

fn cmd() -> Command {
    env::set_var(EDITOR_KEY, "echo"); // exit on success
    env::set_var(ALWAYS_DELETE_KEY, "true");

    let templates = PathBuf::from_iter([PROJECT_DIR, "templates"]);
    env::set_var(TEMPLATE_DIR_KEY, templates.to_string_lossy().as_ref());

    Command::cargo_bin("atmpt").unwrap()
}

// ======= Failures =======

#[test]
fn fail_on_conflicting_opts() {
    cmd().args(["-l", "-d"]).assert().failure();
}

#[test]
fn fail_on_conflicting_opts_with_template() {
    cmd().args(["cpp", "-l", "-d"]).assert().failure();
}

#[test]
fn incorrect_template() {
    cmd().arg("_blahblah!").assert().failure();
}

// ======= Successes =======

#[test]
fn correct_template() {
    cmd().arg("cpp").assert().success();
}
