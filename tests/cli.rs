use assert_cmd::Command;
use predicates::str::contains;

#[test]
fn parses_valid_tag_via_cli() {
    let mut cmd = Command::cargo_bin("notes2-cli").unwrap();
    cmd.write_stdin("/option_volume_int/\n\n")
        .assert()
        .stdout(contains("parsed: Option"));
}

#[test]
fn reports_invalid_tag_via_cli() {
    let mut cmd = Command::cargo_bin("notes2-cli").unwrap();
    cmd.write_stdin("/invalid/\n\n")
        .assert()
        .stdout(contains("invalid tag"));
}
