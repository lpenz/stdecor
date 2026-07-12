// Copyright (C) 2026 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::io::Write;
use std::process::Command;

fn stdecor() -> Command {
    Command::new(env!("CARGO_BIN_EXE_stdecor"))
}

#[test]
fn basic_prefix() {
    let output = stdecor()
        .args(["-p", "[test]", "--", "echo", "hello"])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "[test] hello\n");
}

#[test]
fn stderr_prefix() {
    let output = stdecor()
        .args(["-p", "[err]", "--", "sh", "-c", "echo ok >&2; echo out"])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "[err] out\n");
    assert_eq!(String::from_utf8(output.stderr).unwrap(), "[err] ok\n");
}

#[test]
fn quick_exit() {
    let output = stdecor()
        .args(["-p", "[q]", "--", "printf", "line1\nline2\n"])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        "[q] line1\n[q] line2\n"
    );
}

#[test]
fn nonzero_exit() {
    let output = stdecor()
        .args(["-p", "[x]", "--", "sh", "-c", "echo fail; exit 42"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(42));
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "[x] fail\n");
}

#[test]
fn both_streams_quick_exit() {
    let output = stdecor()
        .args([
            "-p",
            "[b]",
            "--",
            "sh",
            "-c",
            "echo out; echo err >&2; exit 0",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "[b] out\n");
    assert_eq!(String::from_utf8(output.stderr).unwrap(), "[b] err\n");
}

#[test]
fn date_prefix() {
    let output = stdecor()
        .args(["-d", "--", "echo", "hello"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    // Timestamp format: 2026-07-11T22:05:00.123+00:00
    assert!(
        stdout.starts_with("20"),
        "expected timestamp prefix, got: {stdout}"
    );
    assert!(stdout.contains("hello"));
}

#[test]
fn pipe_mode() {
    let mut child = stdecor()
        .args(["-p", "[pipe]"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(b"line1\nline2\n")
        .unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        "[pipe] line1\n[pipe] line2\n"
    );
}

#[test]
fn no_trailing_newline() {
    let output = stdecor()
        .args(["-p", "[n]", "--", "printf", "no newline"])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        "[n] no newline\n"
    );
}

#[test]
fn without_separator() {
    let output = stdecor()
        .args(["-p", "[sep]", "echo", "hello"])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "[sep] hello\n");
}

#[test]
fn date_prefix_with_text() {
    let output = stdecor()
        .args(["-d", "-p", "[ts]", "--", "echo", "hello"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.starts_with("20"),
        "expected timestamp prefix, got: {stdout}"
    );
    assert!(stdout.contains("[ts] "));
    assert!(stdout.contains("hello"));
}

#[test]
fn pipe_mode_date() {
    let mut child = stdecor()
        .args(["-d", "-p", "[pipe]"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all(b"line1\n").unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.starts_with("20"),
        "expected timestamp prefix, got: {stdout}"
    );
    assert!(stdout.contains("[pipe] "));
    assert!(stdout.contains("line1"));
}

#[test]
fn pipe_mode_empty() {
    let mut child = stdecor()
        .args(["-p", "[empty]"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    drop(child.stdin.take().unwrap());
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "");
}

#[test]
fn command_no_output() {
    let output = stdecor()
        .args(["-p", "[silent]", "--", "/bin/true"])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "");
    assert_eq!(String::from_utf8(output.stderr).unwrap(), "");
}

#[test]
fn invalid_command() {
    let output = stdecor()
        .args(["-p", "[bad]", "--", "nonexistent_command_xyz"])
        .output()
        .unwrap();
    assert!(!output.status.success());
}
