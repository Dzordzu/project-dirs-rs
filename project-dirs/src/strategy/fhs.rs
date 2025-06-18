use crate::FullProjectDirs;
use crate::Project;
use std::path::PathBuf;

/// Get linux-style [`FullProjectDirs`] for the current system. Follows FHS standard.
pub trait Fhs {
    /// Get standard project directories. Assumes that files can be shared across the system (ex.
    /// using nfs)
    fn fhs(&self) -> FullProjectDirs;

    /// Get local project directories. Assumes that files are not shared across the system
    fn fhs_local(&self) -> FullProjectDirs;
}

#[cfg(windows)]
fn fhs_normalize_path(path: PathBuf) -> PathBuf {
    let components = path
        .components()
        .peekable()
        .filter(|x| !matches!(x, std::path::Component::RootDir));
    PathBuf::from(components.fold(String::new(), |acc, x| {
        format!("{acc}/{}", x.as_os_str().to_string_lossy())
    }))
}

#[cfg(not(windows))]
fn fhs_normalize_path(path: PathBuf) -> PathBuf {
    path
}

impl Fhs for Project {
    fn fhs(&self) -> FullProjectDirs {
        #[cfg(not(windows))]
        let application_name = &self.application_name;

        #[cfg(windows)]
        let application_name = &self.application_name_unix();

        FullProjectDirs {
            cache: fhs_normalize_path(PathBuf::from("/var/cache/").join(application_name)),
            data: fhs_normalize_path(PathBuf::from("/var/lib").join(application_name)),
            log: fhs_normalize_path(PathBuf::from("/var/log").join(application_name)),
            runtime: Some(fhs_normalize_path(
                PathBuf::from("/run").join(application_name),
            )),
            state: fhs_normalize_path(PathBuf::from("/var/lib").join(application_name)),
            project_root: None,
            // Unique values for other types
            bin: fhs_normalize_path(PathBuf::from("/usr/bin")),
            config: fhs_normalize_path(PathBuf::from("/etc").join(application_name)),
            include: fhs_normalize_path(PathBuf::from("/usr/include").join(application_name)),
            lib: fhs_normalize_path(PathBuf::from("/usr/lib").join(application_name)),
        }
    }
    fn fhs_local(&self) -> FullProjectDirs {
        #[cfg(not(windows))]
        let application_name = &self.application_name;

        #[cfg(windows)]
        let application_name = &self.application_name_unix();

        FullProjectDirs {
            cache: fhs_normalize_path(PathBuf::from("/var/cache/").join(application_name)),
            data: fhs_normalize_path(PathBuf::from("/var/lib").join(application_name)),
            log: fhs_normalize_path(PathBuf::from("/var/log").join(application_name)),
            runtime: Some(fhs_normalize_path(
                PathBuf::from("/run").join(application_name),
            )),
            state: fhs_normalize_path(PathBuf::from("/var/lib").join(application_name)),
            project_root: None,
            // Unique values for other types
            bin: fhs_normalize_path(PathBuf::from("/usr/local/bin")),
            config: fhs_normalize_path(PathBuf::from("/usr/local/etc").join(application_name)),
            include: fhs_normalize_path(PathBuf::from("/usr/local/include").join(application_name)),
            lib: fhs_normalize_path(PathBuf::from("/usr/local/lib").join(application_name)),
        }
    }
}
