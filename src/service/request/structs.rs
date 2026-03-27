use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::service::request::enums::DateBool;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct PythonReleaseData {
    #[serde(alias = "cycle")]
    pub major_release: Release,
    #[serde(alias = "releaseDate")]
    pub release_date: NaiveDate,
    #[serde(alias = "latestReleaseDate")]
    pub latest_release_date: NaiveDate,
    #[serde(alias = "support")]
    pub support_date: DateBool,
    pub eol: NaiveDate,
    pub latest: Release,
}
impl std::fmt::Display for PythonReleaseData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.major_release, f)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(try_from = "String")]
#[serde(into = "String")]
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
