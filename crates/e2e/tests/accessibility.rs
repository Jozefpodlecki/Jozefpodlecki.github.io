use portfolio_e2e::{harness, pages::portfolio::PortfolioPage};
use thirtyfour::{error::{WebDriverError, WebDriverResult}, By};

#[tokio::test]
async fn hero_heading_has_clean_copy_and_navigation_is_named() -> WebDriverResult<()> {
    harness::run_named("hero-accessibility", |driver, config| Box::pin(async move {
        let page = PortfolioPage::new(driver, config);
        page.open("/").await?;

        let heading = page.wait_for_present("[data-component='home-hero'] h1").await?;
        page.wait_for_motion().await?;

        let heading_text = heading.text().await?
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        let expected_heading = "Hi, I'm Józef I build software";
        if heading_text != expected_heading {
            return Err(WebDriverError::RequestFailed(format!(
                "Unexpected hero heading: expected {expected_heading:?}, got {heading_text:?}"
            )));
        }

        let navigation = driver.find(By::Css("[data-component='site-navigation']")).await?;
        let navigation_label = navigation.attr("aria-label").await?;
        if navigation_label.as_deref() != Some("Primary navigation") {
            return Err(WebDriverError::RequestFailed(format!(
                "Unexpected primary navigation label: {navigation_label:?}"
            )));
        }

        let theme = driver.find(By::Css("[data-component='theme-toggle']")).await?;
        if theme.attr("aria-label").await?.is_none() {
            return Err(WebDriverError::RequestFailed(
                "Theme toggle is missing an accessible name".to_owned()
            ));
        }
        Ok(())
    })).await
}

#[tokio::test]
async fn photograph_toggle_exposes_expanded_state_and_details() -> WebDriverResult<()> {
    harness::run_named("photo-accessibility", |driver, config| Box::pin(async move {
        let page = PortfolioPage::new(driver, config);
        page.open("/").await?;

        let toggle = page.wait_for_present("[data-component='photo-info-toggle']").await?;
        portfolio_e2e::ensure_eq!(toggle.attr("aria-expanded").await?, Some("false".to_owned()));
        toggle.click().await?;
        page.wait_for_present("[data-component='photo-viewer'][role='dialog'][aria-modal='true']").await?;
        page.wait_for_present("[data-component='photo-details']").await?;
        portfolio_e2e::ensure_eq!(toggle.attr("aria-expanded").await?, Some("true".to_owned()));
        portfolio_e2e::ensure_eq!(toggle.attr("aria-controls").await?, Some("photo-viewer".to_owned()));

        page.wait_for_present("[data-component='photo-viewer-close']").await?.click().await?;
        page.wait_until_gone("[data-component='photo-viewer']").await?;
        portfolio_e2e::ensure_eq!(toggle.attr("aria-expanded").await?, Some("false".to_owned()));

        Ok(())
    })).await
}

#[tokio::test]
async fn travel_dialog_exposes_state_and_accessible_country_controls() -> WebDriverResult<()> {
    harness::run_named("travel-accessibility", |driver, config| Box::pin(async move {
        let page = PortfolioPage::new(driver, config);
        page.open("/about").await?;
        page.wait_for_present("[data-component='world-map'] [data-country='POL'][role='button'][aria-label]").await?;

        let expand = page.wait_for_present("[data-component='travel-expand']").await?;
        portfolio_e2e::ensure_eq!(expand.attr("aria-expanded").await?, Some("false".to_owned()));
        page.wait_for_motion().await?;
        page.click_visible("[data-component='travel-expand']").await?;

        let dialog = page.wait_for_present("[data-component='travel-map-dialog']").await?;
        portfolio_e2e::ensure_eq!(dialog.attr("aria-modal").await?, Some("true".to_owned()));
        portfolio_e2e::ensure_eq!(expand.attr("aria-expanded").await?, Some("true".to_owned()));
        portfolio_e2e::ensure_eq!(expand.attr("aria-controls").await?, Some("travel-map-dialog".to_owned()));

        page.click_visible("[data-component='travel-collapse']").await?;
        page.wait_until_gone("[data-component='travel-map-dialog']").await?;
        portfolio_e2e::ensure_eq!(expand.attr("aria-expanded").await?, Some("false".to_owned()));
        Ok(())
    })).await
}
