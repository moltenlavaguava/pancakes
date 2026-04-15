use std::{path::PathBuf, str::FromStr};

use pep440_rs::Version;
use serde::Deserialize;
use url::Url;

#[derive(Debug, Clone, Deserialize)]
pub struct UVRawVersionOutput {
    version: String,
    url: Option<Url>,
    path: Option<PathBuf>,
}
#[derive(Debug, Clone)]
pub struct CurrentReleaseData {
    pub latest_release: Option<Version>,
    pub current_version: Option<Version>,
}
impl CurrentReleaseData {
    pub fn from_uv_raw_version_output(output: Vec<UVRawVersionOutput>) -> CurrentReleaseData {
        // get latest *stable* version
        let latest = output
            .iter()
            .find_map(|r| Version::from_str(&r.version).ok().filter(|p| p.is_stable()));
        let current = output
            .iter()
            .find(|r| r.path.is_some())
            .map(|o| Version::from_str(&o.version).ok())
            .flatten();
        Self {
            latest_release: latest,
            current_version: current,
        }
    }
}
