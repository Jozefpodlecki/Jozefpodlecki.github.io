use portfolio_e2e::{harness, pages::portfolio::PortfolioPage, selectors};
use thirtyfour::error::WebDriverResult;

#[tokio::test]
async fn social_links_are_exposed() -> WebDriverResult<()> {
    harness::run_named("social_links_are_exposed", |driver, config| Box::pin(async move {
        let page = PortfolioPage::new(driver, config);
        page.open("/").await?;
        let github = page.wait_for_present(selectors::GITHUB_LINK).await?;
        let linkedin = page.wait_for_present(selectors::LINKEDIN_LINK).await?;
        portfolio_e2e::ensure!(github.attr("href").await?.unwrap_or_default().starts_with("https://"));
        portfolio_e2e::ensure!(linkedin.attr("href").await?.unwrap_or_default().starts_with("https://"));

        let cv = page.wait_for_present("[data-social='cv']").await?;
        portfolio_e2e::ensure!(cv.attr("href").await?.unwrap_or_default().ends_with(".pdf"));
        let badge = page.wait_for_present("[data-social='cv'] span[aria-hidden='true']").await?;
        portfolio_e2e::ensure_eq!(badge.text().await?, "CV");
        page.wait_for_present("[data-social='cv'] svg").await?;

        let copyright = page.wait_for_present("[data-component='copyright']").await?;
        portfolio_e2e::ensure!(copyright.text().await?.contains("Józef Podlecki"));
        let selection = driver.execute(
            "return getComputedStyle(document.querySelector('[data-component=copyright]')).userSelect;",
            vec![],
        ).await?;
        portfolio_e2e::ensure_eq!(selection.json().as_str(), Some("none"));
        Ok(())
    })).await
}

#[tokio::test]
async fn browser_icons_are_declared() -> WebDriverResult<()> {
    harness::run_named("browser_icons_are_declared", |driver, config| Box::pin(async move {
        use thirtyfour::By;

        let page = PortfolioPage::new(driver, config);
        page.open("/").await?;
        let favicon = driver.find(By::Css("link[rel='icon'][type='image/svg+xml']")).await?;
        portfolio_e2e::ensure!(favicon.attr("href").await?.unwrap_or_default().ends_with("icons/favicon.svg"));
        let apple_icon = driver.find(By::Css("link[rel='apple-touch-icon']")).await?;
        portfolio_e2e::ensure!(apple_icon.attr("href").await?.unwrap_or_default().ends_with("icons/apple-touch-icon.png"));
        Ok(())
    })).await
}

#[tokio::test]
async fn home_metadata_is_declared() -> WebDriverResult<()> {
    harness::run_named("home-seo-metadata", |driver, config| Box::pin(async move {
        use thirtyfour::By;
        let page = PortfolioPage::new(driver, config);
        page.open("/").await?;
        let description = driver.find(By::Css("meta[name='description']")).await?;
        portfolio_e2e::ensure!(description.attr("content").await?.unwrap_or_default().contains("software engineer"));
        let canonical = driver.find(By::Css("link[rel='canonical']")).await?;
        portfolio_e2e::ensure_eq!(canonical.attr("href").await?, Some("https://jozefpodlecki.github.io/".to_owned()));
        page.wait_for_present("meta[property='og:title']").await?;
        page.wait_for_present("meta[property='og:image']").await?;
        Ok(())
    })).await
}
