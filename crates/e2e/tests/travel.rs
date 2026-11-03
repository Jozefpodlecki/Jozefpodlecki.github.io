use portfolio_e2e::{harness, pages::portfolio::PortfolioPage};
use thirtyfour::{error::WebDriverResult, extensions::query::ElementQueryable, By};

#[tokio::test]
async fn about_has_biography_and_world_map() -> WebDriverResult<()> {
    harness::run_named("about_has_biography_and_world_map", |driver, config| Box::pin(async move {
        let page = PortfolioPage::new(driver, config);
        page.open("/about").await?;
        let paragraphs = driver.query(By::Css("[data-component='about-biography'] p"))
            .wait(config.timeout, config.poll_interval)
            .any_required().await?;
        portfolio_e2e::ensure_eq!(paragraphs.len(), 2);
        let countries = driver.query(By::Css("[data-component='world-map'] [data-country]"))
            .wait(config.timeout, config.poll_interval)
            .any_required().await?;
        portfolio_e2e::ensure!(countries.len() > 100);
        let not_visited = driver.query(By::Css("[data-component='world-map'] [data-visited='false'][data-transit='false'][role='button']"))
            .nowait()
            .any()
            .await?;
        portfolio_e2e::ensure!(not_visited.is_empty());
        let controls = driver.query(By::Css("[data-component='travel-country-select'], [data-component^='map-zoom-']"))
            .nowait()
            .any()
            .await?;
        portfolio_e2e::ensure!(controls.is_empty());
        Ok(())
    })).await
}

#[tokio::test]
async fn photo_information_can_be_expanded() -> WebDriverResult<()> {
    harness::run_named("photo_information_can_be_expanded", |driver, config| Box::pin(async move {
        let page = PortfolioPage::new(driver, config);
        page.open("/").await?;
        let toggle = driver.query(By::Css("[data-component='photo-info-toggle']"))
            .wait(config.timeout, config.poll_interval)
            .first().await?;
        toggle.click().await?;
        driver.query(By::Css("[data-component='photo-details']"))
            .wait(config.timeout, config.poll_interval)
            .first().await?;
        Ok(())
    })).await
}

#[tokio::test]
async fn background_photo_identifies_ponta_do_rosto() -> WebDriverResult<()> {
    harness::run_named("photo-ponta-do-rosto", |driver, config| Box::pin(async move {
        let page = PortfolioPage::new(driver, config);
        page.open("/").await?;
        let toggle = page.wait_for_present("[data-component='photo-info-toggle']").await?;
        toggle.click().await?;
        page.wait_for_present("[data-component='photo-details']").await?;

        let deadline = tokio::time::Instant::now() + config.timeout;
        let text = loop {
            let result = driver
                .execute(
                    "return document.querySelector('[data-component=photo-details]')?.textContent || '';",
                    vec![],
                )
                .await?;
            let text = result.json().as_str().unwrap_or_default().to_owned();

            if text.contains("Miradouro da Ponta do Rosto") {
                break text;
            }

            if tokio::time::Instant::now() >= deadline {
                return Err(thirtyfour::error::WebDriverError::Timeout(format!(
                    "Photo caption was not rendered. Last content: {text:?}"
                )));
            }

            tokio::time::sleep(config.poll_interval).await;
        };

        portfolio_e2e::ensure!(text.contains("Caniçal, Machico"));
        portfolio_e2e::ensure!(text.contains("2026"));
        Ok(())
    })).await
}

#[tokio::test]
async fn visited_countries_and_fullscreen_map() -> WebDriverResult<()> {
    harness::run_named("visited-countries-map", |driver, config| {
        Box::pin(async move {
            let page = PortfolioPage::new(driver, config);
            page.open("/about").await?;

            for code in ["POL", "GBR", "CYP", "DEU", "HRV", "TUR", "PHL", "PRT", "EGY", "VEN", "AUS"] {
                page.wait_for_present(&format!(
                    "[data-component='travel-map'] [data-country='{code}'][role='button']"
                ))
                .await?;
            }

            page.wait_for_motion().await?;
            page.click_visible("[data-component='travel-expand']")
                .await?;

            let dialog = page
                .wait_for_present("[data-component='travel-map-dialog'][role='dialog']")
                .await?;
            portfolio_e2e::ensure_eq!(dialog.attr("aria-modal").await?, Some("true".to_owned()));
            let content = page
                .wait_for_present("[data-component='travel-dialog-content']")
                .await?;
            portfolio_e2e::ensure!(content.is_displayed().await?);
            let heading = driver
                .query(By::Css("[data-component='travel-map-dialog'] h2"))
                .nowait()
                .any()
                .await?;
            portfolio_e2e::ensure!(heading.is_empty());

            page.wait_for_present("[data-component='travel-map-dialog'] [data-country='ARE'][data-visited='false'][data-transit='true']").await?;

            let detail_panels = driver
                .query(By::Css("[data-component='travel-detail']"))
                .nowait()
                .any()
                .await?;
            portfolio_e2e::ensure!(detail_panels.is_empty());

            page.click_visible("[data-component='travel-collapse']")
                .await?;
            page.wait_until_gone("[data-component='travel-map-dialog']")
                .await?;
            Ok(())
        })
    })
    .await
}

