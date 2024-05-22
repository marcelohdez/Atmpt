use std::{env, path::PathBuf};

use assert_cmd::Command;
use atmpt::{ALWAYS_DELETE_KEY, DATA_DIR_KEY, EDITOR_KEY};

const PROJECT_DIR: &str = env!("CARGO_MANIFEST_DIR");

fn cmd() -> Command {
    env::set_var(EDITOR_KEY, "echo"); // exit on success
    env::set_var(ALWAYS_DELETE_KEY, "true");

    let templates = PathBuf::from_iter([PROJECT_DIR, "templates"]);
    env::set_var(DATA_DIR_KEY, templates.to_string_lossy().as_ref());

    Command::cargo_bin("atmpt").unwrap()
}

// FIXME: Due to the folder being named with the time of creation, multiple
// tests using the same language template may clash and fail...

// ======= Failures =======
#[test]
fn fail_on_conflicting_opts() {
    cmd().args(["-l", "-d"]).assert().failure();
}

#[test]
fn fail_on_conflicting_opts_with_template() {
    cmd().args(["c", "-ld"]).assert().failure();
}

#[test]
fn fail_on_keep_and_delete() {
    cmd().args(["python", "-ny"]).assert().failure();
}

#[test]
fn fail_on_incorrect_template() {
    cmd().arg("_blahblah!").assert().failure();
}

#[test]
fn fail_on_incorrect_editor() {
    cmd().args(["-e", "fakeeditor", "java"]).assert().failure();
}

#[test]
fn fail_on_no_args() {
    cmd().assert().failure();
}

// TODO: Some way of setting each test's dir so this does not make pass_on_retry fail
//
//#[test]
//fn fail_on_retry_without_session_data() {
//    let session = get_session_path();
//
//    if session.exists() {
//        fs::remove_file(get_session_path()).unwrap();
//    }
//
//    cmd().arg("-r").assert().failure();
//}

// ======= Successes =======
#[test]
fn pass_on_correct_template() {
    cmd().arg("cpp").assert().success();
}

//#[test]
//fn pass_on_retry() {
//    cmd().arg("cpp").assert().success();
//    cmd().arg("-r").assert().success();
//}
