use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::model::SiteProject;

const MAX_RECENT_PROJECTS: usize = 12;

/// Recorded entry for a recently opened or saved `.weysko` / `.weysko.json` project.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RecentProjectEntry {
    pub name: String,
    pub path: PathBuf,
    pub theme_label: String,
    pub page_count: usize,
    pub element_count: usize,
    pub opened_at_unix: u64,
}

/// Persistent store of recently opened projects on the local machine.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct RecentProjectsStore {
    pub entries: Vec<RecentProjectEntry>,
}

impl RecentProjectsStore {
    /// Returns the per-user configuration file path across Windows, macOS, and Linux.
    pub fn default_store_path() -> Option<PathBuf> {
        #[cfg(target_os = "windows")]
        {
            env::var("APPDATA")
                .ok()
                .map(|dir| PathBuf::from(dir).join("Weysko").join("recent_projects.json"))
        }
        #[cfg(target_os = "macos")]
        {
            env::var("HOME").ok().map(|home| {
                PathBuf::from(home)
                    .join("Library")
                    .join("Application Support")
                    .join("Weysko")
                    .join("recent_projects.json")
            })
        }
        #[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
        {
            if let Ok(xdg) = env::var("XDG_CONFIG_HOME") {
                Some(PathBuf::from(xdg).join("weysko").join("recent_projects.json"))
            } else {
                env::var("HOME").ok().map(|home| {
                    PathBuf::from(home)
                        .join(".config")
                        .join("weysko")
                        .join("recent_projects.json")
                })
            }
        }
    }

    pub fn load() -> Self {
        Self::default_store_path()
            .map(|p| Self::load_from(&p))
            .unwrap_or_default()
    }

    pub fn save(&self) {
        if let Some(p) = Self::default_store_path() {
            let _ = self.save_to(&p);
        }
    }

    pub fn load_from(path: &Path) -> Self {
        let Ok(raw) = fs::read_to_string(path) else {
            return Self::default();
        };
        serde_json::from_str(&raw).unwrap_or_default()
    }

    pub fn save_to(&self, path: &Path) -> std::io::Result<()> {
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent)?;
            }
        }
        let raw = serde_json::to_string_pretty(self).map_err(std::io::Error::other)?;
        fs::write(path, raw)
    }

    /// Inserts or promotes a project file in the recent list and caps the list length.
    pub fn record(&mut self, project: &SiteProject, path: &Path) {
        let canonical = fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
        self.entries
            .retain(|e| !paths_equal(&e.path, &canonical) && !paths_equal(&e.path, path));

        let element_count = project
            .pages
            .iter()
            .flat_map(|p| p.sections.iter())
            .map(|s| s.elements.len())
            .sum();

        let opened_at_unix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        self.entries.insert(
            0,
            RecentProjectEntry {
                name: project.name.clone(),
                path: canonical,
                theme_label: project.theme.preset.label().to_string(),
                page_count: project.pages.len(),
                element_count,
                opened_at_unix,
            },
        );
        self.entries.truncate(MAX_RECENT_PROJECTS);
    }

    pub fn remove(&mut self, path: &Path) {
        self.entries.retain(|e| !paths_equal(&e.path, path));
    }

    /// Formats a human-readable relative age label for UI cards.
    pub fn relative_time_label(unix_secs: u64) -> String {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(unix_secs);
        let diff = now.saturating_sub(unix_secs);
        if diff < 60 {
            "Just now".to_string()
        } else if diff < 3600 {
            format!("{}m ago", diff / 60)
        } else if diff < 86_400 {
            format!("{}h ago", diff / 3600)
        } else {
            format!("{}d ago", diff / 86_400)
        }
    }
}

fn paths_equal(a: &Path, b: &Path) -> bool {
    if cfg!(windows) {
        a.to_string_lossy()
            .eq_ignore_ascii_case(&b.to_string_lossy())
    } else {
        a == b
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::templates::StarterTemplate;

    #[test]
    fn recent_projects_record_dedup_cap_remove_and_roundtrip() {
        let tmp = env::temp_dir().join(format!("weysko_recent_{}", uuid::Uuid::new_v4()));
        let store_path = tmp.join("recent.json");
        let proj = StarterTemplate::BlankCanvas.build();

        let mut store = RecentProjectsStore::default();
        for i in 0..15 {
            let p = tmp.join(format!("site_{}.weysko", i));
            store.record(&proj, &p);
        }
        assert_eq!(store.entries.len(), MAX_RECENT_PROJECTS);

        let target = tmp.join("site_14.weysko");
        store.record(&proj, &target);
        assert_eq!(store.entries.len(), MAX_RECENT_PROJECTS);
        assert_eq!(store.entries[0].path, target);

        store.save_to(&store_path).unwrap();
        let loaded = RecentProjectsStore::load_from(&store_path);
        assert_eq!(loaded.entries.len(), MAX_RECENT_PROJECTS);

        store.remove(&target);
        assert_eq!(store.entries.len(), MAX_RECENT_PROJECTS - 1);

        let _ = fs::remove_dir_all(&tmp);
    }
}