#[tokio::test]
async fn poland_origin_and_dubai_visit_appear_in_tooltips() -> WebDriverResult<()> {
    harness::run_named("travel-poland-dubai-details", |driver, config| Box::pin(async move {
        let page = PortfolioPage::new(driver, config);
        page.open("/about").await?;

        for (code, expected) in [
            ("POL", "country of origin"),
            ("ARE", "Dubai International Airport"),
            ("PRT", "Miradouro da Ponta do Rosto"),
        ] {
            page.wait_for_present(&format!(
                "[data-component='travel-map'] [data-country='{code}'][role='button']"
            ))
            .await?;

            driver.execute(
                "const target = document.querySelector(arguments[0]); target.dispatchEvent(new MouseEvent('click', { bubbles: true, cancelable: true }));",
                vec![serde_json::json!(format!(
                    "[data-component='travel-map'] [data-country='{code}'][role='button']"
                ))],
            ).await?;

            let tooltip = page.wait_for_present("[data-component='travel-map'] [data-component='travel-tooltip']").await?;
            let text = tooltip.text().await?;
            portfolio_e2e::ensure!(
                text.contains(expected),
                "Tooltip for {code} should contain {expected}, got {text}"
            );
            if code == "ARE" {
                portfolio_e2e::ensure!(text.contains("2026"));
                portfolio_e2e::ensure!(text.contains("transit only"));
                page.wait_for_present("[data-component='travel-map'] [data-component='travel-visit'][data-kind='transit'] svg").await?;
            }
            if code == "POL" {
                portfolio_e2e::ensure!(text.contains("country of origin"));
            }
            if code == "PRT" {
                portfolio_e2e::ensure!(text.contains("2026"));
                portfolio_e2e::ensure!(text.contains("Machico"));
            }
        }
        Ok(())
    })).await
}

#[tokio::test]
async fn expanded_map_preserves_all_embedded_visit_details() -> WebDriverResult<()> {
    harness::run_named("travel-tooltip-parity", |driver, config| Box::pin(async move {
        let page = PortfolioPage::new(driver, config);
        page.open("/about").await?;
        page.wait_for_motion().await?;

        for code in ["DEU", "POL", "ARE"] {
            let embedded_selector = format!(
                "[data-component='travel-map'] [data-country='{code}'][role='button']"
            );
            page.wait_for_present(&embedded_selector).await?;
            driver.execute(
                "document.querySelector(arguments[0]).dispatchEvent(new MouseEvent('click', { bubbles: true, cancelable: true }));",
                vec![serde_json::json!(embedded_selector)],
            ).await?;

            let embedded_text = page
                .wait_for_present("[data-component='travel-map'] [data-component='travel-tooltip']")
                .await?
                .text()
                .await?;

            page.click_visible("[data-component='travel-expand']").await?;
            page.wait_for_present("[data-component='travel-map-dialog']").await?;

            let expanded_selector = format!(
                "[data-component='travel-map-dialog'] [data-country='{code}'][role='button']"
            );
            driver.execute(
                "document.querySelector(arguments[0]).dispatchEvent(new MouseEvent('click', { bubbles: true, cancelable: true }));",
                vec![serde_json::json!(expanded_selector)],
            ).await?;

            let expanded_text = page
                .wait_for_present("[data-component='travel-map-dialog'] [data-component='travel-tooltip']")
                .await?
                .text()
                .await?;
            portfolio_e2e::ensure_eq!(embedded_text, expanded_text);

            page.click_visible("[data-component='travel-collapse']").await?;
            page.wait_until_gone("[data-component='travel-map-dialog']").await?;
        }

        Ok(())
    })).await
}

#[tokio::test]
async fn germany_includes_a_visit_and_a_separate_airport_transit() -> WebDriverResult<()> {
    harness::run_named("travel-germany-transit", |driver, config| Box::pin(async move {
        let page = PortfolioPage::new(driver, config);
        page.open("/about").await?;
        page.wait_for_present("[data-component='travel-map'] [data-country='DEU'][data-visited='true'][data-transit='true']").await?;

        driver.execute(
            "document.querySelector('[data-component=travel-map] [data-country=DEU]').dispatchEvent(new MouseEvent('click', { bubbles: true, cancelable: true }));",
            vec![],
        ).await?;

        let tooltip = page.wait_for_present("[data-component='travel-map'] [data-component='travel-tooltip']").await?;
        let text = tooltip.text().await?;
        portfolio_e2e::ensure!(text.contains("Hamburg"));
        portfolio_e2e::ensure!(text.contains("Frankfurt Airport"));
        portfolio_e2e::ensure!(text.contains("transit only"));
        page.wait_for_present("[data-component='travel-map'] [data-component='travel-visit'][data-kind='transit'] svg").await?;
        Ok(())
    })).await
}
