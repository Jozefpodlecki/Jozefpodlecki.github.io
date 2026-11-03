use serde::Deserialize;
use gloo::net::http::Request;

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Profile {
    pub full_name: String,
    pub role: String,
    pub intro: String,
    #[serde(default)] pub about: Vec<String>,
    #[serde(default)] pub education: Vec<Education>,
    #[serde(default)] pub background: BackgroundPhoto,
    pub social: Social,
}
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Education {
    pub institution: String,
    pub location: String,
    pub qualification: String,
    pub period: String,
    #[serde(default)] pub website: String,
    #[serde(default)] pub logo: String,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
pub struct BackgroundPhoto {
    #[serde(default)] pub image: String,
    #[serde(default)] pub title: String,
    #[serde(default)] pub location: String,
    #[serde(default)] pub year: Option<u16>,
    #[serde(default)] pub photographer: String,
    #[serde(default)] pub description: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Social {
    pub github: String,
    pub linkedin: String,
}
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Experience {
    pub company: String,
    pub location: String,
    pub role: String,
    pub period: String,
    #[serde(default)] pub employment_type: String,
    #[serde(default)] pub work_mode: String,
    #[serde(default)] pub responsibilities: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Project {
    #[serde(default = "default_project_priority")]
    pub priority: u32,
    pub name: String,
    pub description: String,
    #[serde(default)] pub slug: String,
    #[serde(default)] pub link: String,
    #[serde(default)] pub github: String,
    #[serde(default)] pub tags: Vec<String>,
    #[serde(default)] pub body: String,
    #[serde(default)] pub image: String,
}
fn default_project_priority() -> u32 {
    100
}

pub fn sort_projects(projects: &mut [Project]) {
    projects.sort_by_key(|project| project.priority);
}

#[derive(Clone, Debug, PartialEq)]
pub struct SiteData {
    pub profile: Profile,
    pub experience: Vec<Experience>,
}

#[derive(Clone, Debug, PartialEq, thiserror::Error)]
pub enum ContentError {
    #[error("{resource}: network request failed: {detail}")]
    Network { resource: String, detail: String },
    #[error("{resource}: HTTP {status}")]
    Http { resource: String, status: u16 },
    #[error("{resource}: invalid JSON: {detail}")]
    Decode { resource: String, detail: String },
    #[error("{resource}: invalid content: {detail}")]
    Validation { resource: String, detail: String },
}

impl ContentError {
    pub fn is_transient(&self) -> bool {
        matches!(self, Self::Network { .. } | Self::Http { status: 408 | 429 | 500..=599, .. })
    }

    pub fn category(&self) -> &'static str {
        match self {
            Self::Network { .. } => "network",
            Self::Http { .. } => "http",
            Self::Decode { .. } => "decode",
            Self::Validation { .. } => "validation",
        }
    }
}

fn asset_url(filename: &str) -> String {
    format!("data/{filename}")
}

pub async fn fetch_json<T: serde::de::DeserializeOwned>(filename: &str) -> Result<T, ContentError> {
    let url = asset_url(filename);
    let response = Request::get(&url).send().await.map_err(|error| {
        ContentError::Network { resource: filename.into(), detail: error.to_string() }
    })?;
    if !response.ok() {
        return Err(ContentError::Http { resource: filename.into(), status: response.status() });
    }
    response.json::<T>().await.map_err(|error| {
        ContentError::Decode { resource: filename.into(), detail: error.to_string() }
    })
}

pub async fn fetch_core() -> Result<SiteData, ContentError> {
    let (profile, experience) = futures::join!(fetch_json::<Profile>("profile.json"), fetch_json::<Vec<Experience>>("experience.json"));
    let profile = profile?;
    let experience = experience?;
    if profile.full_name.trim().is_empty() || profile.role.trim().is_empty() {
        return Err(ContentError::Validation { resource: "profile.json".into(), detail: "Required profile fields are empty".into() });
    }
    Ok(SiteData { profile, experience })
}


pub fn validate_projects(items: &[Project]) -> Result<(), ContentError> {
    let mut slugs = std::collections::HashSet::new();

    for project in items {
        let slug = project.slug.trim();
        if slug.is_empty() || project.name.trim().is_empty() {
            return Err(ContentError::Validation {
                resource: "projects.json".into(),
                detail: "Every project needs a name and a nonempty slug".into(),
            });
        }
        if !slugs.insert(slug) {
            return Err(ContentError::Validation {
                resource: "projects.json".into(),
                detail: format!("Duplicate project slug: {slug}"),
            });
        }
        if [&project.github, &project.link]
            .iter()
            .any(|url| !url.is_empty() && !url.starts_with("https://"))
        {
            return Err(ContentError::Validation {
                resource: "projects.json".into(),
                detail: format!("Project {slug} has an invalid external URL"),
            });
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn project(slug: &str) -> Project {
        Project {
            priority: 100,
            name: "Example".into(),
            slug: slug.into(),
            description: String::new(),
            link: String::new(),
            github: String::new(),
            tags: Vec::new(),
            body: String::new(),
            image: String::new(),
        }
    }

    #[test]
    fn duplicate_project_slugs_are_rejected() {
        let result = validate_projects(&[project("alpha"), project("alpha")]);
        assert!(matches!(result, Err(ContentError::Validation { .. })));
    }

    #[test]
    fn project_links_must_use_https() {
        let mut item = project("alpha");
        item.github = "http://example.com".into();
        assert!(validate_projects(&[item]).is_err());
    }

    #[test]
    fn priority_sort_preserves_input_order_for_equal_values() {
        let mut projects = vec![project("a"), project("b"), project("c")];
        projects[0].priority = 20;
        projects[1].priority = 5;
        projects[2].priority = 20;
        sort_projects(&mut projects);
        let slugs: Vec<&str> = projects.iter().map(|project| project.slug.as_str()).collect();
        assert_eq!(slugs, ["b", "a", "c"]);
    }

    #[test]
    fn priority_defaults_to_one_hundred() {
        let project: Project = serde_json::from_str(r#"{"name":"Example","slug":"example","description":""}"#).unwrap();
        assert_eq!(project.priority, 100);
    }

    #[test]
    fn empty_project_list_is_valid() {
        assert!(validate_projects(&[]).is_ok());
    }
}
