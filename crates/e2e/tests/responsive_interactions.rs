use portfolio_e2e::{harness, pages::portfolio::PortfolioPage};
use thirtyfour::{error::WebDriverResult, prelude::*};

#[tokio::test]
async fn narrow_screen_navigation_filters_and_map_remain_operable() -> WebDriverResult<()> {
    harness::run_named("mobile-interactions", |driver, config| Box::pin(async move {
        driver.set_window_rect(0, 0, 320, 700).await?;
        let page = PortfolioPage::new(driver, config);
        page.open("/").await?;
        page.wait_for_motion().await?;

        page.click_visible("[data-component='site-navigation'] a[href='#/projects']").await?;
        page.wait_for_present("[data-component='projects-page']").await?;
        let buttons = driver.find_all(By::Css("[data-component='project-filter']")).await?;
        portfolio_e2e::ensure!(!buttons.is_empty(), "No mobile project filters found");

        let screen = driver.execute(
            "return { overflow: document.documentElement.scrollWidth > innerWidth + 1, cards: document.querySelectorAll('[data-component=project-card]').length };",
            vec![],
        ).await?;
        portfolio_e2e::ensure_eq!(screen.json()["overflow"].as_bool(), Some(false));
        portfolio_e2e::ensure!(screen.json()["cards"].as_u64().unwrap_or(0) >= 3);

        page.click_visible("[data-component='site-navigation'] a[href='#/about']").await?;
        page.wait_for_present("[data-component='travel-expand']").await?;
        page.wait_for_motion().await?;
        page.click_visible("[data-component='travel-expand']").await?;
        page.wait_for_present("[data-component='travel-map-dialog']").await?;
        page.click_visible("[data-component='travel-collapse']").await?;
        page.wait_until_gone("[data-component='travel-map-dialog']").await?;
        Ok(())
    })).await
}

#[tokio::test]
async fn external_links_have_safe_targets_and_usable_labels() -> WebDriverResult<()> {
    harness::run_named("safe-links", |driver, config| Box::pin(async move {
        let page = PortfolioPage::new(driver, config);
        page.open("/projects").await?;
        page.wait_for_present("[data-component='project-card']").await?;
        let result = driver.execute(
            "return [...document.querySelectorAll('a[target=_blank]')].map(a => ({ href: a.href, local: new URL(a.href).origin === location.origin, safe: a.relList.contains('noopener') && a.relList.contains('noreferrer'), label: a.getAttribute('aria-label') || a.textContent.trim() || a.title }));",
            vec![],
        ).await?;
        for link in result.json().as_array().into_iter().flatten() {
            if link["local"].as_bool() == Some(false) {
                portfolio_e2e::ensure!(link["href"].as_str().is_some_and(|href| href.starts_with("https://")), "External link must use HTTPS: {link}");
            }
            portfolio_e2e::ensure_eq!(link["safe"].as_bool(), Some(true), "External link is missing rel safeguards: {link}");
            portfolio_e2e::ensure!(link["label"].as_str().is_some_and(|label| !label.is_empty()), "External link lacks an accessible label: {link}");
        }
        Ok(())
    })).await
}
