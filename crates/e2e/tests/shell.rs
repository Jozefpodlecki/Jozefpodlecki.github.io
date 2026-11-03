use portfolio_e2e::{harness, pages::portfolio::PortfolioPage, selectors};
use thirtyfour::{error::WebDriverResult, By};

#[tokio::test]
async fn navigation_marks_current_route() -> WebDriverResult<()> {
    harness::run_named("navigation-current-route", |driver, config| {
        Box::pin(async move {
            let page = PortfolioPage::new(driver, config);

            for (route, selected) in [
                ("/", "home"),
                ("/about", "about"),
                ("/projects", "projects"),
                ("/projects/sudoku", "projects"),
            ] {
                page.open(route).await?;
                page.wait_for_present(selectors::APP_READY).await?;

                let active = page.wait_for_present(
                    &format!("[data-component='navigation-item'][data-route='{selected}'][data-active='true']"),
                ).await?;

                portfolio_e2e::ensure_eq!(active.attr("aria-current").await?, Some("page".to_owned()));

                let others = driver.find_all(By::Css(
                    "[data-component='navigation-item'][data-active='true']"
                )).await?;

                portfolio_e2e::ensure_eq!(others.len(), 1, "Expected one active item on {route}");
            }

            Ok(())
        })
    }).await
}

#[tokio::test]
async fn photo_disclosure_updates_accessible_state() -> WebDriverResult<()> {
    harness::run_named("photo-disclosure", |driver, config| {
        Box::pin(async move {
            let page = PortfolioPage::new(driver, config);
            page.open("/").await?;
            page.wait_for_present(selectors::APP_READY).await?;

            let button = page.wait_for_present("[data-component='photo-info-toggle']").await?;
            portfolio_e2e::ensure_eq!(button.text().await?, "About this place");
            portfolio_e2e::ensure_eq!(button.attr("aria-expanded").await?, Some("false".to_owned()));

            button.click().await?;
            page.wait_for_present("[data-component='photo-viewer'][role='dialog'][aria-modal='true']").await?;
            page.wait_for_present("[data-component='photo-details']").await?;
            portfolio_e2e::ensure_eq!(button.attr("aria-expanded").await?, Some("true".to_owned()));
            portfolio_e2e::ensure_eq!(button.attr("aria-controls").await?, Some("photo-viewer".to_owned()));

            driver.find(By::Css("[data-component='photo-info-dismiss']")).await?.click().await?;
            page.wait_until_gone("[data-component='photo-viewer']").await?;
            portfolio_e2e::ensure_eq!(button.attr("aria-expanded").await?, Some("false".to_owned()));

            button.click().await?;
            page.wait_for_present("[data-component='photo-viewer']").await?;
            page.wait_for_present("[data-component='photo-viewer-close']").await?.click().await?;
            page.wait_until_gone("[data-component='photo-viewer']").await?;
            portfolio_e2e::ensure_eq!(button.attr("aria-expanded").await?, Some("false".to_owned()));

            Ok(())
        })
    }).await
}

#[tokio::test]
async fn photo_button_is_hidden_on_small_viewports() -> WebDriverResult<()> {
    harness::run_named("photo-responsive-visibility", |driver, config| {
        Box::pin(async move {
            driver.set_window_rect(0, 0, 375, 700).await?;
            let page = PortfolioPage::new(driver, config);
            page.open("/").await?;
            page.wait_for_present(selectors::APP_READY).await?;

            let button = page.wait_for_present("[data-component='photo-info-toggle']").await?;
            portfolio_e2e::ensure!(!button.is_displayed().await?);

            driver.set_window_rect(0, 0, 1100, 800).await?;
            portfolio_e2e::ensure!(button.is_displayed().await?);
            portfolio_e2e::ensure_eq!(button.text().await?, "About this place");

            Ok(())
        })
    }).await
}

