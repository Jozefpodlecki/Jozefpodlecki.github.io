use axum::{
    body::Body,
    extract::{Path as AxumPath, State},
    http::{header, Response, StatusCode},
    routing::get,
    Router,
};
use std::{
    path::{Component, Path, PathBuf},
    sync::{atomic::{AtomicUsize, Ordering}, Arc},
    time::Duration,
};
use tokio::{net::TcpListener, task::JoinHandle};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FixtureScenario {
    Success,
    Missing,
    Malformed,
    Invalid,
    Delayed,
    Retry,
    RetryTwice,
    ProjectsMissing,
    ProjectsMalformed,
    ProjectsRetry,
    ProjectsRetryTwice,
    ProjectsMany,
}

#[derive(Clone)]
struct FixtureState {
    root: PathBuf,
    scenario: FixtureScenario,
    attempts: Arc<AtomicUsize>,
    delay: Duration,
}

pub struct FixtureServer {
    url: String,
    task: JoinHandle<()>,
}

impl FixtureServer {
    pub async fn start(scenario: FixtureScenario) -> std::io::Result<Self> {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../dist");
        Self::start_at(root, scenario).await
    }

    pub async fn start_at(root: PathBuf, scenario: FixtureScenario) -> std::io::Result<Self> {
        crate::observability::init();
        if !root.join("index.html").is_file() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Missing Trunk build: {}", root.join("index.html").display()),
            ));
        }
        let state = FixtureState {
            root,
            scenario,
            attempts: Arc::new(AtomicUsize::new(0)),
            delay: Duration::from_millis(1500),
        };
        let router = Router::new()
            .route("/", get(index))
            .route("/{*path}", get(asset))
            .with_state(state);
        let listener = TcpListener::bind("127.0.0.1:0").await?;
        let url = format!("http://{}", listener.local_addr()?);
        let task = tokio::spawn(async move {
            if let Err(error) = axum::serve(listener, router).await {
                tracing::error!(%error, "Fixture server failed");
            }
        });
        tracing::info!(%url, ?scenario, "Fixture server started");
        Ok(Self { url, task })
    }

    pub fn url(&self) -> &str {
        &self.url
    }
}

impl Drop for FixtureServer {
    fn drop(&mut self) {
        tracing::debug!(url = %self.url, "Stopping fixture server");
        self.task.abort();
    }
}

async fn index(State(state): State<FixtureState>) -> Response<Body> {
    serve_file(&state.root, "index.html").await
}

