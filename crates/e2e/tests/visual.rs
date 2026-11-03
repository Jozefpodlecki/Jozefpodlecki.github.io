use std::{env, path::Path};
use thirtyfour::error::{WebDriverError, WebDriverResult};

fn compare_baseline(actual: &Path, label: &str) -> WebDriverResult<()> {
    let Some(directory) = env::var_os("E2E_VISUAL_BASELINE_DIR") else {
        return Ok(());
    };
    let expected = Path::new(&directory).join(format!("{label}.png"));
    let reference = image::open(&expected).map_err(|error| {
        WebDriverError::RequestFailed(format!(
            "Cannot read visual baseline {}: {error}",
            expected.display()
        ))
    })?.to_rgba8();
    let candidate = image::open(actual).map_err(|error| {
        WebDriverError::RequestFailed(format!(
            "Cannot read captured screenshot {}: {error}",
            actual.display()
        ))
    })?.to_rgba8();
    portfolio_e2e::ensure_eq!(
        reference.dimensions(),
        candidate.dimensions(),
        "Screenshot dimensions differ for {label}"
    );
    let total_difference: u64 = reference.pixels()
        .zip(candidate.pixels())
        .map(|(before, after)| {
            before.0.iter()
                .zip(after.0.iter())
                .take(3)
                .map(|(a, b)| u64::from(a.abs_diff(*b)))
                .sum::<u64>()
        })
        .sum();
    let pixels = u64::from(reference.width()) * u64::from(reference.height());
    let mean_difference = total_difference as f64 / (pixels * 3) as f64;
    let maximum = env::var("E2E_VISUAL_MAX_MAE")
        .ok()
        .and_then(|text| text.parse::<f64>().ok())
        .unwrap_or(5.0);
    portfolio_e2e::ensure!(
        mean_difference <= maximum,
        "Visual regression in {label}: mean channel difference {mean_difference:.2}, allowed {maximum:.2}"
    );
    Ok(())
}

use portfolio_e2e::{artifacts, harness, pages::portfolio::PortfolioPage, selectors};

#[tokio::test]
async fn capture_portfolio_pages() -> WebDriverResult<()> {
    harness::run_named("visual", |driver, config| Box::pin(async move {
        let page = PortfolioPage::new(driver, config);
        for (width, height, label) in [(1440, 900, "desktop"), (768, 1024, "tablet"), (390, 844, "mobile"), (320, 700, "narrow")] {
            driver.set_window_rect(0, 0, width, height).await?;
            for (route, name) in [("/", "home"), ("/about", "about"), ("/projects", "projects"), ("/projects/sudoku", "case-study")] {
                page.open(route).await?;
                page.wait_for_present(selectors::APP_READY).await?;
                page.wait_for_present(selectors::MAIN_HEADING).await?;
                if route == "/projects" {
                    page.wait_for_present(selectors::PROJECT_CARDS).await?;
                }
                page.wait_for_images().await?;
                page.wait_for_motion().await?;
                let capture_name = format!("{label}-{name}");
                let image = artifacts::capture(driver, &capture_name).await?;
                compare_baseline(&image, &capture_name)?;
            }
        }
        Ok(())
    })).await
}
