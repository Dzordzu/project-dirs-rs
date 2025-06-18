use project_dirs::Project;
use project_dirs::strategy::fhs::Fhs;
use std::path::PathBuf;

#[test]
fn main_test() {
    main()
}

pub fn main() {
    let project = Project::new("qualifier", "org", "My Magic Super/App");

    let pn_unix = project.application_name_unix();
    assert_eq!(pn_unix, "my-magic-super-app");

    assert_eq!(
        project.fhs().log,
        PathBuf::from("/var/log/my-magic-super-app")
    );
}
