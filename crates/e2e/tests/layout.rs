use portfolio_e2e::{harness, pages::portfolio::PortfolioPage, selectors};
use thirtyfour::error::WebDriverResult;

#[tokio::test]
async fn pages_fit_viewports_and_images_load() -> WebDriverResult<()> {
    harness::run_named("layout", |driver, config| {
        Box::pin(async move {
            let page = PortfolioPage::new(driver, config);
            for (width, height) in [(1440, 900), (768, 1024), (390, 844), (320, 700)] {
                driver.set_window_rect(0, 0, width, height).await?;
                for route in ["/", "/about", "/projects", "/projects/sudoku"] {
                    page.open(route).await?;
                    page.wait_for_present(selectors::APP_READY).await?;
                    page.wait_for_present(selectors::MAIN_HEADING).await?;

                    page.wait_for_images().await?;

                    let layout = driver.execute(
                        "return { overflow: document.documentElement.scrollWidth > window.innerWidth + 1, brokenImages: Array.from(document.images).filter(image => !image.complete || image.naturalWidth === 0).map(image => image.currentSrc || image.src) };",
                        vec![],
                    ).await?;
                    let result = layout.json();
                    portfolio_e2e::ensure_eq!(result["overflow"].as_bool(), Some(false), "Horizontal overflow at {width}x{height} on {route}: {result}");
                    portfolio_e2e::ensure_eq!(result["brokenImages"].as_array().map(Vec::len), Some(0), "Broken images at {width}x{height} on {route}: {result}");
                }
            }
            Ok(())
        })
    }).await
}
