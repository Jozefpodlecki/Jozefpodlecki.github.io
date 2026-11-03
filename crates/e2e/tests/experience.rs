use portfolio_e2e::{harness, pages::portfolio::PortfolioPage, selectors};
use thirtyfour::error::WebDriverResult;

#[tokio::test]
async fn about_experience_has_entries_and_portrait() -> WebDriverResult<()> {
    harness::run_named("about_experience_has_entries_and_portrait", |driver, config| Box::pin(async move {
        let page = PortfolioPage::new(driver, config);
        page.open("/about").await?;
        page.wait_for_present(selectors::APP_READY).await?;
        page.wait_for_present(selectors::ABOUT_PORTRAIT).await?;
        page.wait_for_present(selectors::EXPERIENCE_SECTION).await?;
        let education = page.wait_for_present("[data-component='education-section']").await?;
        let education_text = education.text().await?;
        portfolio_e2e::ensure!(education_text.contains("University of Warmia and Mazury"));
        portfolio_e2e::ensure!(education_text.contains("Engineering of IT Systems"));
        portfolio_e2e::ensure!(education_text.contains("2015 - 2019"));
        let university = page.wait_for_present("[data-component='education-institution-link']").await?;
        portfolio_e2e::ensure_eq!(university.attr("href").await?, Some("https://uwm.edu.pl/".to_owned()));
        let logo = page.wait_for_present("[data-component='education-logo'] img").await?;
        portfolio_e2e::ensure_eq!(logo.attr("alt").await?, Some("University of Warmia and Mazury in Olsztyn logo".to_owned()));
        logo.scroll_into_view().await?;
        let decoded = tokio::time::timeout(
            std::time::Duration::from_secs(10),
            async {
                loop {
                    let result = driver.execute(
                        "const image = document.querySelector('[data-component=education-logo] img'); return { loaded: image.complete && image.naturalWidth > 0, failed: image.complete && image.naturalWidth === 0, url: image.currentSrc };",
                        vec![],
                    ).await?;
                    let state = result.json();
                    if state["loaded"].as_bool() == Some(true) || state["failed"].as_bool() == Some(true) {
                        return Ok::<_, thirtyfour::error::WebDriverError>(state.clone());
                    }
                    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                }
            },
        ).await.map_err(|error| thirtyfour::error::WebDriverError::RequestFailed(
            format!("UWM logo did not finish loading: {error}")
        ))??;
        portfolio_e2e::ensure!(
            decoded["loaded"].as_bool() == Some(true),
            "UWM logo failed to load: {decoded}"
        );
        let entries = page.experience_entries().await?;
        portfolio_e2e::ensure!(!entries.is_empty());
        for entry in &entries {
            portfolio_e2e::ensure!(!entry.attr("data-experience-index").await?.unwrap_or_default().is_empty());
        }
        Ok(())
    })).await
}

#[tokio::test]
async fn biography_and_recent_experience_use_real_content() -> WebDriverResult<()> {
    harness::run_named("biography_and_recent_experience_use_real_content", |driver, config| Box::pin(async move {
        use thirtyfour::By;

        let page = PortfolioPage::new(driver, config);
        page.open("/about").await?;
        let biography = page.wait_for_present("[data-component='about-biography']").await?;
        let text = biography.text().await?;
        portfolio_e2e::ensure!(text.contains("software engineer"));
        portfolio_e2e::ensure!(text.contains("Rust"));
        portfolio_e2e::ensure!(!text.to_lowercase().contains("lorem ipsum"));

        let entries = page.experience_entries().await?;
        portfolio_e2e::ensure!(entries.len() >= 3);
        let recent = &entries[0];
        portfolio_e2e::ensure!(recent.text().await?.contains("Sep 2025 - Present"));
        let responsibilities = recent.find_all(By::Css("[data-component='experience-responsibilities'] li")).await?;
        portfolio_e2e::ensure!(responsibilities.len() >= 4);
        portfolio_e2e::ensure!(recent.text().await?.contains("SQL Server"));
        Ok(())
    })).await
}

#[tokio::test]
async fn experience_cards_are_nonselectable_with_debug_context_menu_allowed() -> WebDriverResult<()> {
    harness::run_named("experience-selection-and-context-menu", |driver, config| Box::pin(async move {
        let page = PortfolioPage::new(driver, config);
        page.open("/about").await?;
        page.wait_for_present("[data-component='experience-entry']").await?;

        let result = driver.execute(
            "const entry = document.querySelector('[data-component=experience-entry]'); const text = entry.querySelector('[data-component=experience-responsibilities]'); const event = new MouseEvent('contextmenu', { bubbles: true, cancelable: true }); text.dispatchEvent(event); return { cardSelection: getComputedStyle(entry).userSelect, contextMenuBlocked: event.defaultPrevented };",
            vec![],
        ).await?;
        let state = result.json();
        portfolio_e2e::ensure_eq!(state["cardSelection"].as_str(), Some("none"));
        portfolio_e2e::ensure_eq!(state["contextMenuBlocked"].as_bool(), Some(false));
        Ok(())
    })).await
}

#[tokio::test]
async fn education_logo_falls_back_when_image_fails() -> WebDriverResult<()> {
    harness::run_named("education-logo-fallback", |driver, config| {
        Box::pin(async move {
            let page = PortfolioPage::new(driver, config);
            page.open("/about").await?;
            page.wait_for_present("[data-component='education-logo'] img")
                .await?;

            driver
                .execute(
                    "document.querySelector('[data-component=education-logo] img').src = '/images/absent-education-logo.png';",
                    vec![],
                )
                .await?;

            let fallback = page
                .wait_for_present("[data-component='education-logo-fallback']")
                .await?;

            portfolio_e2e::ensure_eq!(
                fallback.attr("role").await?,
                Some("img".to_owned())
            );
            portfolio_e2e::ensure!(
                fallback.attr("aria-label").await?.unwrap_or_default().contains("University of Warmia and Mazury")
            );
            portfolio_e2e::ensure!(
                page.wait_for_present("[data-component='education-institution-link']")
                    .await?
                    .is_displayed()
                    .await?
            );
            Ok(())
        })
    })
    .await
}
