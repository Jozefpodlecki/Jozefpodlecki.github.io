use portfolio_e2e::{harness, pages::portfolio::PortfolioPage, selectors};
use thirtyfour::error::WebDriverResult;

#[tokio::test]
async fn ready_state_replaces_splash() -> WebDriverResult<()> {
    harness::run_named("ready_state_replaces_splash", |driver, config| Box::pin(async move {
        let page = PortfolioPage::new(driver, config);
        page.open("/").await?;
        page.wait_for_present(selectors::APP_READY).await?;
        page.wait_until_gone(selectors::SPLASH).await?;
        Ok(())
    })).await
}
#[tokio::test]
async fn about_route_shows_experience() -> WebDriverResult<()> {
    harness::run_named("about_route_shows_experience", |driver, config| Box::pin(async move {
        let page = PortfolioPage::new(driver, config);
        page.open("/about").await?;
        page.wait_for_heading("A bit about me.").await?;
        page.wait_for_present(selectors::EXPERIENCE_SECTION).await?;
        Ok(())
    })).await
}

#[tokio::test]
async fn three_projects_are_visible() -> WebDriverResult<()> {
    harness::run_named("three_projects_are_visible", |driver, config| {
        Box::pin(async move {
            let page = PortfolioPage::new(driver, config);
            page.open("/projects").await?;

            page.wait_for_count(selectors::PROJECT_CARDS, 3).await?;

            Ok(())
        })
    })
    .await
}
