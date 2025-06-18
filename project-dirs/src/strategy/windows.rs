use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::{Directory, Project, ProjectDirs};

#[cfg(target_os = "windows")]
use crate::FullProjectDirs;

#[cfg(target_os = "windows")]
pub(crate) mod win_only {
    use super::WindowsEnv;
    use std::os::windows::ffi::OsStringExt;
    use std::path::PathBuf;
    use std::slice;
    use windows_sys::Win32::UI::Shell;

    use std::ffi::OsString;
    use std::ffi::c_void;

    /// Method copied from dirs-sys-rs. Better to do it, than to write a mem-leaking code by hand!
    pub fn known_folder(folder_id: windows_sys::core::GUID) -> Option<PathBuf> {
        unsafe {
            let mut path_ptr: windows_sys::core::PWSTR = std::ptr::null_mut();
            let result =
                Shell::SHGetKnownFolderPath(&folder_id, 0, std::ptr::null_mut(), &mut path_ptr);
            if result == 0 {
                let len = windows_sys::Win32::Globalization::lstrlenW(path_ptr) as usize;
                let path = slice::from_raw_parts(path_ptr, len);
                let ostr: OsString = OsStringExt::from_wide(path);
                windows_sys::Win32::System::Com::CoTaskMemFree(path_ptr as *const c_void);
                Some(PathBuf::from(ostr))
            } else {
                windows_sys::Win32::System::Com::CoTaskMemFree(path_ptr as *const c_void);
                None
            }
        }
    }

    impl WindowsEnv {
        pub fn new_system() -> Self {
            Self {
                program_files: known_folder(Shell::FOLDERID_ProgramFiles),
                program_data: known_folder(Shell::FOLDERID_ProgramData),
                roaming_app_data: known_folder(Shell::FOLDERID_RoamingAppData),
                local_app_data: known_folder(Shell::FOLDERID_LocalAppData),
            }
        }
    }
}

fn join_win_path(path: &Path, project: &Project) -> PathBuf {
    path.join(&project.organization_name)
        .join(&project.application_name)
}

fn changing_data(result: &mut HashMap<Directory, PathBuf>, path: &Path, project: &Project) {
    result.insert(Directory::Data, join_win_path(path, project).join("data"));
    result.insert(Directory::Cache, join_win_path(path, project).join("cache"));
    result.insert(Directory::Runtime, join_win_path(path, project).join("tmp"));
    result.insert(Directory::State, join_win_path(path, project).join("state"));
    result.insert(Directory::Log, join_win_path(path, project).join("logs"));
}

fn static_data(result: &mut HashMap<Directory, PathBuf>, path: &Path, project: &Project) {
    result.insert(Directory::Bin, join_win_path(path, project).join("bin"));
    result.insert(
        Directory::Config,
        join_win_path(path, project).join("config"),
    );
    result.insert(
        Directory::Include,
        join_win_path(path, project).join("include"),
    );
    result.insert(Directory::Lib, join_win_path(path, project).join("lib"));
}

/// Environment variables for [`Windows`] trait.
#[derive(Debug, Clone, Default)]
pub struct WindowsEnv {
    pub program_files: Option<PathBuf>,
    pub program_data: Option<PathBuf>,
    pub roaming_app_data: Option<PathBuf>,
    pub local_app_data: Option<PathBuf>,
}

pub const PROGRAM_FILES: &str = "%ProgramFiles%";
pub const PROGRAM_DATA: &str = "%ProgramData%";
pub const ROAMING_APP_DATA: &str = "%RoamingAppData%";
pub const LOCAL_APP_DATA: &str = "%LocalAppData%";

impl WindowsEnv {
    pub fn extend_with_env(
        &mut self,
        other: impl Iterator<Item = (impl AsRef<str>, Option<impl AsRef<str>>)>,
        allow_clearing: bool,
    ) {
        for (k, v) in other {
            let pathbuf_new_value: Option<PathBuf> = v.and_then(|str_value| {
                if !str_value.as_ref().is_empty() {
                    Some(PathBuf::from(str_value.as_ref()))
                } else {
                    None
                }
            });
            let str_key: &str = k.as_ref();

            if allow_clearing || pathbuf_new_value.is_some() {
                match str_key {
                    PROGRAM_FILES => self.program_files = pathbuf_new_value,
                    PROGRAM_DATA => self.program_data = pathbuf_new_value,
                    ROAMING_APP_DATA => self.roaming_app_data = pathbuf_new_value,
                    LOCAL_APP_DATA => self.local_app_data = pathbuf_new_value,
                    _ => (),
                }
            };
        }
    }
}