#[tokio::test]
async fn main_content_has_keyboard_skip_link() -> WebDriverResult<()> {
    harness::run_named("skip-navigation", |driver, config| {
        Box::pin(async move {
            let page = PortfolioPage::new(driver, config);
            page.open("/").await?;
            page.wait_for_present(selectors::APP_READY).await?;

            let skip = page.wait_for_present("[data-component='skip-navigation']").await?;
            portfolio_e2e::ensure!(skip.attr("href").await?.unwrap_or_default().ends_with("#main-content"));
            page.wait_for_present("[data-component='route-content']").await?;

            Ok(())
        })
    }).await
}


#[tokio::test]
async fn photo_viewer_preserves_route_and_supports_fit_modes() -> WebDriverResult<()> {
    harness::run_named("photo-viewer-fit-route", |driver, config| {
        Box::pin(async move {
            let page = PortfolioPage::new(driver, config);
            page.open("/about").await?;
            let original_url = driver.current_url().await?;

            let toggle = page.wait_for_present("[data-component='photo-info-toggle']").await?;
            toggle.click().await?;
            let viewer = page.wait_for_present("[data-component='photo-viewer']").await?;
            page.wait_for_images().await?;
            let image = page.wait_for_present("[data-component='photo-viewer-image']").await?;
            portfolio_e2e::ensure!(image.attr("src").await?.unwrap_or_default().ends_with("wallpaper.jpg"));
            portfolio_e2e::ensure_eq!(viewer.attr("data-photo-fit").await?, Some("cover".to_owned()));

            let fit = page.wait_for_present("[data-component='photo-viewer-fit']").await?;
            fit.click().await?;
            portfolio_e2e::ensure_eq!(viewer.attr("data-photo-fit").await?, Some("contain".to_owned()));
            portfolio_e2e::ensure_eq!(fit.attr("aria-pressed").await?, Some("true".to_owned()));

            fit.click().await?;
            portfolio_e2e::ensure_eq!(viewer.attr("data-photo-fit").await?, Some("cover".to_owned()));
            portfolio_e2e::ensure_eq!(fit.attr("aria-pressed").await?, Some("false".to_owned()));

            let scrolling = driver.execute(
                "return document.documentElement.getAttribute('data-photo-viewer-open');",
                vec![],
            ).await?;
            portfolio_e2e::ensure_eq!(scrolling.json().as_str(), Some("true"));

            page.wait_for_present("[data-component='photo-viewer-close']").await?.click().await?;
            page.wait_until_gone("[data-component='photo-viewer']").await?;
            portfolio_e2e::ensure_eq!(driver.current_url().await?, original_url);

            let restored = driver.execute(
                "return document.activeElement?.getAttribute('data-component');",
                vec![],
            ).await?;
            portfolio_e2e::ensure_eq!(restored.json().as_str(), Some("photo-info-toggle"));

            Ok(())
        })
    }).await
}

#[tokio::test]
async fn light_palettes_preserve_background_photograph() -> WebDriverResult<()> {
    harness::run_named("light-background-visible", |driver, config| {
        Box::pin(async move {
            let page = PortfolioPage::new(driver, config);
            page.open("/").await?;
            page.wait_for_present("[data-component='site-background']").await?;

            let state = driver.execute(
                "const root = document.documentElement;
                 const originalTheme = root.dataset.theme;
                 const originalPalette = root.dataset.palette;
                 const results = ['teal', 'navy'].map(palette => {
                     root.dataset.theme = 'light';
                     root.dataset.palette = palette;
                     const style = getComputedStyle(document.querySelector('[data-component=site-background]'));
                     return { palette, opacity: Number(style.opacity), image: style.backgroundImage };
                 });
                 root.dataset.theme = originalTheme;
                 root.dataset.palette = originalPalette;
                 return results;",
                vec![],
            ).await?;

            portfolio_e2e::ensure_eq!(state.json().as_array().map(Vec::len), Some(2));

            for value in state.json().as_array().into_iter().flatten() {
                portfolio_e2e::ensure!(
                    value["opacity"].as_f64().is_some_and(|opacity| opacity >= 0.65),
                    "Light theme obscures the background: {value}"
                );
                portfolio_e2e::ensure!(
                    value["image"].as_str().is_some_and(|image| image.contains("wallpaper.jpg")),
                    "Background photograph missing: {value}"
                );
            }
            Ok(())
        })
    }).await
}
