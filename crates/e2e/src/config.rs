use std::{env, time::Duration};

#[derive(Debug, Clone)]
pub struct TestConfig {
    pub base_url: String,
    pub timeout: Duration,
    pub poll_interval: Duration,
    pub headless: bool,
}

impl TestConfig {
    pub fn from_env() -> Self {
        Self {
            base_url: env::var("E2E_BASE_URL").unwrap_or_else(|_| "http://localhost:1420".into()).trim_end_matches('/').to_owned(),
            timeout: Duration::from_secs(env::var("E2E_TIMEOUT_SECS").ok().and_then(|v| v.parse().ok()).unwrap_or(20)),
            poll_interval: Duration::from_millis(200),
            headless: env::var("E2E_HEADLESS").map(|v| v != "0" && !v.eq_ignore_ascii_case("false")).unwrap_or(true),
        }
    }

    pub fn route_url(&self, hash_route: &str) -> String {
        format!("{}/#{}", self.base_url, hash_route)
    }
}
