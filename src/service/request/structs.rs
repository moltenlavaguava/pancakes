use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PythonReleaseData {
    #[serde(rename(deserialize = "cycle"))]
    pub major_release: Release,
    #[serde(rename(deserialize = "releaseDate"))]
    pub release_date: NaiveDate,
    pub eol: NaiveDate,
    pub latest: Release,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(try_from = "String")]
pub struct Release {
    major: u64,
    minor: u64,
    patch: Option<u64>,
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
