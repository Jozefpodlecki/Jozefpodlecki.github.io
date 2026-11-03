use portfolio_e2e::{harness, pages::portfolio::PortfolioPage};
use thirtyfour::prelude::*;

#[tokio::test]
async fn palette_override_keeps_colour_across_navigation_and_theme_changes() -> WebDriverResult<()> {
    harness::run_named("palette-override", |driver, config| {
        Box::pin(async move {
            driver.goto(format!("{}?palette=navy#/", config.base_url)).await?;
            let page = PortfolioPage::new(driver, config);
            page.wait_for_present("[data-component='home-hero']").await?;

            let root = driver.find(By::Css("html")).await?;
            portfolio_e2e::ensure_eq!(
                root.attr("data-palette").await?,
                Some("navy".to_owned())
            );

            let initial_theme = root.attr("data-theme").await?;
            let initial_accent = driver.execute(
                "return getComputedStyle(document.documentElement).getPropertyValue('--accent').trim();",
                vec![],
            ).await?;
            let initial_expected = if initial_theme.as_deref() == Some("dark") {
                "#79afff"
            } else {
                "#245db4"
            };
            portfolio_e2e::ensure_eq!(initial_accent.json().as_str(), Some(initial_expected));

            page.wait_for_present("[data-component='theme-toggle']").await?.click().await?;
            portfolio_e2e::ensure_eq!(
                root.attr("data-palette").await?,
                Some("navy".to_owned())
            );

            let current = root.attr("data-theme").await?;
            let updated = driver.execute(
                "return getComputedStyle(document.documentElement).getPropertyValue('--accent').trim();",
                vec![],
            ).await?;
            let expected = if current.as_deref() == Some("dark") {
                "#79afff"
            } else {
                "#245db4"
            };
            portfolio_e2e::ensure_eq!(updated.json().as_str(), Some(expected));

            driver.goto(format!("{}?palette=navy#/projects", config.base_url)).await?;
            page.wait_for_present("[data-component='projects-page']").await?;
            let root = driver.find(By::Css("html")).await?;
            portfolio_e2e::ensure_eq!(
                root.attr("data-palette").await?,
                Some("navy".to_owned())
            );
            Ok(())
        })
    }).await
}

#[tokio::test]
async fn connection_notice_handles_offline_and_recovery() -> WebDriverResult<()> {
    harness::run_named("connection-notice", |driver, config| {
        Box::pin(async move {
            let page = PortfolioPage::new(driver, config);
            page.open("/").await?;
            page.wait_for_present("[data-component='site-frame']").await?;

            driver.execute("window.dispatchEvent(new Event('offline'));", vec![]).await?;
            page.wait_for_present("[data-component='connection-notice']").await?;
            page.wait_for_present("[data-component='connection-dismiss']").await?.click().await?;
            page.wait_until_gone("[data-component='connection-notice']").await?;

            driver.execute("window.dispatchEvent(new Event('online'));", vec![]).await?;
            page.wait_for_present("[data-component='site-navigation']").await?;
            Ok(())
        })
    }).await
}
