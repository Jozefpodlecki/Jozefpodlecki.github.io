use portfolio_e2e::{harness, pages::portfolio::PortfolioPage};
use thirtyfour::{error::{WebDriverError, WebDriverResult}, prelude::*};

async fn active_element(driver: &WebDriver) -> WebDriverResult<String> {
    let result = driver.execute(
        "return document.activeElement?.getAttribute('data-component') || document.activeElement?.id || '';",
        vec![],
    ).await?;
    Ok(result.json().as_str().unwrap_or_default().to_owned())
}

#[tokio::test]
async fn skip_link_moves_keyboard_focus_to_main_content() -> WebDriverResult<()> {
    harness::run_named("keyboard-skip-link", |driver, config| Box::pin(async move {
        let page = PortfolioPage::new(driver, config);
        page.open("/").await?;
        page.wait_for_present("[data-component='skip-navigation']").await?;
        driver.action_chain().send_keys(Key::Tab).perform().await?;
        portfolio_e2e::ensure_eq!(active_element(driver).await?, "skip-navigation");
        driver.action_chain().send_keys(Key::Enter).perform().await?;
        let focused = driver.active_element().await?;
        portfolio_e2e::ensure_eq!(focused.attr("id").await?, Some("main-content".to_owned()));
        let location = driver.current_url().await?.to_string();
        portfolio_e2e::ensure!(location.ends_with("#/"), "Skip link changed the app route: {location}");
        Ok(())
    })).await
}

#[tokio::test]
async fn travel_dialog_traps_focus_and_restores_it_after_escape() -> WebDriverResult<()> {
    harness::run_named("travel-keyboard-focus", |driver, config| Box::pin(async move {
        let page = PortfolioPage::new(driver, config);
        page.open("/about").await?;
        page.wait_for_motion().await?;
        let expand = page.wait_for_present("[data-component='travel-expand']").await?;
        expand.scroll_into_view().await?;
        expand.send_keys(Key::Enter).await?;

        page.wait_for_present("[data-component='travel-map-dialog']").await?;
        portfolio_e2e::ensure_eq!(active_element(driver).await?, "travel-collapse");

        driver.action_chain().send_keys(Key::Tab).perform().await?;
        let focus_state = driver.execute(
            "return document.activeElement?.matches('[data-component=world-map] [data-visited=true][tabindex=\"0\"]');",
            vec![],
        ).await?;
        portfolio_e2e::ensure_eq!(focus_state.json().as_bool(), Some(true));

        driver.action_chain().send_keys(Key::Enter).perform().await?;
        page.wait_for_present("[data-component='travel-map-dialog'] [data-component='travel-tooltip']").await?;
        driver.action_chain().send_keys(Key::Escape).perform().await?;

        page.wait_until_gone("[data-component='travel-map-dialog']").await?;
        portfolio_e2e::ensure_eq!(active_element(driver).await?, "travel-expand");
        Ok(())
    })).await
}

#[tokio::test]
async fn photo_information_dismisses_on_outside_click_and_escape() -> WebDriverResult<()> {
    harness::run_named("photo-dismiss-journeys", |driver, config| Box::pin(async move {
        let page = PortfolioPage::new(driver, config);
        page.open("/").await?;
        let toggle = page.wait_for_present("[data-component='photo-info-toggle']").await?;
        toggle.click().await?;
        page.wait_for_present("[data-component='photo-viewer']").await?;
        page.wait_for_present("[data-component='photo-info-dismiss']").await?.click().await?;
        page.wait_until_gone("[data-component='photo-viewer']").await?;

        toggle.click().await?;
        page.wait_for_present("[data-component='photo-viewer']").await?;
        page.wait_for_present("[data-component='photo-viewer-close']").await?.send_keys(Key::Escape).await?;
        page.wait_until_gone("[data-component='photo-viewer']").await?;

        Ok(())
    })).await
}

