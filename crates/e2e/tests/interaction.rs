use portfolio_e2e::{harness, pages::portfolio::PortfolioPage};
use thirtyfour::error::WebDriverResult;

#[tokio::test]
async fn development_context_menu_is_allowed_without_disabling_links() -> WebDriverResult<()> {
    harness::run_named("project-artwork-context-menu", |driver, config| Box::pin(async move {
        let page = PortfolioPage::new(driver, config);
        page.open("/projects/sudoku").await?;
        page.wait_for_present("[data-component='project-cover-image']").await?;
        page.wait_for_present("[data-component='project-detail-repository-link']").await?;

        let result = driver.execute(
            "const check = selector => { const element = document.querySelector(selector); const event = new MouseEvent('contextmenu', { bubbles: true, cancelable: true }); element.dispatchEvent(event); return event.defaultPrevented; }; return { artwork: check('[data-component=project-cover-image]'), repository: check('[data-component=project-detail-repository-link]'), overview: getComputedStyle(document.querySelector('[data-component=case-study-body]')).userSelect };",
            vec![],
        ).await?;
        let state = result.json();
        portfolio_e2e::ensure_eq!(
            page.wait_for_present("[data-component='site-frame']").await?.attr("data-context-menu-policy").await?,
            Some("enabled".to_owned())
        );
        portfolio_e2e::ensure_eq!(state["artwork"].as_bool(), Some(false), "Artwork context menu should be available in debug builds: {state}");
        portfolio_e2e::ensure_eq!(state["repository"].as_bool(), Some(false), "Repository link context menu should be available in debug builds: {state}");
        portfolio_e2e::ensure_ne!(state["overview"].as_str(), Some("none"), "Overview text should remain selectable: {state}");
        Ok(())
    })).await
}

#[tokio::test]
async fn travel_map_remains_interactive_with_nonselectable_illustration() -> WebDriverResult<()> {
    harness::run_named("travel-map-selection", |driver, config| Box::pin(async move {
        let page = PortfolioPage::new(driver, config);
        page.open("/about").await?;
        page.wait_for_present("[data-component='world-map']").await?;

        let result = driver.execute(
            "const map = document.querySelector('[data-component=world-map]'); const event = new MouseEvent('contextmenu', { bubbles: true, cancelable: true }); map.dispatchEvent(event); const country = map.querySelector('path[data-visited=true]'); return { contextMenuBlocked: event.defaultPrevented, mapUserSelect: getComputedStyle(map).userSelect, countryPointerEvents: getComputedStyle(country).pointerEvents };",
            vec![],
        ).await?;
        let state = result.json();
        portfolio_e2e::ensure_eq!(state["contextMenuBlocked"].as_bool(), Some(false), "Map context menu should be available in debug builds: {state}");
        portfolio_e2e::ensure_eq!(state["mapUserSelect"].as_str(), Some("none"), "Map should be nonselectable: {state}");
        portfolio_e2e::ensure_ne!(state["countryPointerEvents"].as_str(), Some("none"), "Visited country interaction should remain enabled: {state}");
        Ok(())
    })).await
}

#[tokio::test]
async fn presentation_copy_is_not_selectable_but_detailed_content_is() -> WebDriverResult<()> {
    harness::run_named("presentation-selection", |driver, config| Box::pin(async move {
        let page = PortfolioPage::new(driver, config);

        for (route, selector) in [
            ("/", "[data-component='home-hero']"),
            ("/about", "[data-component='about-intro']"),
            ("/projects", "[data-component='projects-page'] .page-heading"),
            ("/projects", "[data-component='project-card']"),
        ] {
            page.open(route).await?;
            page.wait_for_present(selector).await?;
            let script = format!(
                "return getComputedStyle(document.querySelector({selector:?})).userSelect;"
            );
            let result = driver.execute(&script, vec![]).await?;
            portfolio_e2e::ensure_eq!(
                result.json().as_str(),
                Some("none"),
                "Presentation should not allow text selection: {selector}"
            );
        }

        page.open("/projects/sudoku").await?;
        page.wait_for_present("[data-component='case-study-body']").await?;
        let result = driver.execute(
            "return getComputedStyle(document.querySelector('[data-component=case-study-body]')).userSelect;",
            vec![],
        ).await?;
        portfolio_e2e::ensure_ne!(result.json().as_str(), Some("none"));
        Ok(())
    })).await
}
