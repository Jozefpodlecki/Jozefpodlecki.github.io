use crate::{artifacts, config::TestConfig, observability, server::TrunkServer};
use std::{future::Future, pin::Pin};
use thirtyfour::prelude::*;
use tracing::{debug, error, info, info_span, warn, Instrument};

pub type TestAction<'a> =
    Pin<Box<dyn Future<Output = WebDriverResult<()>> + Send + 'a>>;

pub async fn run<F>(action: F) -> WebDriverResult<()>
where
    F: for<'a> FnOnce(&'a WebDriver, &'a TestConfig) -> TestAction<'a>,
{
    run_named("browser-test", action).await
}

pub async fn run_named<F>(name: &str, action: F) -> WebDriverResult<()>
where
    F: for<'a> FnOnce(&'a WebDriver, &'a TestConfig) -> TestAction<'a>,
{
    observability::init();
    let span = info_span!("e2e_test", name = %name);

    async move {
        info!("Starting Trunk-backed browser test");
        let server = TrunkServer::start_named(name).await?;

        let mut config = TestConfig::from_env();
        config.base_url = server.base_url.clone();

        let result = run_browser(name, config, action).await;
        let cleanup = server.stop().await;
        finish(result, cleanup, "Trunk server")
    }
    .instrument(span)
    .await
}

pub async fn run_with_config<F>(
    name: &str,
    config: TestConfig,
    action: F,
) -> WebDriverResult<()>
where
    F: for<'a> FnOnce(&'a WebDriver, &'a TestConfig) -> TestAction<'a>,
{
    observability::init();
    let span = info_span!("e2e_fixture_test", name = %name, base_url = %config.base_url);
    run_browser(name, config, action).instrument(span).await
}

async fn run_browser<F>(name: &str, config: TestConfig, action: F) -> WebDriverResult<()>
where
    F: for<'a> FnOnce(&'a WebDriver, &'a TestConfig) -> TestAction<'a>,
{
    let mut caps = DesiredCapabilities::chrome();

    if config.headless {
        caps.add_arg("--headless=new")?;
    }

    caps.add_arg("--window-size=1440,900")?;
    caps.add_arg("--disable-dev-shm-usage")?;

    info!(headless = config.headless, base_url = %config.base_url, "Starting managed WebDriver");
    let driver = WebDriver::managed(caps).await?;
    let result = action(&driver, &config).await;

    if let Err(ref failure) = result {
        error!(error = %failure, "Browser test failed");
        if let Ok(url) = driver.current_url().await {
            warn!(%url, "Browser location at failure");
        }
        match artifacts::capture_failure(&driver, name).await {
            Ok(path) => info!(path = %path.display(), "Saved failure screenshot"),
            Err(error) => warn!(%error, "Could not capture failure screenshot"),
        }
    } else {
        info!("Browser assertions passed");
    }

    debug!("Closing WebDriver session");
    let cleanup = driver.quit().await;
    finish(result, cleanup, "WebDriver session")
}

fn finish(
    result: WebDriverResult<()>,
    cleanup: WebDriverResult<()>,
    resource: &str,
) -> WebDriverResult<()> {
    if let Err(ref error) = cleanup {
        warn!(%error, resource, "Cleanup failed");
    }

    match result {
        Err(error) => Err(error),
        Ok(()) => cleanup,
    }
}