#[tokio::test]
async fn reduced_motion_disables_reveals_and_decorative_motion() -> WebDriverResult<()> {
    harness::run_named("reduced-motion-browser", |driver, config| Box::pin(async move {
        driver.cdp().send_raw(
            "Emulation.setEmulatedMedia",
            serde_json::json!({
                "features": [{"name": "prefers-reduced-motion", "value": "reduce"}]
            }),
        ).await.map_err(|error| WebDriverError::RequestFailed(format!(
            "Could not emulate reduced motion: {error}"
        )))?;
        let page = PortfolioPage::new(driver, config);
        page.open("/").await?;
        page.wait_for_present("[data-component='home-hero']").await?;
        let state = driver.execute(
            "return { prefersReduced: matchMedia('(prefers-reduced-motion: reduce)').matches, revealVisible: [...document.querySelectorAll('[data-component=motion-reveal]')].every(e => parseFloat(e.style.opacity || '0') >= .999), glowHidden: getComputedStyle(document.querySelector('[data-component=pointer-glow]')).display === 'none' };",
            vec![],
        ).await?;
        let data = state.json();
        portfolio_e2e::ensure_eq!(data["prefersReduced"].as_bool(), Some(true));
        portfolio_e2e::ensure_eq!(data["revealVisible"].as_bool(), Some(true));
        portfolio_e2e::ensure_eq!(data["glowHidden"].as_bool(), Some(true));

        page.open("/projects").await?;
        page.wait_for_present("[data-component='project-card']").await?;
        page.wait_for_motion().await?;
        Ok(())
    })).await
}

#[tokio::test]
async fn theme_contrast_and_persistence_hold_across_routes() -> WebDriverResult<()> {
    harness::run_named("theme-all-routes", |driver, config| Box::pin(async move {
        let page = PortfolioPage::new(driver, config);
        page.open("/").await?;
        page.wait_for_present("[data-component='theme-toggle']").await?;
        for route in ["/", "/about", "/projects", "/projects/sudoku"] {
            page.open(route).await?;
            page.wait_for_present("[data-component='route-content']").await?;
            let before = driver.execute(
                "return { theme: document.documentElement.dataset.theme, text: getComputedStyle(document.body).color, background: getComputedStyle(document.body).backgroundColor };",
                vec![],
            ).await?.json().clone();

            page.wait_for_present("[data-component='theme-toggle']").await?.click().await?;
            let after = driver.execute(
                "return { theme: document.documentElement.dataset.theme, text: getComputedStyle(document.body).color, background: getComputedStyle(document.body).backgroundColor };",
                vec![],
            ).await?.json().clone();
            portfolio_e2e::ensure_ne!(before["theme"], after["theme"]);
            portfolio_e2e::ensure_ne!(before["text"], after["text"]);
            portfolio_e2e::ensure_ne!(before["background"], after["background"]);

            let contrast = driver.execute(
                "const channel = x => { const s = x / 255; return s <= .04045 ? s / 12.92 : ((s + .055) / 1.055) ** 2.4; }; const luminance = value => { const [r, g, b] = value.match(/[\\d.]+/g).slice(0, 3).map(Number); return .2126 * channel(r) + .7152 * channel(g) + .0722 * channel(b); }; const body = getComputedStyle(document.body); const light = Math.max(luminance(body.color), luminance(body.backgroundColor)); const dark = Math.min(luminance(body.color), luminance(body.backgroundColor)); return (light + .05) / (dark + .05);",
                vec![],
            ).await?;
            portfolio_e2e::ensure!(
                contrast.json().as_f64().unwrap_or(0.0) >= 4.5,
                "Body text contrast is below 4.5:1 on {route}: {}", contrast.json()
            );

            driver.refresh().await?;
            page.wait_for_present("[data-component='theme-toggle']").await?;
            let persisted = driver.execute(
                "return document.documentElement.dataset.theme;",
                vec![],
            ).await?;
            portfolio_e2e::ensure_eq!(persisted.json(), &after["theme"]);
        }
        Ok(())
    })).await
}

#[tokio::test]
async fn project_filters_and_case_studies_work_from_keyboard() -> WebDriverResult<()> {
    harness::run_named("projects-keyboard", |driver, config| Box::pin(async move {
        let page = PortfolioPage::new(driver, config);
        page.open("/projects").await?;
        let filter = page.wait_for_present("[data-component='project-filter'][data-filter-tag='Rust']").await?;
        filter.send_keys(Key::Enter).await?;
        let selected = driver.execute(
            "return document.querySelector('[data-component=project-filter][data-filter-tag=Rust]')?.getAttribute('aria-pressed');",
            vec![],
        ).await?;
        portfolio_e2e::ensure_eq!(selected.json().as_str(), Some("true"));

        let first = page.wait_for_present("[data-component='project-card-links'] a[href^='#/projects/']").await?;
        first.send_keys(Key::Enter).await?;
        page.wait_for_present("[data-component='project-case-study']").await?;
        Ok(())
    })).await
}
