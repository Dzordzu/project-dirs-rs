use std::{collections::HashMap, path::PathBuf};

use crate::{Directory, FullProjectDirs, Project, ProjectDirs};

/// Environment variables for [`Xdg`] trait
#[derive(Debug, Clone, Default)]
pub struct XdgEnv {
    pub home_dir: Option<PathBuf>,
    pub xdg_data_home: Option<PathBuf>,
    pub xdg_config_home: Option<PathBuf>,
    pub xdg_state_home: Option<PathBuf>,
    pub xdg_cache_home: Option<PathBuf>,
    pub xdg_runtime_dir: Option<PathBuf>,
}

fn xdg_variable(varname: &str) -> Option<PathBuf> {
    let maybe_str_val = std::env::var(varname).ok();

    maybe_str_val.and_then(|str_val| {
        if str_val.is_empty() {
            None
        } else {
            Some(PathBuf::from(str_val))
        }
    })
}

pub const HOME: &str = "HOME";
pub const XDG_DATA_HOME: &str = "XDG_DATA_HOME";
pub const XDG_CONFIG_HOME: &str = "XDG_CONFIG_HOME";
pub const XDG_STATE_HOME: &str = "XDG_STATE_HOME";
pub const XDG_CACHE_HOME: &str = "XDG_CACHE_HOME";
pub const XDG_RUNTIME_DIR: &str = "XDG_RUNTIME_DIR";
pub const XDG_DATA_DIRS: &str = "XDG_DATA_DIRS";
pub const XDG_CONFIG_DIRS: &str = "XDG_CONFIG_DIRS";

impl XdgEnv {
    pub fn new_system() -> Self {
        Self {
            home_dir: crate::dir_utils::home_dir(),
            xdg_data_home: xdg_variable(XDG_DATA_HOME),
            xdg_config_home: xdg_variable(XDG_CONFIG_HOME),
            xdg_state_home: xdg_variable(XDG_STATE_HOME),
            xdg_cache_home: xdg_variable(XDG_CACHE_HOME),
            xdg_runtime_dir: xdg_variable(XDG_RUNTIME_DIR),
        }
    }

    pub fn extend_with_env(
        &mut self,
        other: impl Iterator<Item = (impl AsRef<str>, Option<impl AsRef<str>>)>,
        allow_clearing: bool,
    ) {
        for (k, v) in other {
            let pathbuf_vaule: Option<PathBuf> = v.and_then(|str_value| {
                if !str_value.as_ref().is_empty() {
                    Some(PathBuf::from(str_value.as_ref()))
                } else {
                    None
                }
            });
            let str_key: &str = k.as_ref();

            if allow_clearing || pathbuf_vaule.is_some() {
                match str_key {
                    XDG_DATA_HOME => self.xdg_data_home = pathbuf_vaule,
                    XDG_CONFIG_HOME => self.xdg_config_home = pathbuf_vaule,
                    XDG_STATE_HOME => self.xdg_state_home = pathbuf_vaule,
                    XDG_CACHE_HOME => self.xdg_cache_home = pathbuf_vaule,
                    XDG_RUNTIME_DIR => self.xdg_runtime_dir = pathbuf_vaule,
                    HOME => self.home_dir = pathbuf_vaule,
                    _ => (),
                }
            };
        }
    }

    pub fn is_ok(&self) -> bool {
        self.home_dir.is_some()
            || (self.xdg_data_home.is_some()
                && self.xdg_config_home.is_some()
                && self.xdg_state_home.is_some()
                && self.xdg_cache_home.is_some())
    }
}

/// Read XDG_DATA_DIRS to the vector
pub fn xdg_data_dirs() -> Vec<PathBuf> {
    std::env::var(XDG_DATA_DIRS)
        .map(|string| string.split(':').map(PathBuf::from).collect())
        .unwrap_or(vec![
            PathBuf::from("/usr/local/share"),
            PathBuf::from("/usr/share"),
        ])
}

/// Read XDG_CONFIG_DIRS to the vector
pub fn xdg_config_dirs() -> Vec<PathBuf> {
    std::env::var(XDG_CONFIG_DIRS)
        .map(|string| string.split(':').map(PathBuf::from).collect())
        .unwrap_or(vec![PathBuf::from("/etc/xdg")])
}

