use std::env::consts::{ARCH, OS};
use std::process::Command;

use chrono::prelude::Utc;

#[cfg(debug_assertions)]
const BUILD_TYPE: &str = "debug";
#[cfg(not(debug_assertions))]
const BUILD_TYPE: &str = "release";

pub fn version(pkg_name: &str, pkg_version: &str, git_dir: &str) -> String {
    if have_git(git_dir) {
        format!(
            "{} {} ({}:{}{}, {} build, {} [{}], {} UTC [{}])",
            pkg_name,
            pkg_version,
            get_branch_name(git_dir),
            get_commit_hash(git_dir),
            if is_working_tree_clean(git_dir) {
                ""
            } else {
                "+"
            },
            BUILD_TYPE,
            OS,
            ARCH,
            Utc::now().format("%b %d %Y, %T"),
            get_rustc_version(git_dir),
        )
    } else {
        format!(
            "{} {} ({} build, {} [{}], {} UTC [{}])",
            pkg_name,
            pkg_version,
            BUILD_TYPE,
            OS,
            ARCH,
            Utc::now().format("%b %d %Y, %T"),
            get_rustc_version(git_dir),
        )
    }
}

fn have_git(git_dir: &str) -> bool {
    Command::new("git")
        .arg("--version")
        .current_dir(git_dir)
        .output()
        .is_ok()
}

fn get_commit_hash(git_dir: &str) -> String {
    let output = Command::new("git")
        .arg("log")
        .arg("-1")
        .arg("--pretty=format:%h") // Abbreviated commit hash
        // .arg("--pretty=format:%H") // Full commit hash
        .current_dir(git_dir)
        .output()
        .unwrap();

    assert!(output.status.success());

    String::from_utf8_lossy(&output.stdout).to_string()
}

fn get_branch_name(git_dir: &str) -> String {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--abbrev-ref")
        .arg("HEAD")
        .current_dir(git_dir)
        .output()
        .unwrap();

    assert!(output.status.success());

    String::from_utf8_lossy(&output.stdout)
        .trim_end()
        .to_string()
}

fn is_working_tree_clean(git_dir: &str) -> bool {
    let status = Command::new("git")
        .arg("diff")
        .arg("--quiet")
        .arg("--exit-code")
        .current_dir(git_dir)
        .status()
        .unwrap();

    status.code().unwrap() == 0
}

fn get_rustc_version(git_dir: &str) -> String {
    let output = Command::new("rustc")
        .arg("--version")
        .current_dir(git_dir)
        .output()
        .unwrap();

    assert!(output.status.success());

    String::from_utf8_lossy(&output.stdout)
        .trim_end()
        .to_string()
}
