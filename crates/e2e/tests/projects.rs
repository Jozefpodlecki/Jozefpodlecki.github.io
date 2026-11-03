use portfolio_e2e::{harness, pages::portfolio::PortfolioPage, selectors};
use thirtyfour::{error::WebDriverResult, By};

#[tokio::test]
async fn technology_filter_updates_project_cards() -> WebDriverResult<()> {
    harness::run_named("technology_filter_updates_project_cards", |driver, config| Box::pin(async move {
        let page = PortfolioPage::new(driver, config);
        page.open("/projects").await?;
        page.wait_for_present(selectors::PROJECT_FILTERS).await?;
        page.wait_for_present(selectors::PROJECT_GRID).await?;
        page.wait_for_count(selectors::PROJECT_CARDS, 3).await?;
        let all_filter = driver.find(By::Css("[data-component='project-filter']")).await?;
        portfolio_e2e::ensure_eq!(all_filter.text().await?, "All");
        driver.find(By::Css("[data-filter-tag='Windows']")).await?.click().await?;
        page.wait_for_count(selectors::PROJECT_CARDS, 1).await?;
        driver.find(By::Css("[data-filter-tag='All']")).await?.click().await?;
        page.wait_for_count(selectors::PROJECT_CARDS, 3).await?;
        Ok(())
    })).await
}

#[tokio::test]
async fn project_detail_renders_for_sudoku_slug() -> WebDriverResult<()> {
    harness::run_named("project_detail_renders_for_sudoku_slug", |driver, config| Box::pin(async move {
        let page = PortfolioPage::new(driver, config);
        page.open("/projects/sudoku").await?;
        page.wait_for_heading("Sudoku").await?;
        page.wait_for_present("[data-component='case-study-body']").await?;
        Ok(())
    })).await
}

#[tokio::test]
async fn case_study_has_structured_sections() -> WebDriverResult<()> {
    harness::run_named("case_study_has_structured_sections", |driver, config| Box::pin(async move {
        let page = PortfolioPage::new(driver, config);
        page.open("/projects/sudoku").await?;
        page.wait_for_present("[data-component='project-heading']").await?;
        page.wait_for_present("[data-component='project-cover']").await?;
        page.wait_for_present("[data-component='case-study-body']").await?;
        page.wait_for_present("[data-component='project-actions']").await?;
        let paragraphs = driver.find_all(By::Css("[data-component='case-study-body'] p")).await?;
        portfolio_e2e::ensure_eq!(paragraphs.len(), 3);
        Ok(())
    })).await
}

#[tokio::test]
async fn project_cards_link_to_real_repositories() -> WebDriverResult<()> {
    harness::run_named("project_cards_link_to_real_repositories", |driver, config| Box::pin(async move {
        let page = PortfolioPage::new(driver, config);
        page.open("/projects").await?;
        page.wait_for_count(selectors::PROJECT_CARDS, 3).await?;

        for slug in ["sudoku", "win-drives", "zipeek"] {
            let selector = format!(
                "[data-component='project-card'][data-project-slug='{slug}'] [data-component='project-repository-link']"
            );
            let repository = page.wait_for_present(&selector).await?;
            let url = repository.attr("href").await?.unwrap_or_default();
            portfolio_e2e::ensure_eq!(url, format!("https://github.com/Jozefpodlecki/{}", if slug == "sudoku" { "Sudoku" } else { slug }));
        }

        let live = driver.find_all(By::Css("[data-component='project-live-link']")).await?;
        portfolio_e2e::ensure_eq!(live.len(), 1);
        portfolio_e2e::ensure_eq!(live[0].attr("href").await?, Some("https://jozefpodlecki.github.io/Sudoku/".to_owned()));
        Ok(())
    })).await
}

#[tokio::test]
async fn project_motion_settles_before_visual_review() -> WebDriverResult<()> {
    harness::run_named("project-motion", |driver, config| Box::pin(async move {
        let page = PortfolioPage::new(driver, config);
        page.open("/projects").await?;
        page.wait_for_count(selectors::PROJECT_CARDS, 3).await?;
        page.wait_for_present("[data-component='motion-reveal']").await?;
        page.wait_for_motion().await?;
        Ok(())
    })).await
}

