use crate::Directory;
use std::collections::HashMap;
use std::path::PathBuf;

/// List of missing directories for [`ProjectDirs`] to [`FullProjectDirs`] conversion
pub type MissingError = Vec<Directory>;

/// Project directories by directory type ([`Directory`] to [`PathBuf`] mapping)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct ProjectDirs(pub HashMap<Directory, PathBuf>);

impl ProjectDirs {
    pub fn get(&self, dir: &Directory) -> Option<&PathBuf> {
        self.0.get(dir)
    }
}

/// Fully defined project directories by directory type ([`Directory`] to [`PathBuf`] mapping)
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FullProjectDirs {
    /// Binaries directory. This is where the project executable(s) is/are located
    pub bin: PathBuf,
    /// Non-essential data, usually used to speed up the application
    pub cache: PathBuf,
    /// General config store. E.g. conf.d dir and other config files
    pub config: PathBuf,
    /// Essential files for application like db files, cross-session data etc.
    pub data: PathBuf,
    /// Include dir for C/C++ headers.
    pub include: PathBuf,
    /// Shared library dir for the app
    pub lib: PathBuf,
    /// Directory handling application logs
    pub log: PathBuf,
    /// Project root dir. Not meaningful for strategies like FHS or XDG
    pub project_root: Option<PathBuf>,
    /// Runtime files are similar to the cache, but don't persist between session/reboot
    /// **NOTE**: May be missing in some strategies
    pub runtime: Option<PathBuf>,
    /// Non-essential data files that should persist between sessions. E.g. logs, history
    pub state: PathBuf,
}

impl ProjectDirs {
    pub fn new(dirs: HashMap<Directory, PathBuf>) -> Self {
        Self(dirs)
    }

    pub fn empty() -> Self {
        Self(HashMap::new())
    }
}

impl From<FullProjectDirs> for ProjectDirs {
    fn from(value: FullProjectDirs) -> Self {
        let mut result = ProjectDirs::new(HashMap::from([
            (Directory::Cache, value.cache),
            (Directory::Config, value.config),
            (Directory::Data, value.data),
            (Directory::State, value.state),
            (Directory::Log, value.log),
            (Directory::Bin, value.bin),
            (Directory::Lib, value.lib),
            (Directory::Include, value.include),
        ]));

        if let Some(runtime) = value.runtime {
            result.0.insert(Directory::Runtime, runtime);
        }

        if let Some(project_root) = value.project_root {
            result.0.insert(Directory::ProjectRoot, project_root);
        }

        result
    }
}

impl TryFrom<ProjectDirs> for FullProjectDirs {
    type Error = MissingError;

    fn try_from(mut value: ProjectDirs) -> Result<Self, Self::Error> {
        let mut errors = Vec::<Directory>::new();

        for dir in [
            Directory::Cache,
            Directory::Config,
            Directory::Data,
            Directory::State,
            Directory::Log,
            Directory::Bin,
            Directory::Lib,
        ] {
            if !value.0.contains_key(&dir) {
                errors.push(dir);
            }
        }

        if errors.is_empty() {
            Ok(FullProjectDirs {
                cache: value.0.remove(&Directory::Cache).unwrap(),
                config: value.0.remove(&Directory::Config).unwrap(),
                data: value.0.remove(&Directory::Data).unwrap(),
                state: value.0.remove(&Directory::State).unwrap(),
                log: value.0.remove(&Directory::Log).unwrap(),
                bin: value.0.remove(&Directory::Bin).unwrap(),
                include: value.0.remove(&Directory::Include).unwrap(),
                lib: value.0.remove(&Directory::Lib).unwrap(),
                runtime: value.0.remove(&Directory::Runtime),
                project_root: value.0.remove(&Directory::ProjectRoot),
            })
        } else {
            Err(errors)
        }
    }
}
