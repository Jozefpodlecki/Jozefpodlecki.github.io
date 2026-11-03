use portfolio_e2e::config::TestConfig;

#[test]
fn route_url_handles_hash_routing() {
    let cfg = TestConfig {
        base_url: "https://example.org/subpath".into(),
        timeout: std::time::Duration::from_secs(1),
        poll_interval: std::time::Duration::from_millis(1),
        headless: true,
    };
    assert_eq!(cfg.route_url("/projects"), "https://example.org/subpath/#/projects");
}
