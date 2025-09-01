use std::path::Path;

use crate::{FullProjectDirs, Project};

/// Get a unix dir for a given path. Independent from project
pub fn unix(full_project_path: &Path) -> FullProjectDirs {
    FullProjectDirs {
        cache: full_project_path.join("cache"),
        data: full_project_path.join("data"),
        log: full_project_path.join("log"),
        runtime: Some(full_project_path.join("tmp")),
        state: full_project_path.join("state"),
        bin: full_project_path.join("bin"),
        config: full_project_path.into(),
        include: full_project_path.join("include"),
        lib: full_project_path.join("lib"),
        project_root: Some(full_project_path.into()),
    }
}

/// Retrieve unix-style [`FullProjectDirs`]
pub trait Unix {
    /// Use custom prefix and parent path for the unix-style directories
    fn unix_prefixed(&self, parent_path: &Path, prefix: &str) -> FullProjectDirs;

    /// Same as unix_prefixed, but assumes prefix is "."
    fn unix(&self, parent_path: &Path) -> FullProjectDirs {
        self.unix_prefixed(parent_path, "")
    }

    /// Get path to the unix-style directories for the current working directory (PWD). Assumes
    /// prefix is ".".
    fn unix_pwd(&self) -> Result<FullProjectDirs, std::io::Error>;

    /// Get path to the unix-style directories for the current user. Assumes prefix is ".".
    fn unix_home(&self) -> Option<FullProjectDirs>;

    /// Get path to the unix-style directories for the current binary. Assumes prefix is ".".
    fn unix_binary(&self) -> Result<FullProjectDirs, std::io::Error>;
}

impl Unix for Project {
    fn unix_prefixed(&self, parent_path: &Path, prefix: &str) -> FullProjectDirs {
        let project_name = format!("{}{}", prefix, self.application_name);
        let full_project_path = parent_path.join(project_name);

        unix(&full_project_path)
    }

    fn unix_pwd(&self) -> Result<FullProjectDirs, std::io::Error> {
        std::env::current_dir().map(|path| self.unix_prefixed(&path, "."))
    }

    fn unix_home(&self) -> Option<FullProjectDirs> {
        crate::dir_utils::home_dir().map(|path| self.unix_prefixed(&path, "."))
    }

    fn unix_binary(&self) -> Result<FullProjectDirs, std::io::Error> {
        std::env::current_exe().map(|path| self.unix_prefixed(&path, "."))
    }
}
