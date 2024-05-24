use std::{env, ffi::OsStr, fs, path::PathBuf, str::FromStr};

use assert_cmd::{assert::Assert, Command};
use atmpt::{get_atmpt_dir, ALWAYS_DELETE_KEY, DATA_DIR_KEY, EDITOR_KEY, TMP_DIR_KEY};

fn cmd() -> Command {
    Command::cargo_bin(atmpt::PROGRAM_NAME).unwrap()
}

fn cmd_opts(
    tmp_dir: &str,
    args: impl IntoIterator<Item = impl AsRef<OsStr>>,
    delete_on_exit: bool,
    clear_tmp_dir: bool,
) -> Assert {
    let tmp_dir = get_atmpt_dir(&None).join(tmp_dir);
    if clear_tmp_dir && tmp_dir.exists() {
        fs::remove_dir_all(&tmp_dir).unwrap();
    }
    fs::create_dir_all(&tmp_dir).unwrap();

    let mut cmd = cmd();
    if delete_on_exit {
        cmd.env(ALWAYS_DELETE_KEY, "true");
    }

    cmd.env(EDITOR_KEY, "echo")
        .env(
            DATA_DIR_KEY,
            PathBuf::from_str(env!("CARGO_MANIFEST_DIR"))
                .unwrap()
                .join("templates"), // use templates folder as data directory
        )
        .env(TMP_DIR_KEY, tmp_dir)
        .args(args)
        .assert()
}

fn cmd_always_delete(tmp_dir: &str, args: impl IntoIterator<Item = impl AsRef<OsStr>>) -> Assert {
    cmd_opts(tmp_dir, args, true, false)
}

// ======= Failures =======
#[test]
fn fail_on_conflicting_opts() {
    cmd_always_delete("conflicting_opts", ["-ld"]).failure();
}

#[test]
fn fail_on_conflicting_opts_with_template() {
    cmd_always_delete("conflicting_opts_templ", ["c", "-ld"]).failure();
}

#[test]
fn fail_on_keep_and_delete() {
    cmd_always_delete("keep_and_delete", ["python", "-ny"]).failure();
}

#[test]
fn fail_on_incorrect_template() {
    cmd_always_delete("incorrect_templ", ["_blahblah!"]).failure();
}

#[test]
fn fail_on_incorrect_editor() {
    cmd_always_delete("incorrect_editor", ["-e", "fakeeditor", "java"]).failure();
}

#[test]
fn fail_on_no_args() {
    cmd_always_delete("no_args", [""]).failure();
}

#[test]
fn fail_on_retry_without_session_data() {
    cmd_opts("retry_no_session", ["-r"], true, true).failure();
}

// ======= Successes =======
#[test]
fn pass_on_correct_template() {
    cmd_always_delete("correct_templ", ["cpp"]).success();
}

#[test]
fn pass_on_retry() {
    const DIR: &str = "retry";

    cmd_always_delete(DIR, ["cpp"]).success();
    cmd_always_delete(DIR, ["-r"]).success();
}

#[test]
fn pass_on_previous() {
    const DIR: &str = "previous";

    cmd_opts(DIR, ["c", "-y"], false, true).success(); // keep directory with attempt
    cmd_always_delete(DIR, ["-p"]).success();
}
