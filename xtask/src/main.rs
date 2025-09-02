mod builds;
mod packaging;
mod testing;
mod utils;

use clap::{Args, Parser, Subcommand, ValueEnum};
use xshell::Shell;

pub const TESTS_RELATIVE_PARENT: &str = "test_suite";
pub const TESTS_DIR: &str = "example-project-dirs";

#[derive(Debug, Clone, Parser)]
pub struct Cli {
    #[command(subcommand)]
    cmd: Command,
}

#[derive(Debug, Clone, Args)]
pub struct Lint {
    /// Force clippy to fix errors
    #[clap(short = 'F', long, default_value = "false")]
    force: bool,

    /// Try to fix errors
    #[clap(short, long, default_value = "false")]
    fix: bool,

    /// Warning as errors. More strict
    #[clap(short, long, default_value = "false")]
    no_mercy: bool,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Coverage {
    Html,
    Cmd,
    No,
}

#[derive(Debug, Clone, Args)]
pub struct Test {
    #[clap(short, long, default_value = "cmd")]
    coverage: Coverage,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Command {
    /// Build dev verions of the projects
    Dev,
    /// Run test
    Test(Test),
    /// Run fmt and clippy
    Lint(Lint),
    /// Generate json-schema
    JsonSchema,
    /// Operate on docker
    DockerBuild,
}

fn main() {
    let cli = Cli::parse();

    let sh = Shell::new().unwrap();

    match cli.cmd {
        Command::JsonSchema => {
            let builder_output = xshell::cmd!(sh, "cargo r -p xtask-json -- builder")
                .output()
                .unwrap();
            let builder_result_output = xshell::cmd!(sh, "cargo r -p xtask-json -- builder_result")
                .output()
                .unwrap();

            let builder = std::str::from_utf8(&builder_output.stdout).unwrap();
            let builder_result = std::str::from_utf8(&builder_result_output.stdout).unwrap();

            let project_root = crate::utils::get_project_root().unwrap();

            std::fs::write(project_root.join("builder.schema.json"), builder).unwrap();

            std::fs::write(
                project_root.join("builder_result.schema.json"),
                builder_result,
            )
            .unwrap();
        }
        Command::Lint(lint) => {
            let mut fmt_cmd = vec!["fmt", "--all"];
            let mut clippy_cmd = vec!["clippy", "--workspace"];

            if lint.fix {
                clippy_cmd.push("--fix");
            } else {
                fmt_cmd.push("--check");
            }

            if lint.force {
                clippy_cmd.extend(["--allow-dirty", "--allow-staged"]);
            }

            if lint.no_mercy {
                clippy_cmd.extend(["--features", "pedantic"]);
            }

            xshell::cmd!(sh, "cargo {fmt_cmd...}").run().unwrap();
            xshell::cmd!(sh, "cargo {clippy_cmd...}").run().unwrap();
        }
        Command::Dev => {
            xshell::cmd!(sh, "cargo build --workspace").run().unwrap();
        }
        Command::DockerBuild => {
            xshell::cmd!(sh, "docker build . -t project-dirs-bin")
                .run()
                .unwrap();
        }
        Command::Test(test) => {
            let project_root = crate::utils::get_project_root().unwrap();
            setup_test_dirs(&project_root);
            xshell::cmd!(sh, "cargo test --doc --workspace")
                .run()
                .unwrap();
            xshell::cmd!(sh, "cargo test --examples").run().unwrap();
            match test.coverage {
                Coverage::Cmd => {
                    xshell::cmd!(sh, "cargo llvm-cov test").run().unwrap();
                }
                Coverage::Html => {
                    xshell::cmd!(sh, "cargo llvm-cov test --html")
                        .run()
                        .unwrap();
                }
                Coverage::No => {
                    xshell::cmd!(sh, "cargo test").run().unwrap();
                }
            }

            destroy_test_dirs(&project_root);
        }
    }
}

fn destroy_test_dirs(project_root: &std::path::Path) {
    let root = project_root.join(TESTS_RELATIVE_PARENT).join(TESTS_DIR);
    std::fs::remove_dir_all(root).unwrap();
}

fn setup_test_dirs(project_root: &std::path::Path) {
    let root = project_root.join(TESTS_RELATIVE_PARENT).join(TESTS_DIR);
    std::fs::create_dir_all(root.join("bin")).unwrap();
    std::fs::create_dir_all(root.join("state")).unwrap();
    std::fs::create_dir_all(root.join("tmp")).unwrap();
    std::fs::create_dir_all(root.join("data")).unwrap();
    std::fs::write(root.join("log"), "").unwrap(); // log is not a directory,
    // but a file
}
