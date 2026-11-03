use portfolio_e2e::{harness, pages::portfolio::PortfolioPage};
use thirtyfour::{error::{WebDriverError, WebDriverResult}, prelude::*};

#[tokio::test]
async fn hero_flow_field_draws_pixels_and_stays_decorative() -> WebDriverResult<()> {
    harness::run_named("hero-flow-field", |driver, config| Box::pin(async move {
        driver.set_window_rect(0, 0, 1440, 900).await?;
        let page = PortfolioPage::new(driver, config);
        page.open("/").await?;
        page.wait_for_present("[data-component='hero-flow-canvas']").await?;
        page.wait_for_motion().await?;

        let state = driver.execute(
            "const panel = document.querySelector('[data-component=hero-flow-field]');
             const canvas = document.querySelector('[data-component=hero-flow-canvas]');
             const context = canvas.getContext('2d');
             const sample = context.getImageData(0, 0, 1, 1).data;
             return {
                 hiddenFromAssistiveTechnology: panel.getAttribute('aria-hidden') === 'true',
                 hasDimensions: canvas.width > 0 && canvas.height > 0,
                 painted: sample[3] > 0,
                 interactive: getComputedStyle(canvas).pointerEvents !== 'none'
             };",
            vec![],
        ).await?;

        let data = state.json();
        portfolio_e2e::ensure_eq!(data["hiddenFromAssistiveTechnology"].as_bool(), Some(true));
        portfolio_e2e::ensure_eq!(data["hasDimensions"].as_bool(), Some(true));
        portfolio_e2e::ensure_eq!(data["painted"].as_bool(), Some(true));
        portfolio_e2e::ensure_eq!(data["interactive"].as_bool(), Some(true));
        Ok(())
    })).await
}

#[tokio::test]
async fn hero_flow_field_respects_reduced_motion() -> WebDriverResult<()> {
    harness::run_named("hero-flow-reduced-motion", |driver, config| Box::pin(async move {
        driver.set_window_rect(0, 0, 1440, 900).await?;
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
        let panel = page.wait_for_present("[data-component='hero-flow-field']").await?;
        portfolio_e2e::ensure_eq!(panel.attr("data-motion").await?, Some("static".to_owned()));
        Ok(())
    })).await
}

#[tokio::test]
async fn hero_flow_field_regenerates_seed_on_reload() -> WebDriverResult<()> {
    harness::run_named("hero-flow-seed", |driver, config| Box::pin(async move {
        driver.set_window_rect(0, 0, 1440, 900).await?;
        let page = PortfolioPage::new(driver, config);
        page.open("/").await?;

        let first = page
            .wait_for_present("[data-component='hero-flow-canvas']")
            .await?
            .attr("data-seed")
            .await?;

        driver.refresh().await?;

        let second = page
            .wait_for_present("[data-component='hero-flow-canvas']")
            .await?
            .attr("data-seed")
            .await?;

        portfolio_e2e::ensure!(first.is_some());
        portfolio_e2e::ensure!(second.is_some());
        portfolio_e2e::ensure_ne!(first, second);
        Ok(())
    })).await
}

#[tokio::test]
async fn hero_flow_information_opens_and_dismisses() -> WebDriverResult<()> {
    harness::run_named("hero-flow-information", |driver, config| Box::pin(async move {
        driver.set_window_rect(0, 0, 1440, 900).await?;
        let page = PortfolioPage::new(driver, config);
        page.open("/").await?;
        page.wait_for_motion().await?;

        let toggle = page
            .wait_for_present("[data-component='hero-flow-info-toggle']")
            .await?;
        portfolio_e2e::ensure_eq!(toggle.attr("aria-expanded").await?, Some("false".to_owned()));

        let layout_before = driver.execute(
            "const visual = document.querySelector('[data-component=hero-flow-visual]'); const rect = visual.getBoundingClientRect(); return { height: rect.height, bottom: rect.bottom };",
            vec![],
        ).await?.json().clone();

        toggle.click().await?;
        let details = page
            .wait_for_present("[data-component='hero-flow-info-details']")
            .await?;
        portfolio_e2e::ensure!(details.text().await?.contains("Canvas 2D"));
        portfolio_e2e::ensure_eq!(toggle.attr("aria-expanded").await?, Some("true".to_owned()));

        let layout_after = driver.execute(
            "const visual = document.querySelector('[data-component=hero-flow-visual]'); const panel = document.querySelector('[data-component=hero-flow-info-details]'); const trigger = document.querySelector('[data-component=hero-flow-info-toggle]'); const visualRect = visual.getBoundingClientRect(); const panelRect = panel.getBoundingClientRect(); const triggerRect = trigger.getBoundingClientRect(); return { height: visualRect.height, bottom: visualRect.bottom, floating: getComputedStyle(panel).position === 'absolute', above: panelRect.bottom <= triggerRect.top };",
            vec![],
        ).await?.json().clone();
        portfolio_e2e::ensure_eq!(layout_before["height"], layout_after["height"]);
        portfolio_e2e::ensure_eq!(layout_before["bottom"], layout_after["bottom"]);
        portfolio_e2e::ensure_eq!(layout_after["floating"].as_bool(), Some(true));
        portfolio_e2e::ensure_eq!(layout_after["above"].as_bool(), Some(true));

        page.wait_for_present("[data-component='hero-flow-info-dismiss']")
            .await?
            .click()
            .await?;
        page.wait_until_gone("[data-component='hero-flow-info-details']")
            .await?;

        toggle.click().await?;
        page.wait_for_present("[data-component='hero-flow-info-details']").await?;
        toggle.send_keys(Key::Escape).await?;
        page.wait_until_gone("[data-component='hero-flow-info-details']").await?;
        Ok(())
    })).await
}

#[tokio::test]
async fn hero_flow_modes_change_speed_without_resetting_seed() -> WebDriverResult<()> {
    harness::run_named("hero-flow-modes", |driver, config| Box::pin(async move {
        driver.set_window_rect(0, 0, 1440, 900).await?;
        let page = PortfolioPage::new(driver, config);
        page.open("/").await?;

        let canvas = page.wait_for_present("[data-component='hero-flow-canvas']").await?;
        let original_seed = canvas.attr("data-seed").await?;
        let panel = page.wait_for_present("[data-component='hero-flow-field']").await?;
        portfolio_e2e::ensure_eq!(panel.attr("data-flow-mode").await?, Some("calm".to_owned()));

        let icon_count = driver.execute(
            "return [...document.querySelectorAll('[data-component=hero-flow-mode]')].filter(button => button.querySelector('svg')).length;",
            vec![],
        ).await?;
        portfolio_e2e::ensure_eq!(icon_count.json().as_u64(), Some(2));

        let dynamic = page.wait_for_present("[data-component='hero-flow-mode'][data-mode='dynamic']").await?;
        dynamic.click().await?;
        portfolio_e2e::ensure_eq!(dynamic.attr("aria-pressed").await?, Some("true".to_owned()));
        portfolio_e2e::ensure_eq!(panel.attr("data-flow-mode").await?, Some("dynamic".to_owned()));
        portfolio_e2e::ensure_eq!(canvas.attr("data-seed").await?, original_seed);

        let calm = page.wait_for_present("[data-component='hero-flow-mode'][data-mode='calm']").await?;
        calm.click().await?;
        portfolio_e2e::ensure_eq!(calm.attr("aria-pressed").await?, Some("true".to_owned()));
        portfolio_e2e::ensure_eq!(panel.attr("data-flow-mode").await?, Some("calm".to_owned()));
        Ok(())
    })).await
}
