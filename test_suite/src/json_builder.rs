use crate::ENV;
use serde_json::json;
use serde_json_assert::{self as sja, assert_json_matches_no_panic};
use std::{collections::HashMap, path::PathBuf};

use crate::utils::get_project_root;

pub struct TestEntry {
    pub input_file: PathBuf,
    pub output_file: PathBuf,
    pub env_file: Option<PathBuf>,
}

pub struct TestEntries(pub HashMap<String, TestEntry>);

fn get_test_files() -> TestEntries {
    let test_files_dir = get_project_root()
        .unwrap()
        .join("test_suite")
        .join("json-builder");

    let mut result = HashMap::new();

    for entry in test_files_dir.read_dir().unwrap() {
        if entry.is_err() {
            continue;
        }
        let entry = entry.unwrap();

        // Skip non file entries
        if !entry.file_type().unwrap().is_dir() {
            continue;
        }

        let testname = entry.file_name().to_str().unwrap().to_string();

        let res_entry = result.entry(testname).or_insert([None, None, None]);

        let mut output_files = vec![];
        for file in std::fs::read_dir(entry.path()).unwrap() {
            if let Ok(file) = file {
                let filename = file.file_name().to_str().unwrap().to_string();

                match filename.as_str() {
                    "input.json" => {
                        res_entry[0] = Some(file.path());
                    }
                    "output.default.json" => {
                        res_entry[1] = Some(file.path());
                    }
                    "env.json" => {
                        res_entry[2] = Some(file.path());
                    }
                    x if x.starts_with("output") && x.ends_with(".json") => {
                        output_files.push(file.path());
                    }
                    _ => {}
                }
            }
        }
        if !output_files.is_empty() {
            for potential_output in output_files {
                let run_only_on = potential_output
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .split(".")
                    .nth(1)
                    .unwrap();
                if run_only_on_matches_system(run_only_on) {
                    res_entry[1] = Some(potential_output);
                }
            }
        }
    }

    result
        .into_iter()
        .fold(TestEntries(HashMap::new()), |mut acc, (k, v)| {
            if let [Some(input), Some(output), env_file] = v {
                acc.0.insert(
                    k,
                    TestEntry {
                        input_file: input,
                        output_file: output,
                        env_file,
                    },
                );
            }
            acc
        })
}

#[test]
fn non_zero_test_files() {
    let test_files = get_test_files();
    assert!(!test_files.0.is_empty());
}

fn test_single_input_output(
    name: String,
    input_file: PathBuf,
    output_file: PathBuf,
    override_project_root: bool,
) -> bool {
    let mut builder: project_dirs_builder::Builder =
        serde_json::from_reader(std::fs::File::open(input_file).unwrap()).unwrap();

    if override_project_root {
        if let project_dirs_builder::Spec::Custom(spec) = &mut builder.spec {
            for entry in spec {
                if let project_dirs_builder::Strategy::Unix(unix_spec) = &mut entry.1.strategy {
                    if let project_dirs_builder::Unix::Custom { path, .. } = unix_spec {
                        if path.starts_with("/PROJECT_ROOT") {
                            *path = get_project_root()
                                .unwrap()
                                .join(path.strip_prefix("/PROJECT_ROOT").unwrap());
                        }
                    }
                }
            }
        }
    }

    let result = builder.build();

    let expected = serde_json::from_reader::<_, project_dirs_builder::BuilderResult>(
        std::fs::File::open(output_file).unwrap(),
    );

    assert!(
        expected.is_ok(),
        "Failed to parse expected output for {}. The generated result is:\n{}",
        name,
        serde_json::to_string_pretty(&result).unwrap()
    );

    let expected = expected.unwrap();

    let match_config = sja::Config::new(sja::CompareMode::Strict);
    let compare_result =
        assert_json_matches_no_panic(&json!(result), &json!(expected), &match_config);

    if let Err(compare_result) = compare_result {
        let compare_result = compare_result.replace("\n\n", "\n ");
        let compare_result = format!("\n {}", compare_result);

        let actual_json = serde_json::to_string_pretty(&result).unwrap();

        println!(
            "\x1b[91mFailed to match expected output for {}\x1b[0m\n\nDetected drift: {}\n\nActual (lhs):{}\n",
            name, compare_result, actual_json
        );
        false
    } else {
        true
    }
}

fn is_docker() -> bool {
    std::env::consts::OS == "linux" && std::fs::exists("/.is-docker").is_ok_and(|v| v)
}

fn run_only_on_matches_system(run_only_on: &str) -> bool {
    match run_only_on {
        "unix" | "windows" => run_only_on == std::env::consts::FAMILY,
        "never" => false,
        "unix-not-mac" => std::env::consts::FAMILY == "unix" && std::env::consts::OS != "macos",
        "docker" => is_docker(),
        _ => run_only_on == std::env::consts::OS,
    }
}

#[test]
fn test_builder() {
    let mut count = 0;
    let mut failed = 0;
    let mut is_success = true;

    for test in get_test_files().0 {
        let was_ok = if let Some(env_file) = test.1.env_file {
            let envs: HashMap<String, Option<String>> =
                serde_json::from_reader(std::fs::File::open(env_file).unwrap()).unwrap();

            if let Some(run_only_on) = envs.get("__RUN_ONLY_ON__") {
                if !run_only_on
                    .as_ref()
                    .is_some_and(|val| run_only_on_matches_system(val))
                {
                    continue;
                }
            }

            let change_dir = match envs.get("__ROOT_CHDIR__") {
                Some(Some(path)) => {
                    if path.starts_with("/PROJECT_ROOT") {
                        Some(
                            get_project_root()
                                .unwrap()
                                .join(path.strip_prefix("/PROJECT_ROOT").unwrap()),
                        )
                    } else {
                        panic!("You need to explicitly use /PROJECT_ROOT in __ROOT_CHDIR__");
                    }
                },
                _ => None,
            };

            ENV.lock().unwrap().with_env(envs, change_dir, || {
                test_single_input_output(
                    test.0,
                    test.1.input_file,
                    test.1.output_file,
                    std::env::var("__RESOLVE_PROJECT_ROOT__")
                        .is_ok_and(|v| v == "true" || v == "1"),
                )
            })
        } else {
            test_single_input_output(test.0, test.1.input_file, test.1.output_file, false)
        };

        if !was_ok {
            is_success = false;
            failed += 1;
        }
        count += 1;
    }

    assert!(
        is_success,
        "\x1b[91mBuilder test failed. Failed {} out of {}\x1b[0m",
        failed, count
    );
}
