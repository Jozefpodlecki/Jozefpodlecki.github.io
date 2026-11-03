use portfolio_e2e::{harness, pages::portfolio::PortfolioPage};
use thirtyfour::error::WebDriverResult;

#[tokio::test]
async fn home_route_renders_document() -> WebDriverResult<()> {
    harness::run_named("home_route_renders_document", |driver, config| Box::pin(async move {
        let page = PortfolioPage::new(driver, config);
        page.open("/").await?;
        page.wait_for_body().await?;
        let title = driver.title().await?;
        portfolio_e2e::ensure!(title.contains("Jozef"), "Unexpected title: {title}");
        Ok(())
    })).await
}

#[tokio::test]
async fn projects_route_has_heading_after_refresh() -> WebDriverResult<()> {
    harness::run_named("projects_route_has_heading_after_refresh", |driver, config| Box::pin(async move {
        let page = PortfolioPage::new(driver, config);
        page.open("/projects").await?;
        page.wait_for_heading("Projects").await?;
        driver.refresh().await?;
        page.wait_for_heading("Projects").await?;
        Ok(())
    })).await
}

#[tokio::test]
async fn home_projects_action_has_readable_label() -> WebDriverResult<()> {
    harness::run_named("home_projects_action_has_readable_label", |driver, config| Box::pin(async move {
        let page = PortfolioPage::new(driver, config);
        page.open("/").await?;
        page.wait_for_present("[data-component='home-hero'] a[href='#/projects']").await?;

        let result = driver.execute(
            "const link = document.querySelector('[data-component=home-hero] a[href=\"#/projects\"]'); const sample = document.createElement('span'); sample.style.color = 'var(--accent-contrast)'; document.body.appendChild(sample); const expected = getComputedStyle(sample).color; sample.remove(); return { label: link.textContent.trim(), icon: !!link.querySelector('svg'), actual: getComputedStyle(link).color, expected };",
            vec![],
        ).await?;
        let state = result.json();
        portfolio_e2e::ensure!(state["label"].as_str().is_some_and(|label| label.contains("View my projects")), "Missing hero action label: {state}");
        portfolio_e2e::ensure_eq!(state["icon"].as_bool(), Some(true), "Missing hero action icon: {state}");
        portfolio_e2e::ensure_eq!(state["actual"], state["expected"], "Hero action contrast colour does not match the theme: {state}");
        Ok(())
    })).await
}
