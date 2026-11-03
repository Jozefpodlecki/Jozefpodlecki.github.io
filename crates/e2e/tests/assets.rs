use portfolio_e2e::{
    config::TestConfig,
    fixtures::{FixtureScenario, FixtureServer},
    harness,
    pages::portfolio::PortfolioPage,
};
use thirtyfour::error::{WebDriverError, WebDriverResult};

#[tokio::test]
async fn production_distribution_contains_decodable_project_artwork() -> WebDriverResult<()> {
    let server = FixtureServer::start(FixtureScenario::Success).await.map_err(|error| {
        WebDriverError::RequestFailed(format!("Build dist before E2E: {error}"))
    })?;
    let mut config = TestConfig::from_env();
    config.base_url = server.url().to_owned();

    harness::run_with_config("production-assets", config, move |driver, config| Box::pin(async move {
        let page = PortfolioPage::new(driver, config);
        page.open("/projects").await?;
        page.wait_for_count("[data-component='project-artwork']", 3).await?;
        page.wait_for_images().await?;

        let result = driver.execute(
            "return Promise.all(['sudoku', 'win-drives', 'zipeek'].map(async slug => { const url = `images/${slug}.webp`; const response = await fetch(url, { cache: 'no-store' }); const blob = await response.blob(); return { slug, status: response.status, mime: response.headers.get('content-type'), size: blob.size }; }));",
            vec![],
        ).await?;
        let assets = result.json().as_array().ok_or_else(|| {
            WebDriverError::RequestFailed(format!("Invalid asset response: {}", result.json()))
        })?;
        for asset in assets {
            portfolio_e2e::ensure_eq!(asset["status"].as_u64(), Some(200), "Missing project artwork: {asset}");
            portfolio_e2e::ensure_eq!(asset["mime"].as_str(), Some("image/webp"), "Invalid image MIME: {asset}");
            portfolio_e2e::ensure!(asset["size"].as_u64().unwrap_or(0) > 1_000, "Artwork file is unexpectedly small: {asset}");
        }

        let state = driver.execute(
            "return [...document.querySelectorAll('[data-component=project-artwork]')].map(e => e.dataset.imageStatus);",
            vec![],
        ).await?;
        portfolio_e2e::ensure_eq!(
            state.json().as_array().map(|values| values.iter().filter(|v| v.as_str() == Some("ready")).count()),
            Some(3),
            "The production distribution displayed image fallbacks: {}", state.json()
        );
        Ok(())
    })).await
}

#[tokio::test]
async fn deployed_site_smoke_test_when_url_is_configured() -> WebDriverResult<()> {
    let Ok(url) = std::env::var("E2E_DEPLOYED_URL") else {
        return Ok(());
    };
    let mut config = TestConfig::from_env();
    config.base_url = url.trim_end_matches('/').to_owned();
    harness::run_with_config("deployed-production-smoke", config, move |driver, config| Box::pin(async move {
        let page = PortfolioPage::new(driver, config);
        page.open("/projects").await?;
        page.wait_for_count("[data-component='project-artwork']", 3).await?;
        page.wait_for_images().await?;
        let state = driver.execute(
            "return [...document.querySelectorAll('[data-component=project-artwork]')].every(e => e.dataset.imageStatus === 'ready');",
            vec![],
        ).await?;
        portfolio_e2e::ensure_eq!(state.json().as_bool(), Some(true), "Deployed project artwork failed to decode");
        page.open("/projects/sudoku").await?;
        page.wait_for_present("[data-component='project-case-study']").await?;
        page.wait_for_images().await?;
        Ok(())
    })).await
}
