use std::path::PathBuf;

use serde::Deserialize;
use url::Url;

#[derive(Debug, Clone, Deserialize)]
struct UVVersionParts {
    major: u64,
    minor: u64,
    patch: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UVRawVersionOutput {
    version_parts: UVVersionParts,
    url: Option<Url>,
    path: Option<PathBuf>,
}
pub struct CurrentReleaseData {
    latest_release: Release,
    current_version: Option<Release>,
}
impl CurrentReleaseData {
    pub fn from_uv_raw_version_output(output: Vec<UVRawVersionOutput>) -> CurrentReleaseData {
        let latest = output
            .get(0)
            .expect("Expected avaliable Python versions to be greater than 0")
            .version_parts
            .clone();
        let current = output
            .iter()
            .find(|r| r.path.is_some())
            .map(|o| o.version_parts.clone().into());
        Self {
            latest_release: latest.into(),
            current_version: current.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Release {
    pub major: u64,
    pub minor: u64,
    pub patch: Option<u64>,
}
impl std::fmt::Display for Release {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)?;
        if let Some(p) = self.patch {
            write!(f, ".{}", p)?;
        }
        Ok(())
    }
}
impl TryFrom<String> for Release {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = s.split(".").collect();
        let major = if let Some(t) = parts.get(0)
            && let Ok(v) = t.parse()
        {
            v
        } else {
            return Err(format!("Failed to parse major release value"));
        };
        let minor = if let Some(t) = parts.get(1)
            && let Ok(v) = t.parse()
        {
            v
        } else {
            return Err(format!("Failed to parse minor release value"));
        };
        let patch = {
            if let Some(t) = parts.get(2) {
                if let Ok(v) = t.parse() {
                    Some(v)
                } else {
                    return Err(format!("Failed to parse patch release value"));
                }
            } else {
                None
            }
        };
        Ok(Self {
            major,
            minor,
            patch,
        })
    }
}
impl Into<String> for Release {
    fn into(self) -> String {
        self.to_string()
    }
}
impl From<UVVersionParts> for Release {
    fn from(value: UVVersionParts) -> Self {
        Release {
            major: value.major,
            minor: value.minor,
            patch: Some(value.patch),
        }
    }
}
