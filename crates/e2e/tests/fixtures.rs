use portfolio_e2e::{
    config::TestConfig,
    fixtures::{FixtureScenario, FixtureServer},
    harness,
    pages::portfolio::PortfolioPage,
    selectors,
};
use thirtyfour::error::{WebDriverError, WebDriverResult};

async fn verify_scenario(scenario: FixtureScenario) -> WebDriverResult<()> {
    let server = FixtureServer::start(scenario).await.map_err(|error| {
        WebDriverError::RequestFailed(format!(
            "Fixture server could not start: {error}. Run cargo make build first"
        ))
    })?;
    let mut config = TestConfig::from_env();
    config.base_url = server.url().to_owned();
    harness::run_with_config(&format!("fixture-{scenario:?}"), config, move |driver, config| {
        Box::pin(async move {
            let page = PortfolioPage::new(driver, config);
            page.open("/").await?;
            match scenario {
                FixtureScenario::Retry | FixtureScenario::RetryTwice => {
                    let attempts = if scenario == FixtureScenario::RetryTwice { 2 } else { 1 };
                    for attempt in 0..attempts {
                        page.wait_for_present(selectors::CONTENT_ERROR).await?;
                        page.wait_for_present(selectors::CONTENT_RETRY).await?.click().await?;
                        if attempt + 1 < attempts {
                            page.wait_until_gone(selectors::CONTENT_ERROR).await?;
                            page.wait_for_present(selectors::CONTENT_ERROR).await?;
                        }
                    }
                    page.wait_for_present(selectors::APP_READY).await?;
                }
                FixtureScenario::Missing | FixtureScenario::Malformed | FixtureScenario::Invalid => {
                    page.wait_for_present(selectors::CONTENT_ERROR).await?;
                    page.wait_for_present(selectors::CONTENT_RETRY).await?;
                }
                FixtureScenario::Success | FixtureScenario::Delayed | FixtureScenario::ProjectsMissing | FixtureScenario::ProjectsMalformed | FixtureScenario::ProjectsRetry | FixtureScenario::ProjectsRetryTwice | FixtureScenario::ProjectsMany => {
                    page.wait_for_present(selectors::APP_READY).await?;
                    page.wait_until_gone(selectors::SPLASH).await?;
                }
            }
            Ok(())
        })
    }).await
}

#[tokio::test]
async fn success() -> WebDriverResult<()> { verify_scenario(FixtureScenario::Success).await }

#[tokio::test]
async fn missing() -> WebDriverResult<()> { verify_scenario(FixtureScenario::Missing).await }

#[tokio::test]
async fn malformed() -> WebDriverResult<()> { verify_scenario(FixtureScenario::Malformed).await }

#[tokio::test]
async fn invalid() -> WebDriverResult<()> { verify_scenario(FixtureScenario::Invalid).await }

#[tokio::test]
async fn delayed() -> WebDriverResult<()> { verify_scenario(FixtureScenario::Delayed).await }

#[tokio::test]
async fn retry() -> WebDriverResult<()> { verify_scenario(FixtureScenario::Retry).await }

async fn verify_projects(scenario: FixtureScenario) -> WebDriverResult<()> {
    let server = FixtureServer::start(scenario).await.map_err(|error| {
        WebDriverError::RequestFailed(format!(
            "Fixture server could not start: {error}. Run cargo make build first"
        ))
    })?;
    let mut config = TestConfig::from_env();
    config.base_url = server.url().to_owned();
    harness::run_with_config(&format!("project-fixture-{scenario:?}"), config, move |driver, config| {
        Box::pin(async move {
            let page = PortfolioPage::new(driver, config);
            page.open("/projects").await?;
            page.wait_for_present("[data-component='projects-error']").await?;
            page.wait_for_present("[data-component='content-recovery'][data-error-source='projects']").await?;
            page.wait_for_present(selectors::CONTENT_RETRY).await?;
            if matches!(scenario, FixtureScenario::ProjectsRetry | FixtureScenario::ProjectsRetryTwice) {
                let attempts = if scenario == FixtureScenario::ProjectsRetryTwice { 2 } else { 1 };
                for attempt in 0..attempts {
                    page.wait_for_present(selectors::CONTENT_RETRY).await?.click().await?;
                    if attempt + 1 < attempts {
                        page.wait_until_gone("[data-component='projects-error']").await?;
                        page.wait_for_present("[data-component='projects-error']").await?;
                    }
                }
                page.wait_for_present(selectors::PROJECT_CARDS).await?;
                page.wait_until_gone("[data-component='projects-error']").await?;
            }
            Ok(())
        })
    }).await
}

#[tokio::test]
async fn projects_missing() -> WebDriverResult<()> {
    verify_projects(FixtureScenario::ProjectsMissing).await
}

#[tokio::test]
async fn projects_malformed() -> WebDriverResult<()> {
    verify_projects(FixtureScenario::ProjectsMalformed).await
}

#[tokio::test]
async fn projects_retry() -> WebDriverResult<()> {
    verify_projects(FixtureScenario::ProjectsRetry).await
}

#[tokio::test]
async fn profile_retry_twice() -> WebDriverResult<()> {
    verify_scenario(FixtureScenario::RetryTwice).await
}

#[tokio::test]
async fn projects_retry_twice() -> WebDriverResult<()> {
    verify_projects(FixtureScenario::ProjectsRetryTwice).await
}