/// Retrive [`ProjectDirs`] and [`FullProjectDirs`] for Windows based systems.
pub trait Windows {
    /// Returns the project directories on the current system (global installation)
    #[cfg(target_family = "windows")]
    fn windows_system(&self) -> Option<FullProjectDirs> {
        self.windows_system_with_env(WindowsEnv::new_system())
            .try_into()
            .ok()
    }

    /// Returns the project directories for the current user on the current system (user
    /// installation)
    #[cfg(target_family = "windows")]
    fn windows_user(&self) -> Option<FullProjectDirs> {
        self.windows_user_with_env(WindowsEnv::new_system())
            .try_into()
            .ok()
    }

    /// Returns the project directories for the current user on the current system. Assumes that
    /// nothing is shared across the domain (nothing is roamed)
    #[cfg(target_family = "windows")]
    fn windows_user_local(&self) -> Option<FullProjectDirs> {
        self.windows_user_local_with_env(WindowsEnv::new_system())
            .try_into()
            .ok()
    }

    /// Returns the project directories for the current user on the current system. Assumes that
    /// everything is shared across the domain
    #[cfg(target_family = "windows")]
    fn windows_user_shared(&self) -> Option<FullProjectDirs> {
        self.windows_user_shared_with_env(WindowsEnv::new_system())
            .try_into()
            .ok()
    }

    /// Returns the project directories for the given environment (using %ProgramFiles%, and
    /// %ProgramData%) variables.
    fn windows_system_with_env(&self, env: WindowsEnv) -> ProjectDirs;

    /// Returns the project directories for the given environment (using %RoamingAppData%, and
    /// %LocalAppData%) variables.
    fn windows_user_with_env(&self, env: WindowsEnv) -> ProjectDirs;

    /// Returns the project directories for the given environment (using %LocalAppData%) variables.
    fn windows_user_local_with_env(&self, env: WindowsEnv) -> ProjectDirs;

    /// Returns the project directories for the given environment (using %RoamingAppData%) variables.
    fn windows_user_shared_with_env(&self, env: WindowsEnv) -> ProjectDirs;
}

impl Windows for Project {
    fn windows_system_with_env(&self, env: WindowsEnv) -> ProjectDirs {
        let mut result = HashMap::new();

        if let Some(static_data_dir) = env.program_files {
            static_data(&mut result, &static_data_dir, self);
        }

        if let Some(changing_data_dir) = env.program_data {
            changing_data(&mut result, &changing_data_dir, self);
        }

        ProjectDirs::new(result)
    }

    fn windows_user_with_env(&self, env: WindowsEnv) -> ProjectDirs {
        let mut result = HashMap::new();

        if let Some(static_data_dir) = env.roaming_app_data {
            static_data(&mut result, &static_data_dir, self);
        }

        if let Some(changing_data_dir) = env.local_app_data {
            changing_data(&mut result, &changing_data_dir, self);
        }

        ProjectDirs::new(result)
    }

    fn windows_user_local_with_env(&self, env: WindowsEnv) -> ProjectDirs {
        let mut result = HashMap::new();
        if let Some(data_dir) = env.local_app_data {
            changing_data(&mut result, &data_dir, self);
            static_data(&mut result, &data_dir, self);
            result.insert(Directory::ProjectRoot, data_dir);
        }
        ProjectDirs::new(result)
    }

    fn windows_user_shared_with_env(&self, env: WindowsEnv) -> ProjectDirs {
        let mut result = HashMap::new();
        if let Some(data_dir) = env.roaming_app_data {
            changing_data(&mut result, &data_dir, self);
            static_data(&mut result, &data_dir, self);
            result.insert(Directory::ProjectRoot, data_dir);
        }
        ProjectDirs::new(result)
    }
}
