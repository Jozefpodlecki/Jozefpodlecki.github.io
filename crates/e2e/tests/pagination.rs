use portfolio_e2e::{
    config::TestConfig,
    fixtures::{FixtureScenario, FixtureServer},
    harness,
    pages::portfolio::PortfolioPage,
    selectors,
};
use thirtyfour::{error::{WebDriverError, WebDriverResult}, By};

#[tokio::test]
async fn pagination_covers_all_projects_and_resets_after_filter_change() -> WebDriverResult<()> {
    let server = FixtureServer::start(FixtureScenario::ProjectsMany).await.map_err(|error| {
        WebDriverError::RequestFailed(format!("Could not start project fixture: {error}"))
    })?;
    let mut config = TestConfig::from_env();
    config.base_url = server.url().to_owned();

    harness::run_with_config("projects-pagination-many", config, |driver, config| {
        Box::pin(async move {
            let page = PortfolioPage::new(driver, config);
            page.open("/projects").await?;
            page.wait_for_count(selectors::PROJECT_CARDS, 9).await?;
            page.wait_for_present("[data-component='project-pagination']").await?;
            let mut seen = std::collections::HashSet::new();

            for (number, expected) in [(1, 9), (2, 9), (3, 5)] {
                let active = page.wait_for_present(&format!(
                    "[data-component='project-page-number'][data-page='{number}'][aria-current='page']"
                )).await?;
                portfolio_e2e::ensure_eq!(active.attr("aria-current").await?, Some("page".to_owned()));
                page.wait_for_count(selectors::PROJECT_CARDS, expected).await?;
                let cards = driver.find_all(By::Css(selectors::PROJECT_CARDS)).await?;
                for card in cards {
                    let slug = card.attr("data-project-slug").await?.unwrap_or_default();
                    portfolio_e2e::ensure!(seen.insert(slug), "Project appeared on more than one page");
                }
                if number < 3 {
                    page.click_visible("[data-component='project-page-next']").await?;
                }
            }

            portfolio_e2e::ensure_eq!(seen.len(), 23);
            let next = page.wait_for_present("[data-component='project-page-next']").await?;
            portfolio_e2e::ensure!(!next.is_enabled().await?);

            page.click_visible("[data-component='project-filter'][data-filter-tag='Windows']").await?;
            page.wait_for_count(selectors::PROJECT_CARDS, 1).await?;
            let pagination = driver.find_all(By::Css("[data-component='project-pagination']")).await?;
            portfolio_e2e::ensure!(pagination.is_empty());

            page.click_visible("[data-component='project-filter'][data-filter-tag='All']").await?;
            page.wait_for_count(selectors::PROJECT_CARDS, 9).await?;
            let first = page.wait_for_present("[data-component='project-page-number'][data-page='1'][aria-current='page']").await?;
            portfolio_e2e::ensure_eq!(first.attr("aria-current").await?, Some("page".to_owned()));
            let previous = page.wait_for_present("[data-component='project-page-previous']").await?;
            portfolio_e2e::ensure!(!previous.is_enabled().await?);
            Ok(())
        })
    }).await
}
