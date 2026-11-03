use std::{rc::Rc, str::FromStr};

use log::Level;
use thiserror::Error;
use web_sys::{Document, Element, Storage, Window, window};

#[derive(Clone, PartialEq)]
pub struct AppEnvironment {
    pub root: Element,
    pub window: Window,
    pub document: Document,
    pub local_storage: Option<Storage>,
    pub app_name: Rc<str>,
    pub version: Rc<str>,
}

#[derive(Debug, Error)]
pub enum AppEnvironmentError {
    #[error("Window object not found")]
    NoWindow,
    #[error("Document object not found")]
    NoDocument,
    #[error("Application root element not found")]
    NoRoot,
}

impl AppEnvironment {
    pub fn new() -> Result<Self, AppEnvironmentError> {
        let window = window().ok_or(AppEnvironmentError::NoWindow)?;
        let document = window.document().ok_or(AppEnvironmentError::NoDocument)?;
        let root = document.get_element_by_id("app").ok_or(AppEnvironmentError::NoRoot)?;
        let local_storage = window.local_storage().ok().flatten();

        Ok(Self {
            root,
            window,
            document,
            local_storage,
            app_name: "Józef Podlecki — Portfolio".into(),
            version: env!("CARGO_PKG_VERSION").into(),
        })
    }

    pub fn log_level(&self) -> Level {
        self.local_storage
            .as_ref()
            .and_then(|storage| storage.get_item("RUST_LOG").ok().flatten())
            .and_then(|value| Level::from_str(&value).ok())
            .unwrap_or(if cfg!(debug_assertions) { Level::Debug } else { Level::Info })
    }
}

#[derive(Clone, PartialEq)]
pub struct ThemeService {
    environment: AppEnvironment,
}

impl ThemeService {
    pub fn new(environment: AppEnvironment) -> Self {
        Self { environment }
    }

    pub fn current_is_dark(&self) -> bool {
        self.environment.document.document_element()
            .and_then(|root| root.get_attribute("data-theme"))
            .as_deref() == Some("dark")
    }

    pub fn set_dark(&self, dark: bool) {
        let theme = if dark { "dark" } else { "light" };
        if let Some(root) = self.environment.document.document_element() {
            if let Err(error) = root.set_attribute("data-theme", theme) {
                log::warn!("Unable to update theme attribute: {error:?}");
            }
        }
        if let Some(storage) = &self.environment.local_storage {
            if let Err(error) = storage.set_item("portfolio-theme", theme) {
                log::warn!("Unable to persist theme: {error:?}");
            }
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct MotionService {
    environment: AppEnvironment,
}

impl MotionService {
    pub fn new(environment: AppEnvironment) -> Self {
        Self { environment }
    }

    pub fn prefers_reduced_motion(&self) -> bool {
        self.environment
            .window
            .match_media("(prefers-reduced-motion: reduce)")
            .ok()
            .flatten()
            .is_some_and(|query| query.matches())
    }

    pub fn now_ms(&self) -> Option<f64> {
        self.environment
            .window
            .performance()
            .map(|performance| performance.now())
    }
}
