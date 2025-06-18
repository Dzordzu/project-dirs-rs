use project_dirs::{Directory, FullProjectDirs, ProjectDirs};
use project_dirs_builder::{
    Builder, BuilderResult, CustomEnv, Fhs, Filter, Spec, SpecEntry, Strategy, Unix, Windows,
};
use schemars::{Schema, schema_for};
use serde::Serialize;

#[derive(Serialize)]
struct Everything {
    directory: Schema,
    full_project_dirs: Schema,
    project_dirs: Schema,
    builder: Schema,
    builder_result: Schema,
    custom_env: Schema,
    fhs: Schema,
    filter: Schema,
    spec: Schema,
    spec_entry: Schema,
    strategy: Schema,
    unix: Schema,
    windows: Schema,
}

impl Everything {
    pub fn new() -> Self {
        Everything {
            directory: schema_for!(Directory),
            full_project_dirs: schema_for!(FullProjectDirs),
            project_dirs: schema_for!(ProjectDirs),
            builder: schema_for!(Builder),
            builder_result: schema_for!(BuilderResult),
            custom_env: schema_for!(CustomEnv),
            fhs: schema_for!(Fhs),
            filter: schema_for!(Filter),
            spec: schema_for!(Spec),
            spec_entry: schema_for!(SpecEntry),
            strategy: schema_for!(Strategy),
            unix: schema_for!(Unix),
            windows: schema_for!(Windows),
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let schema = if let Some(variant) = args.get(1) {
        let e = serde_json::to_value(&Everything::new()).unwrap();
        let v = e.as_object().unwrap().get(variant);

        if v.is_none() {
            eprintln!("ERR: Unknown variant {}", variant);
            std::process::exit(1);
        }

        v.unwrap().clone()
    } else {
        serde_json::to_value(&Everything::new()).unwrap().clone()
    };

    println!("{}", serde_json::to_string_pretty(&schema).unwrap());
}
