#![cfg(test)]
pub mod env_mock;
pub mod json_builder;
pub mod utils;

use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{LazyLock, Mutex},
};

use env_mock::EnvMock;
use project_dirs::{Directory, FullProjectDirs, Project, ProjectDirs};
use utils::*;

fn create_env_mock() -> Mutex<EnvMock> {
    Mutex::new(EnvMock::begin())
}

static ENV: LazyLock<Mutex<EnvMock>> = LazyLock::new(create_env_mock);

#[test]
fn project_name_unix() {
    for (orig, expected) in [
        (A, "funny-bunny-v-2137-0"),
        ("my-proj", "my-proj"),
        ("żołądź project", "od-project"),
        ("123-my_proj", "123-my-proj"),
    ] {
        assert_eq!(Project::new(Q, O, orig).application_name_unix(), expected);
    }
}

#[test]
fn project_name_windows() {
    for (orig, expected) in [
        (A, "funny-bunny v.2137_0"),
        ("my-proj", "my-proj"),
        ("żołądź project", "od project"),
        ("123-my_proj", "123-my_proj"),
    ] {
        assert_eq!(
            Project::new(Q, O, orig).application_name_windows(),
            expected
        );
    }
}

fn any_parent_is(path: &std::path::Path, expected_parent: &std::path::Path) -> bool {
    let mut is_in_target = false;
    let mut maybe_dir = Some(path);
    while let Some(dir) = maybe_dir {
        if dir == expected_parent {
            is_in_target = true;
            break;
        }
        maybe_dir = dir.parent();
    }

    is_in_target
}

#[test]
fn unix_binary_in_target_dir() {
    use project_dirs::strategy::unix::Unix as _;
    let project = Project::new(Q, O, A);
    let dirs: project_dirs::ProjectDirs = project.unix_binary().unwrap().into();

    let project_root = crate::utils::get_project_root().unwrap();
    let expected_path = project_root.join("target");
    for (_, dir) in dirs.0 {
        assert!(
            any_parent_is(&dir, &expected_path),
            "{dir:?} is not in {expected_path:?}"
        );
    }
}

#[test]
fn unix_env_checks() {
    use project_dirs::strategy::xdg::XdgEnv;

    let default_pb = Some(PathBuf::from("/a/b"));

    let mut env = XdgEnv {
        home_dir: None,
        xdg_data_home: default_pb.clone(),
        xdg_config_home: default_pb.clone(),
        xdg_state_home: default_pb.clone(),
        xdg_cache_home: default_pb.clone(),
        xdg_runtime_dir: None,
    };

    assert!(env.is_ok());

    env.xdg_data_home = None;
    assert!(!env.is_ok());

    env.xdg_runtime_dir = default_pb.clone();
    assert!(!env.is_ok());

    env.home_dir = default_pb;
    assert!(env.is_ok());

    env.xdg_state_home = None;
    env.xdg_cache_home = None;
    env.xdg_config_home = None;
    env.home_dir = None;
    assert!(!env.is_ok());
}

#[test]
fn unix_pwd() {
    use project_dirs::strategy::unix::Unix as _;
    let project_root = get_project_root().unwrap();

    let mut env = ENV.lock().unwrap();
    env.chdir(&project_root);

    // Make cure we've changed pwd
    let pwd = std::env::current_dir().unwrap();
    assert_eq!(pwd, project_root);

    let project = Project::new(Q, O, A);
    let dirs: project_dirs::ProjectDirs = project.unix_pwd().unwrap().into();

    for (_, dir) in dirs.0 {
        assert!(any_parent_is(&dir, &project_root));
    }

    env.restore();
}

#[test]
#[cfg(target_family = "unix")]
fn unix_home() {
    use project_dirs::strategy::unix::Unix as _;
    let project = Project::new(Q, O, A);
    let dirs: project_dirs::ProjectDirs = project.unix_home().unwrap().into();

    if let Ok(home_dir) = std::env::var("HOME") {
        let expected_path = std::path::PathBuf::from(home_dir);
        for (_, dir) in dirs.0 {
            assert!(
                any_parent_is(&dir, &expected_path),
                "{dir:?} is not in {expected_path:?}"
            );
        }
    } else {
        assert!(false, "HOME is not set");
    }
}

#[test]
fn full_project_dirs_convert() {
    let full_project_dirs_orig = FullProjectDirs {
        cache: "cache".into(),
        data: "data".into(),
        log: "log".into(),
        runtime: Some("runtime".into()),
        state: "state".into(),
        bin: "bin".into(),
        config: "config".into(),
        include: "include".into(),
        lib: "lib".into(),
        project_root: Some("project_root".into()),
    };

    let project_dirs: ProjectDirs = full_project_dirs_orig.clone().into();
    let full_project_dirs: FullProjectDirs = project_dirs.try_into().unwrap();
    assert_eq!(full_project_dirs, full_project_dirs_orig);

    let partial_project_dirs = ProjectDirs::new(HashMap::from([
        (Directory::Config, "config".into()),
        (Directory::Data, "data".into()),
    ]));

    let failed_full_project_dirs: Result<FullProjectDirs, _> = partial_project_dirs.try_into();
    assert!(failed_full_project_dirs.is_err());

    let mut failed_full_project_dirs = failed_full_project_dirs.unwrap_err();
    let mut failed_dirs = vec![
        Directory::Cache,
        Directory::Log,
        Directory::Bin,
        Directory::Lib,
        Directory::State,
    ];
    failed_full_project_dirs.sort();
    failed_dirs.sort();

    assert_eq!(failed_full_project_dirs, failed_dirs);
}

