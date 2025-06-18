use project_dirs::{Directory, ProjectDirs};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

fn default_true() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "kebab-case")]
pub enum Fhs {
    Local,

    #[default]
    Shared,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub enum Unix {
    Pwd,
    Home,
    Binary,

    #[serde(untagged)]
    Custom {
        path: PathBuf,
        #[serde(default)]
        prefix: Option<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub enum Windows {
    /// User installation for windows
    Standard,
    /// User (local only) installation for windows
    Local,
    /// User (roamed/shared) installation for windows
    Shared,
    /// Global installation for windows
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(tag = "strategy", content = "strategy_config")]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub enum Strategy {
    /// Get local directories based on the current system
    CurrentLocal,
    /// Get user directories based on the current system
    CurrentUser,
    /// Get system directories based on the current system
    CurrentSystem,
    /// Get directories using FHS standard
    Fhs(#[serde(default)] Option<Fhs>),
    /// Get directories using XDG standard
    Xdg,
    /// Get directories using unix-style directory
    Unix(Unix),
    /// Get directories for windows
    Windows(Windows),
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub enum Filter {
    /// Return only directories that exist on the fs
    FsPresent,

    /// Return only directories that don't exist on the fs
    FsAbsent,

    /// Return only directories that exist on the fs and are NOT directories
    FsNotDir,

    /// Return only access denied directories
    FsDenied,

    /// Return everything that is not a valid dir. Negation of the FsPresent
    FsNonValidDir,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct SpecEntry {
    #[serde(flatten)]
    pub strategy: Strategy,
    #[serde(default)]
    pub directories: Vec<Directory>,
    pub filter: Option<Filter>,

    #[serde(default)]
    pub mountpoint: Option<PathBuf>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub enum Spec {
    /// Use system defaults in the [`Scoped`] format
    #[default]
    SystemDefault,

    /// Define own spec, with custom filters, mountpoints etc.
    #[serde(untagged)]
    Custom(HashMap<String, SpecEntry>),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
/// Specify env for the builder
pub struct CustomEnv {
    /// Custom env definition. Empty strings are treated as undefined by default
    #[serde(default)]
    pub env: HashMap<String, Option<String>>,

    /// Use system as a fallback
    #[serde(default = "default_true")]
    pub fallback_to_system: bool,

    /// Allow variable clearing by empty string or undefined values
    #[serde(default)]
    pub allow_variable_clearing: bool,
}

impl Default for CustomEnv {
    fn default() -> Self {
        CustomEnv {
            env: HashMap::new(),
            fallback_to_system: true,
            allow_variable_clearing: false,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Builder {
    pub qualifier: String,
    pub organization: String,
    pub application: String,

    #[serde(default)]
    pub spec: Spec,

    /// Specify env for the custom builder
    /// **NOTE**: It does only work for custom spec builders
    #[serde(default)]
    pub custom_env: CustomEnv,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct BuilderResult {
    pub application_name: String,
    pub dirs: HashMap<String, ProjectDirs>,
}

impl Builder {
    fn system_default(&self, project: &project_dirs::Project) -> HashMap<String, ProjectDirs> {
        let dirs = project.project_dirs();
        HashMap::from([
            ("local".to_string(), dirs.local),
            ("user".to_string(), dirs.user),
            ("system".to_string(), dirs.system),
        ])
    }

    pub fn process_spec_entry(
        &self,
        project: &project_dirs::Project,
        entry: &SpecEntry,
    ) -> ProjectDirs {
        use project_dirs::dir_utils::{Filter as _, Mounted as _};
        use project_dirs::strategy::fhs::Fhs as _;
        use project_dirs::strategy::unix::Unix as _;
        use project_dirs::strategy::windows::{Windows as _, WindowsEnv};
        use project_dirs::strategy::xdg::{Xdg as _, XdgEnv};

        let mut pd: ProjectDirs = match &entry.strategy {
            Strategy::CurrentLocal => project.project_dirs().local,
            Strategy::CurrentUser => project.project_dirs().user,
            Strategy::CurrentSystem => project.project_dirs().system,
            Strategy::Fhs(fhs) => match fhs {
                Some(Fhs::Local) => project.fhs_local().into(),
                Some(Fhs::Shared) | None => project.fhs().into(),
            },
            Strategy::Xdg => {
                let mut env = if self.custom_env.fallback_to_system {
                    XdgEnv::new_system()
                } else {
                    XdgEnv::default()
                };

                env.extend_with_env(
                    self.custom_env.env.iter().map(|x| (x.0, x.1.as_ref())),
                    self.custom_env.allow_variable_clearing,
                );

                if self.custom_env.fallback_to_system {
                    project
                        .xdg_with_env(env)
                        .map(ProjectDirs::from)
                        .unwrap_or(ProjectDirs::empty())
                } else {
                    project.xdg_with_env_exclude_missing(env)
                }
            }
            Strategy::Unix(unix) => match unix {
                Unix::Pwd => project
                    .unix_pwd()
                    .map(Into::into)
                    .unwrap_or(ProjectDirs::empty()),
                Unix::Home => project
                    .unix_home()
                    .map(Into::into)
                    .unwrap_or(ProjectDirs::empty()),
                Unix::Binary => project
                    .unix_binary()
                    .map(Into::into)
                    .unwrap_or(ProjectDirs::empty()),
                Unix::Custom { path, prefix } => match prefix {
                    Some(prefix) => project.unix_prefixed(path, prefix).into(),
                    None => project.unix(path).into(),
                },
            },
            Strategy::Windows(windows) => {
                #[cfg(target_os = "windows")]
                let mut env = if self.custom_env.fallback_to_system {
                    WindowsEnv::new_system()
                } else {
                    WindowsEnv::default()
                };

                #[cfg(not(target_os = "windows"))]
                let mut env = WindowsEnv::default();

                env.extend_with_env(
                    self.custom_env.env.iter().map(|x| (x.0, x.1.as_ref())),
                    self.custom_env.allow_variable_clearing,
                );

                match windows {
                    Windows::Standard => project.windows_user_with_env(env),
                    Windows::Local => project.windows_user_local_with_env(env),
                    Windows::Shared => project.windows_user_shared_with_env(env),
                    Windows::System => project.windows_system_with_env(env),
                }
            }
        };

        if let Some(filter) = &entry.filter {
            pd = match filter {
                Filter::FsPresent => pd.filter_existing_dirs(),
                Filter::FsAbsent => pd.filter_absent(),
                Filter::FsNotDir => pd.filter_non_dirs(),
                Filter::FsDenied => pd.filter_denied(),
                Filter::FsNonValidDir => pd.filter_non_valid(),
            };
        }

        if let Some(mountpoint) = &entry.mountpoint {
            pd = pd.mounted(mountpoint);
        }

        if !entry.directories.is_empty() {
            pd = ProjectDirs::new(
                pd.0.into_iter()
                    .filter(|d| entry.directories.contains(&d.0))
                    .collect(),
            );
        }

        pd
    }

    pub fn build(&self) -> BuilderResult {
        let project =
            project_dirs::Project::new(&self.qualifier, &self.organization, &self.application);

        let application_name = project.application_name().to_string();

        BuilderResult {
            application_name,
            dirs: match &self.spec {
                Spec::SystemDefault => self.system_default(&project),
                Spec::Custom(items) => items.iter().fold(HashMap::new(), |mut acc, item| {
                    acc.insert(item.0.clone(), self.process_spec_entry(&project, item.1));
                    acc
                }),
            },
        }
    }
}
