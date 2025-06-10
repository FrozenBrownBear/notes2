use std::fs;

#[test]
fn sccache_config_present() {
    let config = fs::read_to_string(".cargo/config.toml").expect("read config");
    assert!(config.contains("rustc-wrapper = \"sccache\""), "sccache not configured");
}
