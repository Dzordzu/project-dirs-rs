use std::{
    env,
    fs::read_dir,
    io::{self, ErrorKind},
    path::PathBuf,
};

pub const Q: &str = "com";
pub const O: &str = "my-org Corp";
pub const A: &str = "funny-bunny v.2137_0";

pub fn get_project_root() -> io::Result<PathBuf> {
    let path = env::current_dir()?;
    let path_ancestors = path.as_path().ancestors();

    for p in path_ancestors {
        let has_cargo = read_dir(p)?.any(|p| p.unwrap().file_name() == *"Cargo.lock");
        if has_cargo {
            return Ok(PathBuf::from(p));
        }
    }
    Err(io::Error::new(
        ErrorKind::NotFound,
        "Ran out of places to find Cargo.toml",
    ))
}