#[tokio::test]
async fn navigating_from_gallery_to_case_study_keeps_project_content() -> WebDriverResult<()> {
    harness::run_named("project-route", |driver, config| Box::pin(async move {
        let page = PortfolioPage::new(driver, config);
        page.open("/projects").await?;
        page.wait_for_count(selectors::PROJECT_CARDS, 3).await?;
        driver.find(By::Css("[data-project-slug='sudoku'] [href*='sudoku']")).await?.click().await?;
        page.wait_for_present("[data-component='project-case-study'][data-project-slug='sudoku']").await?;
        page.wait_for_motion().await?;
        Ok(())
    })).await
}

#[tokio::test]
async fn each_project_has_a_local_cover_image() -> WebDriverResult<()> {
    harness::run_named("project-cover-images", |driver, config| Box::pin(async move {
        let page = PortfolioPage::new(driver, config);
        page.open("/projects").await?;
        page.wait_for_count(selectors::PROJECT_CARDS, 3).await?;
        page.wait_for_images().await?;

        for slug in ["sudoku", "win-drives", "zipeek"] {
            let artwork = page.wait_for_present(&format!(
                "[data-component='project-card'][data-project-slug='{slug}'] [data-component='project-artwork']"
            )).await?;
            portfolio_e2e::ensure_eq!(
                artwork.attr("data-image-status").await?,
                Some("ready".to_owned()),
                "Expected a loaded image for {slug}"
            );
            let selector = format!("[data-component='project-card'][data-project-slug='{slug}'] [data-component='project-card-image']");
            let image = page.wait_for_present(&selector).await?;
            portfolio_e2e::ensure!(image.attr("src").await?.unwrap_or_default().contains(&format!("{slug}.webp")));
        }

        page.open("/projects/sudoku").await?;
        page.wait_for_present("[data-component='project-cover-image']").await?;
        page.wait_for_images().await?;
        let cover = page.wait_for_present("[data-component='project-cover'] [data-component='project-artwork']").await?;
        portfolio_e2e::ensure_eq!(cover.attr("data-image-status").await?, Some("ready".to_owned()));
        Ok(())
    })).await
}

#[tokio::test]
async fn missing_project_image_shows_accessible_fallback() -> WebDriverResult<()> {
    harness::run_named("project-image-fallback", |driver, config| Box::pin(async move {
        let page = PortfolioPage::new(driver, config);
        page.open("/projects").await?;
        page.wait_for_count(selectors::PROJECT_CARDS, 3).await?;
        page.wait_for_present("[data-project-slug='sudoku'] [data-component='project-card-image']").await?;
        driver.execute(
            "document.querySelector('[data-project-slug=sudoku] [data-component=project-card-image]').src = 'images/does-not-exist-e2e.webp';",
            vec![],
        ).await?;
        let fallback = page.wait_for_present(
            "[data-project-slug='sudoku'] [data-component='project-artwork'][data-image-status='unavailable'] [data-component='project-artwork-fallback']"
        ).await?;
        portfolio_e2e::ensure_eq!(
            fallback.attr("aria-label").await?,
            Some("Preview unavailable for Sudoku".to_owned())
        );
        portfolio_e2e::ensure!(fallback.is_displayed().await?);
        let broken = driver.find_all(By::Css("[data-project-slug='sudoku'] img[data-component='project-card-image']")).await?;
        portfolio_e2e::ensure!(broken.is_empty(), "Broken image elements should not remain visible");
        Ok(())
    })).await
}

#[tokio::test]
async fn case_study_live_action_has_visible_text_and_icon() -> WebDriverResult<()> {
    harness::run_named("project-live-action", |driver, config| Box::pin(async move {
        let page = PortfolioPage::new(driver, config);
        page.open("/projects/sudoku").await?;
        let action = page.wait_for_present("[data-component='project-detail-live-link']").await?;
        portfolio_e2e::ensure!(action.is_displayed().await?);
        portfolio_e2e::ensure_eq!(action.text().await?, "Live site");
        portfolio_e2e::ensure_eq!(
            action.attr("href").await?, Some("https://jozefpodlecki.github.io/Sudoku/".to_owned())
        );
        portfolio_e2e::ensure!(action.find(By::Css("svg")).await?.is_displayed().await?);
        Ok(())
    })).await
}

