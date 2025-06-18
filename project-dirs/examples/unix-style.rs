use project_dirs::Project;
use project_dirs::strategy::unix::Unix;
use std::path::PathBuf;

#[test]
pub fn main_test() {
    main()
}

pub fn main() {
    let project = Project::new("qualifier", "org", "My Magic Super/App");

    let pn_unix = project.application_name_unix();
    assert_eq!(pn_unix, "my-magic-super-app");

    let my_path = PathBuf::from("/opt/my_dir");

    #[cfg(not(windows))]
    {
        assert_eq!(
            project.unix_prefixed(&my_path, ".").log,
            PathBuf::from("/opt/my_dir/.my-magic-super-app/log")
        );
        assert_eq!(
            project.unix(&my_path).log,
            PathBuf::from("/opt/my_dir/my-magic-super-app/log")
        );
    }

    #[cfg(windows)]
    {
        assert_eq!(
            project.unix_prefixed(&my_path, ".").log,
            PathBuf::from("/opt/my_dir/.My Magic Super-App/log")
        );
        assert_eq!(
            project.unix(&my_path).log,
            PathBuf::from("/opt/my_dir/My Magic Super-App/log")
        );
    }

    // Should return something like /home/user/.my-magic-super-app
    let home_unix = project.unix_home();
    println!("home unix dir: {:#?}", home_unix);

    // Ex. /my/current/dir/.my-magic-super-app
    let pwd_unix = project.unix_pwd();
    println!("pwd unix dir: {:#?}", pwd_unix);
}
