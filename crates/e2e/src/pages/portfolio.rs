use crate::{config::TestConfig, selectors};
use thirtyfour::{prelude::*, error::WebDriverResult};

pub struct PortfolioPage<'a> {
    driver: &'a WebDriver,
    config: &'a TestConfig,
}

impl<'a> PortfolioPage<'a> {
    pub fn new(driver: &'a WebDriver, config: &'a TestConfig) -> Self {
        Self { driver, config }
    }

    pub async fn open(&self, route: &str) -> WebDriverResult<()> {
        let url = self.config.route_url(route);
        tracing::debug!(%route, %url, "Navigating to portfolio route");
        self.driver.goto(url).await
    }

    pub async fn wait_for_heading(&self, text: &str) -> WebDriverResult<WebElement> {
        self.driver
            .query(By::Css(selectors::MAIN_HEADING))
            .with_text(text.to_owned())
            .wait(self.config.timeout, self.config.poll_interval)
            .first()
            .await
    }

    pub async fn wait_for_present(&self, css: &str) -> WebDriverResult<WebElement> {
        self.driver.query(By::Css(css))
            .wait(self.config.timeout, self.config.poll_interval)
            .first().await
    }

    pub async fn click_visible(&self, css: &str) -> WebDriverResult<()> {
        let element = self.wait_for_present(css).await?;
        self.driver
            .execute(
                "document.documentElement.style.scrollBehavior = 'auto';",
                vec![],
            )
            .await?;
        element.scroll_into_view().await?;
        element.click_when_ready().await
    }

    pub async fn experience_entries(&self) -> WebDriverResult<Vec<WebElement>> {
        self.driver
            .query(By::Css(selectors::EXPERIENCE_ENTRIES))
            .wait(self.config.timeout, self.config.poll_interval)
            .any_required()
            .await
    }

    pub async fn wait_for_count(&self, css: &str, expected: usize) -> WebDriverResult<Vec<WebElement>> {
        let deadline = tokio::time::Instant::now() + self.config.timeout;
        loop {
            let matches = self.driver.find_all(By::Css(css)).await?;
            if matches.len() == expected {
                return Ok(matches);
            }
            if tokio::time::Instant::now() >= deadline {
                return Err(thirtyfour::error::WebDriverError::Timeout(format!(
                    "Expected {expected} elements matching {css}, last observed {}",
                    matches.len()
                )));
            }
            tokio::time::sleep(self.config.poll_interval).await;
        }
    }

    pub async fn wait_until_gone(&self, css: &str) -> WebDriverResult<()> {
        self.driver
            .query(By::Css(css))
            .wait(self.config.timeout, self.config.poll_interval)
            .wait_until_gone()
            .await
    }

    pub async fn wait_for_images(&self) -> WebDriverResult<()> {
        let deadline = tokio::time::Instant::now() + self.config.timeout;
        loop {
            let state = self.driver.execute(
                "return Array.from(document.images).every(image => image.complete && (image.naturalWidth > 0 || image.closest('[data-component=project-artwork][data-image-status=unavailable]')));",
                vec![],
            ).await?;
            if state.json().as_bool() == Some(true) {
                return Ok(());
            }
            if tokio::time::Instant::now() >= deadline {
                let diagnostics = self.driver.execute(
                    "return Array.from(document.images).filter(image => !image.complete || image.naturalWidth === 0).map(image => ({ src: image.currentSrc || image.src, complete: image.complete, naturalWidth: image.naturalWidth }));",
                    vec![],
                ).await?;
                return Err(thirtyfour::error::WebDriverError::Timeout(format!(
                    "Images did not settle: {}", diagnostics.json()
                )));
            }
            tokio::time::sleep(self.config.poll_interval).await;
        }
    }

    pub async fn wait_for_motion(&self) -> WebDriverResult<()> {
        let deadline = tokio::time::Instant::now() + self.config.timeout;
        loop {
            let state = self.driver.execute(
                "return Array.from(document.querySelectorAll('[data-component=motion-reveal]')).every(element => parseFloat(element.style.opacity || '1') >= 0.999);",
                vec![],
            ).await?;
            if state.json().as_bool() == Some(true) {
                return Ok(());
            }
            if tokio::time::Instant::now() >= deadline {
                return Err(thirtyfour::error::WebDriverError::Timeout(
                    "Animations did not settle within the configured timeout".into(),
                ));
            }
            tokio::time::sleep(self.config.poll_interval).await;
        }
    }

    pub async fn wait_for_body(&self) -> WebDriverResult<WebElement> {
        self.wait_for_present(selectors::PAGE_BODY).await
    }
}