#[tokio::test]
async fn unknown_project_slug_shows_friendly_not_found() -> WebDriverResult<()> {
    harness::run_named("unknown-project", |driver, config| Box::pin(async move {
        let page = PortfolioPage::new(driver, config);
        page.open("/projects/does-not-exist").await?;
        page.wait_for_present("[data-component='project-not-found']").await?;
        let studies = driver.find_all(By::Css("[data-component='project-case-study']")).await?;
        portfolio_e2e::ensure!(studies.is_empty());
        Ok(())
    })).await
}

#[tokio::test]
async fn each_case_study_exposes_repo_but_only_sudoku_exposes_live_site() -> WebDriverResult<()> {
    harness::run_named("case-study-actions", |driver, config| Box::pin(async move {
        let page = PortfolioPage::new(driver, config);
        for slug in ["sudoku", "win-drives", "zipeek"] {
            page.open(&format!("/projects/{slug}")).await?;
            page.wait_for_present(&format!(
                "[data-component='project-case-study'][data-project-slug='{slug}']"
            )).await?;
            page.wait_for_present("[data-component='project-actions']").await?;
            let repo = page.wait_for_present("[data-component='project-detail-repository-link']").await?;
            portfolio_e2e::ensure!(repo.attr("href").await?.unwrap_or_default().starts_with("https://github.com/Jozefpodlecki/"));
            let live = driver.find_all(By::Css("[data-component='project-detail-live-link']")).await?;
            portfolio_e2e::ensure_eq!(live.len(), usize::from(slug == "sudoku"), "Unexpected live link on {slug}");
        }
        Ok(())
    })).await
}

#[tokio::test]
async fn small_gallery_does_not_show_pagination() -> WebDriverResult<()> {
    harness::run_named("projects-pagination-small", |driver, config| Box::pin(async move {
        let page = PortfolioPage::new(driver, config);
        page.open("/projects").await?;
        page.wait_for_count(selectors::PROJECT_CARDS, 3).await?;
        let pagination = driver.find_all(By::Css("[data-component='project-pagination']")).await?;
        portfolio_e2e::ensure!(pagination.is_empty());
        Ok(())
    })).await
}

#[tokio::test]
async fn footer_links_to_portfolio_repository() -> WebDriverResult<()> {
    harness::run_named("portfolio-source-footer", |driver, config| Box::pin(async move {
        let page = PortfolioPage::new(driver, config);
        page.open("/").await?;
        let link = page.wait_for_present("[data-component='portfolio-source-link']").await?;
        portfolio_e2e::ensure_eq!(
            link.attr("href").await?,
            Some("https://github.com/Jozefpodlecki/Jozefpodlecki.github.io".to_owned())
        );
        portfolio_e2e::ensure_eq!(link.attr("aria-label").await?, Some("Portfolio source code on GitHub".to_owned()));
        portfolio_e2e::ensure!(link.find(By::Css("svg")).await?.is_displayed().await?);
        Ok(())
    })).await
}

#[tokio::test]
async fn projects_respect_priority_in_gallery_and_featured() -> WebDriverResult<()> {
    harness::run_named("projects-priority", |driver, config| Box::pin(async move {
        let page = PortfolioPage::new(driver, config);
        page.open("/projects").await?;
        page.wait_for_count(selectors::PROJECT_CARDS, 3).await?;
        let list = driver.find_all(By::Css("[data-component='project-card']")).await?;
        let mut slugs = Vec::new();
        for card in list {
            slugs.push(card.attr("data-project-slug").await?.unwrap_or_default());
        }
        portfolio_e2e::ensure_eq!(slugs, vec!["sudoku", "win-drives", "zipeek"]);
        page.open("/").await?;
        page.wait_for_count(selectors::PROJECT_CARDS, 3).await?;
        let featured = driver.find_all(By::Css("[data-component='project-card']")).await?;
        let mut slugs = Vec::new();
        for card in featured {
            slugs.push(card.attr("data-project-slug").await?.unwrap_or_default());
        }
        portfolio_e2e::ensure_eq!(slugs, vec!["sudoku", "win-drives", "zipeek"]);
        Ok(())
    })).await
}