/// Error for some of the [`Xdg`] trait methods
#[derive(Debug, Eq, PartialEq)]
pub enum XdgError {
    /// $HOME cannot be resolved and at least one of the XDG variables is missing
    UnresolvedHomeDir,
}

/// Retrive [`ProjectDirs`] and [`FullProjectDirs`] using XDG Base Directories standard
pub trait Xdg {
    /// Retrive [`ProjectDirs`] from XDG variables for the custom env. Do not use fallback to
    /// stanard directories.
    fn xdg_with_env_exclude_missing(&self, env: XdgEnv) -> ProjectDirs;

    /// Tries to retrive [`FullProjectDirs`] from XDG variables. May return error in case of
    /// missing home fallback
    fn xdg_with_env(&self, env: XdgEnv) -> Result<FullProjectDirs, XdgError>;

    /// Retrive [`FullProjectDirs`] from XDG variables. Variables are resolved from the system.
    fn xdg(&self) -> Result<FullProjectDirs, XdgError> {
        self.xdg_with_env(XdgEnv::new_system())
    }
}

impl Xdg for Project {
    fn xdg_with_env_exclude_missing(&self, env: XdgEnv) -> ProjectDirs {
        let bin_dir = env.home_dir.map(|p| p.join(".local").join("bin"));
        let cache_dir = env.xdg_cache_home.map(|p| p.join(&self.application_name));
        let config_dir = env.xdg_config_home.map(|p| p.join(&self.application_name));
        let data_dir = env.xdg_data_home.map(|p| p.join(&self.application_name));
        let include_dir = data_dir.as_ref().map(|p| p.join("include"));
        let lib_dir = data_dir.as_ref().map(|p| p.join("lib"));
        let log_dir = env
            .xdg_state_home
            .as_ref()
            .map(|p| p.join(&self.application_name).join("log"));
        let runtime_dir = env.xdg_runtime_dir.map(|p| p.join(&self.application_name));
        let state_dir = env.xdg_state_home.map(|p| p.join(&self.application_name));

        let pd: HashMap<Directory, PathBuf> = [
            (Directory::Bin, bin_dir),
            (Directory::Cache, cache_dir),
            (Directory::Config, config_dir),
            (Directory::Data, data_dir),
            (Directory::Include, include_dir),
            (Directory::Lib, lib_dir),
            (Directory::Log, log_dir),
            (Directory::Runtime, runtime_dir),
            (Directory::State, state_dir),
        ]
        .into_iter()
        .filter_map(|(k, v)| v.map(|v| (k, v)))
        .collect();

        ProjectDirs::new(pd)
    }

    fn xdg_with_env(&self, env: XdgEnv) -> Result<FullProjectDirs, XdgError> {
        let home_dir = env.home_dir.clone();
        let mut pd = self.xdg_with_env_exclude_missing(env);

        if let Some(home_dir) = home_dir {
            let share_dir = home_dir.join(".local").join("share");
            let state_dir = home_dir.join(".local").join("state");

            pd.0.entry(Directory::Cache)
                .or_insert(home_dir.join(".cache").join(&self.application_name));

            pd.0.entry(Directory::Config)
                .or_insert(home_dir.join(".config").join(&self.application_name));

            pd.0.entry(Directory::Include)
                .or_insert(share_dir.join(&self.application_name).join("include"));

            pd.0.entry(Directory::Lib)
                .or_insert(share_dir.join(&self.application_name).join("lib"));

            pd.0.entry(Directory::Log)
                .or_insert(state_dir.join(&self.application_name).join("log"));

            pd.0.entry(Directory::State)
                .or_insert(state_dir.join(&self.application_name));

            pd.0.entry(Directory::Data)
                .or_insert(share_dir.join(&self.application_name));

            pd.try_into().map_err(|e| {
                eprintln!("Missing: {e:?}");
                unreachable!(
                    "XDG failed despite of having HOME set. This should never happen and IS A BUG."
                )
            })
        } else {
            // bin_dir always depends on the home dir
            Err(XdgError::UnresolvedHomeDir)
        }
    }
}
