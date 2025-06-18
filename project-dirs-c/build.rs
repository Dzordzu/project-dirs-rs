extern crate cbindgen;

use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let config = cbindgen::Config {
        language: cbindgen::Language::C,
        enumeration: cbindgen::EnumConfig{
            prefix_with_name: true,
            ..Default::default()
        },
        ..Default::default()
    };

    cbindgen::Builder::new()
      .with_config(config)
      .with_crate(crate_dir)
      .with_item_prefix("project_dirs__")
      .with_language(cbindgen::Language::C)
      .generate()
      .expect("Unable to generate bindings")
      .write_to_file("project-dirs-c.h");
}