#[test]
fn directories_minor_utils() {
    let pd = ProjectDirs::new(HashMap::from([(Directory::Cache, "cache".into())]));

    assert_eq!(pd.get(&Directory::Cache), pd.0.get(&Directory::Cache));
    assert!(ProjectDirs::empty().0.is_empty());
}

#[test]
#[cfg(not(windows))]
fn triplet() {
    let q = "123asd_&&-.supoer.comą";
    let project = Project::new(q, O, A);
    assert_eq!(project.qualifier(), "123asd-.supoer.com");

    assert_eq!(project.organization_name(), "my-org-corp");
    assert_eq!(project.organization_name_windows(), O);
    assert_eq!(project.application_name(), "funny-bunny-v-2137-0");
    assert_eq!(project.application_name_macos(), "funny-bunny-v-2137-0");
    assert_eq!(project.application_name_windows(), A);
}

#[test]
#[cfg(windows)]
fn triplet() {
    let q = "123asd_&&-.supoer.comą";
    let project = Project::new(q, O, A);
    assert_eq!(project.qualifier(), "123asd-.supoer.com");

    assert_eq!(project.organization_name(), "my-org Corp");
    assert_eq!(project.organization_name_windows(), O);
    assert_eq!(project.application_name(), "funny-bunny v.2137_0");
    assert_eq!(project.application_name_macos(), "funny-bunny-v-2137-0");
    assert_eq!(project.application_name_windows(), A);
}

#[test]
#[cfg(all(target_family = "unix", not(target_os = "macos")))]
fn unix_systems_project_dirs() {
    use project_dirs::strategy::fhs::Fhs as _;
    use project_dirs::strategy::unix::Unix as _;
    use project_dirs::strategy::xdg::Xdg as _;

    let project = Project::new(Q, O, A);
    {
        let _env = ENV.lock().unwrap();
        let dirs = project.project_dirs();
        assert_eq!(dirs.system, project.fhs().into());
        assert_eq!(dirs.local, project.unix_pwd().unwrap().into()); // race condition in pwd
        assert_eq!(dirs.user, project.xdg().unwrap().into()); // race condition in user
    }
}

#[test]
fn xdg_special_dirs() {
    use crate::env_mock::NONE_CHDIR;
    use project_dirs::strategy::xdg::*;
    {
        let mut env = ENV.lock().unwrap();
        env.with_env(
            vec![
                ("XDG_DATA_DIRS".to_string(), Some("/a/b:/e/f".to_string())),
                ("XDG_CONFIG_DIRS".to_string(), Some("/x/y:/z".to_string())),
            ],
            NONE_CHDIR,
            || {
                assert_eq!(
                    xdg_data_dirs(),
                    vec![PathBuf::from("/a/b"), PathBuf::from("/e/f")]
                );

                assert_eq!(
                    xdg_config_dirs(),
                    vec![PathBuf::from("/x/y"), PathBuf::from("/z")]
                );
            },
        );

        env.with_env(
            vec![
                ("XDG_DATA_DIRS".to_string(), None),
                ("XDG_CONFIG_DIRS".to_string(), None),
            ],
            NONE_CHDIR,
            || {
                assert_eq!(xdg_config_dirs(), vec![PathBuf::from("/etc/xdg")]);
                assert_eq!(
                    xdg_data_dirs(),
                    vec![
                        PathBuf::from("/usr/local/share"),
                        PathBuf::from("/usr/share")
                    ]
                );
            },
        );
    }
}

#[test]
fn builder_system_defaults() {
    use project_dirs_builder::{Builder, CustomEnv, Spec, SpecEntry, Strategy};

    let builder = Builder {
        spec: Spec::SystemDefault,
        qualifier: Q.to_string(),
        organization: O.to_string(),
        application: A.to_string(),
        custom_env: CustomEnv::default(),
    };

    let built = builder.build();

    let builder_splitted = project_dirs_builder::Builder {
        spec: Spec::Custom(HashMap::from([
            (
                "user".to_string(),
                SpecEntry {
                    strategy: Strategy::CurrentUser,
                    directories: Vec::new(),
                    filter: None,
                    mountpoint: None,
                },
            ),
            (
                "local".to_string(),
                SpecEntry {
                    strategy: Strategy::CurrentLocal,
                    directories: Vec::new(),
                    filter: None,
                    mountpoint: None,
                },
            ),
            (
                "system".to_string(),
                SpecEntry {
                    strategy: Strategy::CurrentSystem,
                    directories: Vec::new(),
                    filter: None,
                    mountpoint: None,
                },
            ),
        ])),
        qualifier: Q.to_string(),
        organization: O.to_string(),
        application: A.to_string(),
        custom_env: CustomEnv::default(),
    };

    let built_splitted = builder_splitted.build();

    assert_eq!(built, built_splitted);
    assert!(built.dirs.get("system").is_some());
    assert!(built.dirs.get("user").is_some());
    assert!(built.dirs.get("local").is_some());
}
