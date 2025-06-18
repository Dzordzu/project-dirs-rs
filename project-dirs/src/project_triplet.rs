pub fn name_cleanup(name: &str) -> String {
    #[cfg(target_family = "unix")]
    {
        unix_name_cleanup(name)
    }
    #[cfg(target_family = "windows")]
    {
        windows_name_cleanup(name)
    }
}

pub fn unix_name_cleanup(name: &str) -> String {
    name.to_ascii_lowercase()
        .replace(" ", "-")
        .replace("_", "-")
        .replace(".", "-")
        .replace("\\", "-")
        .replace("/", "-")
        .replace("@", "-")
        .replace("#", "-")
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || c == &'-')
        .collect()
}

pub fn windows_name_cleanup(name: &str) -> String {
    name.replace("\\", "-")
        .replace("/", "-")
        .replace("@", "-")
        .replace("#", "-")
        .chars()
        .filter(|c| {
            c.is_ascii_alphanumeric()
                || c.is_whitespace()
                || *c == '-'
                || *c == '.'
                || *c == '_'
                || *c == '('
                || *c == ')'
        })
        .collect()
}

pub fn qualifier_cleanup(qualifier: &str) -> String {
    qualifier
        .to_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || c.is_whitespace() || *c == '-' || *c == '.')
        .collect()
}
