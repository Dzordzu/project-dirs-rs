use project_dirs::{Directory, Project};

pub fn main() {
    // Yep. That's all you need in most cases
    let log_dir = Project::new("org", "My Super Company", "My app")
        .project_dirs()
        .user
        .get(&Directory::Log)
        .unwrap()
        .clone();

    println!("log dir: {:#?}", log_dir);
}
