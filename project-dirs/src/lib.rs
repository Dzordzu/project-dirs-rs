mod proj_dirs;
mod project_triplet;

/// Utility functions and traits for project directories
pub mod dir_utils;

/// Ways of retrieving project directories
pub mod strategy;

pub use proj_dirs::{FullProjectDirs, MissingError, ProjectDirs};

/// Definition of the project essentials. Allows to retrive project directories
pub struct Project {
    /// NOTE: You should rather use qualifier_value
    _orig_qualifier: String,
    /// NOTE: You should rather use organization_name
    _orig_organization: String,
    /// NOTE: You should rather use application_name
    _orig_application: String,

    qualifier_value: String,
    organization_name: String,
    application_name: String,
}

/// Purpose of directory existence. Ex. Bin, Config, Cache etc.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "strum", derive(strum::Display, strum::EnumString))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub enum Directory {
    /// Binaries directory. This is where the project executable(s) is/are located
    Bin,
    /// Non-essential data, usually used to speed up the application
    Cache,
    /// You can store there conf.d dir and other config files
    Config,
    /// Essential files for application like db files, cross-session data etc.
    Data,
    /// C/C++ headers files. Should include files like lib.h, lib.hpp or lib.inc
    Include,
    /// Shared library files. Should include files like lib.a, lib.so, lib.dylib or lib.dll
    Lib,
    /// Application logs. Usually subdir of the state
    Log,
    /// Root directory of the project. Has meaning only for the some strategies
    ProjectRoot,
    /// Runtime files are similar to the cache, but don't persist between session/reboot
    Runtime,
    /// Non-essential data files that should persist between sessions. E.g. logs, history
    State,
}

/// Project directories gathered by scope: user, system and local (pwd)
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Scoped {
    pub user: ProjectDirs,
    pub system: ProjectDirs,
    pub local: ProjectDirs,
}

impl Project {
    pub fn new(qualifier: &str, organization: &str, application: &str) -> Self {
        Self {
            _orig_qualifier: qualifier.to_string(),
            _orig_organization: organization.to_string(),
            _orig_application: application.to_string(),
            qualifier_value: project_triplet::qualifier_cleanup(qualifier),
            organization_name: project_triplet::name_cleanup(organization),
            application_name: project_triplet::name_cleanup(application),
        }
    }

    /// Get application name for UNIX-like systems (excluding mac)
    pub fn application_name_unix(&self) -> String {
        project_triplet::unix_name_cleanup(&self._orig_application)
    }

    /// Get application name for Windows systems
    pub fn application_name_windows(&self) -> String {
        project_triplet::windows_name_cleanup(&self._orig_application)
    }

    /// Get application name for macOS
    pub fn application_name_macos(&self) -> String {
        project_triplet::unix_name_cleanup(&self._orig_application)
    }

    /// Get organization name for Windows
    pub fn organization_name_windows(&self) -> String {
        project_triplet::windows_name_cleanup(&self._orig_organization)
    }

    /// Get organization_name calculated for the current system
    pub fn organization_name(&self) -> &str {
        &self.organization_name
    }

    /// Get application_name calculated for the current system
    pub fn application_name(&self) -> &str {
        &self.application_name
    }

    /// Get qualifier calculated for the current system
    pub fn qualifier(&self) -> &str {
        &self.qualifier_value
    }

    #[cfg(all(target_family = "unix", not(target_os = "macos")))]
    fn unix_project_dirs(&self) -> Scoped {
        use crate::strategy::fhs::Fhs;
        use crate::strategy::unix::Unix;
        use crate::strategy::xdg::{Xdg, XdgEnv};

        Scoped {
            user: self
                .xdg_with_env(XdgEnv::new_system())
                .map(|x| x.into())
                .unwrap_or(ProjectDirs::empty()),
            system: self.fhs().into(),
            local: self
                .unix_pwd()
                .map(Into::into)
                .unwrap_or(ProjectDirs::empty()),
        }
    }

    #[cfg(target_os = "windows")]
    fn windows_project_dirs(&self) -> Scoped {
        use crate::strategy::unix::Unix;

        use crate::strategy::windows::{Windows, WindowsEnv};
        let windows_env = WindowsEnv::new_system();
        Scoped {
            user: self.windows_user_with_env(windows_env.clone()),
            system: self.windows_system_with_env(windows_env),
            local: self
                .unix_pwd()
                .map(Into::into)
                .unwrap_or(ProjectDirs::empty()),
        }
    }

    pub fn project_dirs(&self) -> Scoped {
        #[cfg(all(target_family = "unix", not(target_os = "macos")))]
        {
            self.unix_project_dirs()
        }
        #[cfg(all(target_family = "unix", target_os = "macos"))]
        {
            todo!()
        }
        #[cfg(target_family = "windows")]
        {
            self.windows_project_dirs()
        }
    }
}
