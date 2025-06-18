use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

pub struct EnvMock {
    env: HashMap<String, Option<String>>,
    orig_dir: Option<PathBuf>,
}

pub const NONE_CHDIR: Option<&str> = None;

impl EnvMock {
    pub fn begin() -> Self {
        Self {
            env: HashMap::new(),
            orig_dir: None,
        }
    }

    pub fn env(
        &mut self,
        key: &str,
        value: Option<impl AsRef<str>>,
    ) -> Result<(), std::env::VarError> {
        let old_value = match std::env::var(key.to_uppercase()) {
            Ok(value) => Some(value),
            Err(std::env::VarError::NotPresent) => None,
            Err(e) => return Err(e),
        };

        self.env.insert(key.to_string(), old_value);

        unsafe {
            if let Some(value) = value {
                std::env::set_var(key, value.as_ref());
            } else {
                std::env::remove_var(key);
            }
        }

        Ok(())
    }

    pub fn restore(&mut self) {
        for (key, value) in &self.env {
            if let Some(value) = value {
                unsafe {
                    std::env::set_var(key, value);
                }
            } else {
                unsafe {
                    std::env::remove_var(key);
                }
            }
        }
        self.env.clear();

        if let Some(orig_dir) = self.orig_dir.take() {
            std::env::set_current_dir(orig_dir).unwrap();
        }
    }

    /// Change the current directory
    pub fn chdir(&mut self, dir: impl AsRef<Path>) -> bool {
        let orig_dir = std::env::current_dir().ok();
        if orig_dir.is_none() {
            panic!("Could not determine current directory");
        }
        if std::env::set_current_dir(dir.as_ref()).is_ok() {
            self.orig_dir = orig_dir;
            true
        } else {
            false
        }
    }

    pub fn with_env<F: FnOnce() -> R, R>(
        &mut self,
        envs: impl IntoIterator<Item = (String, Option<String>)>,
        chdir: Option<impl AsRef<Path>>,
        f: F,
    ) -> R {
        if let Some(chdir) = chdir {
            self.chdir(chdir);
        }
        for value in envs {
            self.env(&value.0, value.1.as_ref())
                .inspect_err(|err| {
                    if let Some(to_set) = &value.1 {
                        let to_set_value: &str = to_set.as_ref();
                        eprintln!("Failed to set {}={}", value.0, to_set_value);
                    } else {
                        eprintln!("Failed to unset {}", value.0);
                    }
                    eprintln!("Error: {:?}", err);
                })
                .ok();
        }
        let result = f();
        self.restore();
        result
    }
}
