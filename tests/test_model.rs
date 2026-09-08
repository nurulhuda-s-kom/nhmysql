/*
 * -----------------------------------------------------------
 *  Project  : nhmysql - MySQL Model Generator for Rust
 *  Author   : Nurul Huda, S.Kom
 *  Contact  : +62823-1371-4009
 *  License  : MIT
 * -----------------------------------------------------------
 */

use std::process::Command;

#[test]
fn test_nhmysql_help() {
    let output = Command::new("nhmysql")
        .arg("--help")
        .output()
        .expect("Failed to execute nhmysql");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("MySQL") || stdout.contains("model"));
}

#[test]
fn test_nhmysql_version() {
    let output = Command::new("nhmysql")
        .arg("--version")
        .output()
        .expect("Failed to execute nhmysql");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("nhmysql"));
}

#[test]
fn test_nhmysql_no_args_error() {
    let output = Command::new("nhmysql")
        .output()
        .expect("Failed to execute nhmysql");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let combined = format!("{}{}", stdout, stderr);
    assert!(
        combined.contains("Provide table name(s)")
            || combined.contains("error")
            || combined.contains("DATABASE_URL")
    );
}
