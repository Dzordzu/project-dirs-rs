use std::path::{Path, PathBuf};

use crate::{FullProjectDirs, ProjectDirs};

/// Filter project directories
pub trait Filter {
    /// Return only existing directories.
    fn filter_existing_dirs(&self) -> ProjectDirs;

    /// Return only non-directory entries in places where directories are expected.
    /// E.g. `log` file in the place of the `log` directory.
    fn filter_non_dirs(&self) -> ProjectDirs;

    /// Return only non-existing fs entries.
    /// E.g. missing `log` directory.
    fn filter_absent(&self) -> ProjectDirs;

    /// Return only access denied fs entries.
    /// E.g. no read and execute access to `log` directory.
    fn filter_denied(&self) -> ProjectDirs;

    /// Basically filter using `!path.is_dir()`, negation of the [`Filter::filter_existing_dirs`]
    fn filter_non_valid(&self) -> ProjectDirs;
}

impl Filter for ProjectDirs {
    fn filter_existing_dirs(&self) -> ProjectDirs {
        ProjectDirs::new(
            self.0
                .iter()
                .filter(|(_, p)| p.is_dir())
                .map(|x| (*x.0, x.1.clone()))
                .collect(),
        )
    }

    fn filter_non_dirs(&self) -> ProjectDirs {
        ProjectDirs::new(
            self.0
                .iter()
                .filter(|(_, p)| p.exists() && !p.is_dir())
                .map(|x| (*x.0, x.1.clone()))
                .collect(),
        )
    }

    fn filter_absent(&self) -> ProjectDirs {
        ProjectDirs::new(
            self.0
                .iter()
                .filter(|(_, p)| p.try_exists().map(|e| !e).unwrap_or(false))
                .map(|x| (*x.0, x.1.clone()))
                .collect(),
        )
    }

    fn filter_denied(&self) -> ProjectDirs {
        ProjectDirs::new(
            self.0
                .iter()
                .filter(|(_, p)| p.try_exists().is_err())
                .map(|x| (*x.0, x.1.clone()))
                .collect(),
        )
    }

    fn filter_non_valid(&self) -> ProjectDirs {
        ProjectDirs::new(
            self.0
                .iter()
                .filter(|(_, p)| !p.is_dir())
                .map(|x| (*x.0, x.1.clone()))
                .collect(),
        )
    }
}

/// Mount directories inside the provided mountpoint
pub trait Mounted {
    fn mounted(self, mountpoint: &Path) -> Self;
}

fn strip_root_prefix(p: PathBuf) -> PathBuf {
    if p.starts_with("/") {
        p.strip_prefix("/")
            .expect("Failed to strip / prefix from paths. This is a bug")
            .to_path_buf()
    } else {
        p
    }
}

impl Mounted for PathBuf {
    fn mounted(self, mountpoint: &Path) -> Self {
        mountpoint.join(strip_root_prefix(self.to_path_buf()))
    }
}

impl Mounted for ProjectDirs {
    fn mounted(self, mountpoint: &Path) -> Self {
        ProjectDirs::new(
            self.0
                .into_iter()
                .map(|(d, p)| (d, mountpoint.join(strip_root_prefix(p))))
                .collect(),
        )
    }
}

impl Mounted for FullProjectDirs {
    fn mounted(self, mountpoint: &Path) -> Self {
        let pd: ProjectDirs = self.into();
        pd.mounted(mountpoint).try_into().unwrap()
    }
}

/// Retrive home directory
pub fn home_dir() -> Option<PathBuf> {
    #[cfg(feature = "nonstd_home_dir")]
    return home::home_dir();

    #[cfg(not(feature = "nonstd_home_dir"))]
    return std::env::home_dir();
}
