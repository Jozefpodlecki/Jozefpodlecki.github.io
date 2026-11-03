
use crate::artifacts;
use tracing::{debug, error, info, warn};
use std::{
    fs::{self, File}, net::TcpListener, path::PathBuf, process::Stdio, time::{Duration, Instant},
};
use thirtyfour::error::{WebDriverError, WebDriverResult};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    process::{Child, Command},
    time::sleep,
};

pub struct TrunkServer {
    process: Child,
    pub base_url: String,
    log_path: PathBuf,
}

impl TrunkServer {
    pub async fn start() -> WebDriverResult<Self> {
        Self::start_named("browser-test").await
    }

    pub async fn start_named(test_name: &str) -> WebDriverResult<Self> {
        let listener = TcpListener::bind("127.0.0.1:0")
            .map_err(|e| WebDriverError::RequestFailed(e.to_string()))?;

        let port = listener
            .local_addr()
            .map_err(|e| WebDriverError::RequestFailed(e.to_string()))?
            .port();

        drop(listener);

        let web_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../web");

        let log_path = artifacts::output_dir()
            .join(format!("trunk-{}.log", artifacts::safe_name(test_name)));
        let log_directory = log_path.parent().ok_or_else(|| {
            WebDriverError::RequestFailed(format!(
                "Trunk log directory is missing: {}",
                log_path.display()
            ))
        })?;

        fs::create_dir_all(log_directory).map_err(|error| {
            WebDriverError::RequestFailed(format!(
                "Failed to create Trunk log directory {}: {error}",
                log_directory.display()
            ))
        })?;

        let log = File::create(&log_path).map_err(|error| {
            WebDriverError::RequestFailed(format!(
                "Failed to create Trunk log {}: {error}",
                log_path.display()
            ))
        })?;

        let stderr_log = log.try_clone().map_err(|error| {
            WebDriverError::RequestFailed(format!(
                "Failed to duplicate Trunk log {}: {error}",
                log_path.display()
            ))
        })?;

        info!(port, path = %log_path.display(), "Launching Trunk");
        let process = Command::new("trunk")
            .arg("serve")
            .arg("--port")
            .arg(port.to_string())
            .current_dir(web_root)
            .kill_on_drop(true)
            .stdin(Stdio::null())
            .stdout(Stdio::from(log))
            .stderr(Stdio::from(stderr_log))
            .spawn()
            .map_err(|e| {
                WebDriverError::RequestFailed(format!(
                    "Failed to start Trunk: {e}. Log: {}", log_path.display()
                ))
            })?;

        let mut server = Self {
            process,
            base_url: format!("http://127.0.0.1:{port}"),
            log_path,
        };

        server.wait_until_ready().await?;
        info!(url = %server.base_url, "Trunk is ready");

        Ok(server)
    }

    async fn wait_until_ready(&mut self) -> WebDriverResult<()> {
        let deadline = Instant::now() + Duration::from_secs(120);

        loop {
            if let Some(status) = self
                .process
                .try_wait()
                .map_err(|e| WebDriverError::RequestFailed(e.to_string()))?
            {
                error!(%status, log = %self.log_path.display(), "Trunk exited before readiness");
                return Err(WebDriverError::RequestFailed(format!(
                    "Trunk exited before becoming ready: {status}. Inspect {}",
                    self.log_path.display()
                )));
            }

            if self.is_ready().await {
                return Ok(());
            }

            if Instant::now() >= deadline {
                error!(log = %self.log_path.display(), "Timed out waiting for Trunk");
                return Err(WebDriverError::Timeout(format!(
                    "Trunk did not become ready within 120 seconds. Inspect {}",
                    self.log_path.display()
                )));
            }

            sleep(Duration::from_millis(250)).await;
        }
    }

    async fn is_ready(&self) -> bool {
        let address = self.base_url.trim_start_matches("http://");

        let check = async {
            let mut stream = TcpStream::connect(address).await?;

            stream
                .write_all(
                    b"GET / HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n",
                )
                .await?;

            let mut response = [0u8; 256];
            let length = stream.read(&mut response).await?;

            Ok::<bool, std::io::Error>(
                String::from_utf8_lossy(&response[..length])
                    .starts_with("HTTP/1.1 200"),
            )
        };

        matches!(
            tokio::time::timeout(Duration::from_secs(2), check).await,
            Ok(Ok(true))
        )
    }

    pub async fn stop(mut self) -> WebDriverResult<()> {
        debug!(url = %self.base_url, "Stopping Trunk");
        if self.process.try_wait()
            .map_err(|error| WebDriverError::RequestFailed(error.to_string()))?
            .is_some()
        {
            warn!("Trunk already exited before cleanup");
            return Ok(());
        }

        self.process
            .kill()
            .await
            .map_err(|error| WebDriverError::RequestFailed(error.to_string()))?;
        debug!("Trunk stopped");
        Ok(())
    }
}
