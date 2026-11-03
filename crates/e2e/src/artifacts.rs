use std::{env, path::PathBuf, sync::atomic::{AtomicU64, Ordering}, time::{SystemTime, UNIX_EPOCH}};
use thirtyfour::{error::{WebDriverError, WebDriverResult}, WebDriver};

static SEQUENCE: AtomicU64 = AtomicU64::new(0);

pub fn output_dir() -> PathBuf {
    env::var_os("E2E_ARTIFACTS_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../target/e2e-artifacts"))
}

pub fn safe_name(name: &str) -> String {
    let result: String = name
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' {
                ch
            } else {
                '_'
            }
        })
        .collect();

    if result.is_empty() {
        "test".to_owned()
    } else {
        result
    }
}

pub async fn capture(driver: &WebDriver, name: &str) -> WebDriverResult<PathBuf> {
    let directory = output_dir();
    std::fs::create_dir_all(&directory).map_err(|e| WebDriverError::ParseError(format!("Screenshot directory: {e}")))?;
    let path = directory.join(format!("{}.png", safe_name(name)));
    driver.screenshot(&path).await?;
    tracing::info!(path = %path.display(), "Screenshot saved");
    Ok(path)
}

pub async fn capture_failure(driver: &WebDriver, name: &str) -> WebDriverResult<PathBuf> {
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_millis();
    let sequence = SEQUENCE.fetch_add(1, Ordering::Relaxed);
    capture(driver, &format!("failure-{name}-{timestamp}-{sequence}")).await
}

#[cfg(test)]
mod tests {
    use super::safe_name;

    #[test]
    fn artifact_names_do_not_escape_output_directory() {
        assert_eq!(safe_name("../../travel"), "______travel");
        assert_eq!(safe_name("layout-mobile_390"), "layout-mobile_390");
        assert_eq!(safe_name(""), "test");
    }
}
