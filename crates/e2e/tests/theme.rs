use portfolio_e2e::{harness, pages::portfolio::PortfolioPage, selectors};
use thirtyfour::prelude::*;
#[tokio::test]
async fn theme_selection_persists_after_refresh() -> WebDriverResult<()> {
    harness::run_named("theme_selection_persists_after_refresh", |driver, config| Box::pin(async move {
        let page = PortfolioPage::new(driver, config);
        page.open("/").await?;
        let button = page.wait_for_present(selectors::THEME_TOGGLE).await?;
        button.click().await?;
        let before = driver.find(By::Css("html")).await?.attr("data-theme").await?;
        driver.refresh().await?;
        page.wait_for_present(selectors::APP_READY).await?;
        let after = driver.find(By::Css("html")).await?.attr("data-theme").await?;
        portfolio_e2e::ensure_eq!(before, after, "Stored theme should persist across refreshes");
        Ok(())
    })).await
}