async fn asset(
    State(state): State<FixtureState>,
    AxumPath(path): AxumPath<String>,
) -> Response<Body> {
    if path == "data/projects.json" {
        if state.scenario == FixtureScenario::ProjectsMany {
            let file = state.root.join("data/projects.json");
            let bytes = match tokio::fs::read(file).await {
                Ok(bytes) => bytes,
                Err(_) => return response(StatusCode::NOT_FOUND, "text/plain", b"Projects unavailable".to_vec()),
            };
            let mut projects: Vec<serde_json::Value> = match serde_json::from_slice(&bytes) {
                Ok(projects) => projects,
                Err(_) => return response(StatusCode::INTERNAL_SERVER_ERROR, "text/plain", b"Invalid fixture projects".to_vec()),
            };
            if let Some(template) = projects.first().cloned() {
                for index in 0..20 {
                    let mut project = template.clone();
                    project["name"] = format!("Example Project {}", index + 1).into();
                    project["slug"] = format!("example-project-{}", index + 1).into();
                    project["priority"] = (100 + index).into();
                    project["github"] = "".into();
                    project["link"] = "".into();
                    projects.push(project);
                }
            }
            return response(
                StatusCode::OK,
                "application/json",
                serde_json::to_vec(&projects).unwrap_or_default(),
            );
        }
        match state.scenario {
            FixtureScenario::ProjectsMissing => return response(StatusCode::NOT_FOUND, "text/plain", b"Projects unavailable".to_vec()),
            FixtureScenario::ProjectsMalformed => return response(StatusCode::OK, "application/json", b"[{".to_vec()),
            FixtureScenario::ProjectsRetry | FixtureScenario::ProjectsRetryTwice => {
                if state.attempts.fetch_add(1, Ordering::SeqCst) < (if state.scenario == FixtureScenario::ProjectsRetryTwice { 2 } else { 1 }) {
                    if state.scenario == FixtureScenario::ProjectsRetryTwice {
                        tokio::time::sleep(Duration::from_millis(250)).await;
                    }
                    return response(StatusCode::SERVICE_UNAVAILABLE, "text/plain", b"Try again".to_vec());
                }
            }
            _ => {}
        }
    }
    if path == "data/profile.json" {
        match state.scenario {
            FixtureScenario::Missing => return response(StatusCode::NOT_FOUND, "text/plain", b"Fixture not found".to_vec()),
            FixtureScenario::Malformed => return response(StatusCode::OK, "application/json", br#"{"full_name":"#.to_vec()),
            FixtureScenario::Invalid => return response(StatusCode::OK, "application/json", br#"{"full_name":"","role":"","intro":"","social":{"github":"","linkedin":""}}"#.to_vec()),
            FixtureScenario::Delayed => tokio::time::sleep(state.delay).await,
            FixtureScenario::Retry | FixtureScenario::RetryTwice => {
                if state.attempts.fetch_add(1, Ordering::SeqCst) < (if state.scenario == FixtureScenario::RetryTwice { 2 } else { 1 }) {
                    if state.scenario == FixtureScenario::RetryTwice {
                        tokio::time::sleep(Duration::from_millis(250)).await;
                    }
                    return response(StatusCode::SERVICE_UNAVAILABLE, "text/plain", b"Temporary fixture failure".to_vec());
                }
            }
            FixtureScenario::Success | FixtureScenario::ProjectsMissing | FixtureScenario::ProjectsMalformed | FixtureScenario::ProjectsRetry | FixtureScenario::ProjectsRetryTwice | FixtureScenario::ProjectsMany => {}
        }
    }
    serve_file(&state.root, &path).await
}

async fn serve_file(root: &Path, path: &str) -> Response<Body> {
    let relative = Path::new(path);
    if !relative.components().all(|part| matches!(part, Component::Normal(_))) {
        return response(StatusCode::BAD_REQUEST, "text/plain", b"Invalid path".to_vec());
    }
    let file = root.join(relative);
    let bytes = match tokio::fs::read(&file).await {
        Ok(bytes) => bytes,
        Err(_) => return response(StatusCode::NOT_FOUND, "text/plain", b"Not found".to_vec()),
    };
    let mime = match file.extension().and_then(|ext| ext.to_str()).unwrap_or_default() {
        "html" => "text/html; charset=utf-8",
        "json" => "application/json",
        "js" => "text/javascript",
        "css" => "text/css",
        "wasm" => "application/wasm",
        "svg" => "image/svg+xml",
        "png" => "image/png",
        "webp" => "image/webp",
        "pdf" => "application/pdf",
        "woff2" => "font/woff2",
        "jpg" | "jpeg" => "image/jpeg",
        "ico" => "image/x-icon",
        _ => "application/octet-stream",
    };
    response(StatusCode::OK, mime, bytes)
}

fn response(status: StatusCode, mime: &'static str, body: Vec<u8>) -> Response<Body> {
    Response::builder()
        .status(status)
        .header(header::CONTENT_TYPE, mime)
        .header(header::CACHE_CONTROL, "no-store")
        .body(Body::from(body))
        .expect("valid fixture response")
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    async fn fixture_root() -> PathBuf {
        static NEXT_ID: AtomicUsize = AtomicUsize::new(0);
        let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "portfolio-fixture-{}-{id}",
            std::process::id()
        ));
        tokio::fs::create_dir_all(root.join("data")).await.unwrap();
        tokio::fs::write(root.join("index.html"), "ready")
            .await
            .unwrap();
        tokio::fs::write(root.join("data/profile.json"), r#"{"full_name":"Józef"}"#)
            .await
            .unwrap();
        root
    }

    async fn fetch(url: &str, path: &str) -> (u16, Vec<u8>) {
        let address = url.strip_prefix("http://").unwrap();
        let mut stream = tokio::net::TcpStream::connect(address).await.unwrap();
        stream
            .write_all(
                format!(
                    "GET {path} HTTP/1.1\r\nHost: {address}\r\nConnection: close\r\n\r\n"
                )
                .as_bytes(),
            )
            .await
            .unwrap();
        let mut bytes = Vec::new();
        tokio::time::timeout(Duration::from_secs(5), stream.read_to_end(&mut bytes))
            .await
            .unwrap()
            .unwrap();
        let separator = bytes.windows(4).position(|part| part == b"\r\n\r\n").unwrap();
        let header = String::from_utf8_lossy(&bytes[..separator]);
        let status = header
            .split_whitespace()
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();
        (status, bytes[separator + 4..].to_vec())
    }

    #[tokio::test]
    async fn serves_success_and_static_assets() {
        let root = fixture_root().await;
        let server = FixtureServer::start_at(root.clone(), FixtureScenario::Success).await.unwrap();
        let (status, body) = fetch(server.url(), "/data/profile.json").await;
        assert_eq!(status, 200);
        assert!(String::from_utf8_lossy(&body).contains("full_name"));
        assert_eq!(fetch(server.url(), "/").await.0, 200);
        assert_eq!(fetch(server.url(), "/unknown.json").await.0, 404);
        drop(server);
        tokio::fs::remove_dir_all(root).await.unwrap();
    }

    #[tokio::test]
    async fn serves_failure_scenarios() {
        let root = fixture_root().await;
        for (scenario, status) in [
            (FixtureScenario::Missing, 404),
            (FixtureScenario::Malformed, 200),
            (FixtureScenario::Invalid, 200),
        ] {
            let server = FixtureServer::start_at(root.clone(), scenario).await.unwrap();
            let (actual, body) = fetch(server.url(), "/data/profile.json").await;
            assert_eq!(actual, status);
            if scenario == FixtureScenario::Malformed {
                assert!(serde_json::from_slice::<serde_json::Value>(&body).is_err());
            }
            if scenario == FixtureScenario::Invalid {
                let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
                assert_eq!(json["full_name"], "");
            }
            drop(server);
        }
        tokio::fs::remove_dir_all(root).await.unwrap();
    }

    #[tokio::test]
    async fn retry_state_isolated_per_server() {
        let root = fixture_root().await;
        let first = FixtureServer::start_at(root.clone(), FixtureScenario::Retry).await.unwrap();
        let second = FixtureServer::start_at(root.clone(), FixtureScenario::Retry).await.unwrap();
        assert_ne!(first.url(), second.url());
        assert_eq!(fetch(first.url(), "/data/profile.json").await.0, 503);
        assert_eq!(fetch(first.url(), "/data/profile.json").await.0, 200);
        assert_eq!(fetch(second.url(), "/data/profile.json").await.0, 503);
        assert_eq!(fetch(second.url(), "/data/profile.json").await.0, 200);
        drop(first);
        drop(second);
        tokio::fs::remove_dir_all(root).await.unwrap();
    }
}
