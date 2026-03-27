use chrono::Utc;

use crate::service::request::{enums::DateBool, structs::PythonReleaseData};

pub fn filter_compiled_python_versions(
    full_python_version_list: &Vec<PythonReleaseData>,
) -> impl Iterator<Item = &PythonReleaseData> {
    let current_date = Utc::now().date_naive();
    full_python_version_list
        .iter()
        // 1. make sure the release is actually released
        // 2. ensure support still exists for this version (so a compiled binary exists)
        .filter(move |&d| {
            current_date >= d.release_date && {
                match d.support_date {
                    DateBool::Date(date) => d.latest_release_date <= date,
                    DateBool::Bool(b) => b,
                }
            }
        })
}
