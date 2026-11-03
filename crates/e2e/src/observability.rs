use std::sync::Once;
use tracing_subscriber::{fmt, EnvFilter};

static INITIALIZED: Once = Once::new();

pub fn init() {
    INITIALIZED.call_once(|| {
        let filter = EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new("portfolio_e2e=info,warn"));

        let _ = fmt()
            .with_env_filter(filter)
            .with_target(true)
            .with_test_writer()
            .try_init();
    });
}
