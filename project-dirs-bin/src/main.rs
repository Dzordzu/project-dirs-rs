use clap::Parser;
use std::path::PathBuf;

const BUILDER_SCHEMA: &str = include_str!("../../builder.schema.json");

#[derive(Parser)]
pub struct Cli {
    /// Project name
    #[arg(short, long)]
    pub application: Option<String>,

    /// Organization name
    #[arg(short, long)]
    pub organization_name: Option<String>,

    /// Domain name
    #[arg(short, long)]
    pub qualifier: Option<String>,

    /// Use manifest - empty triplet and system default directories without special env
    #[arg(
        short = 'd',
        long,
        group = "manifest",
        requires("application"),
        requires("organization_name"),
        requires("qualifier")
    )]
    pub use_default_manifest: bool,

    /// File to read to get project dirs
    #[arg(group = "manifest")]
    pub manifest_file: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();

    let content = if cli.use_default_manifest {
        serde_json::to_string(&project_dirs_builder::Builder {
            qualifier: String::new(),
            organization: String::new(),
            application: String::new(),
            custom_env: Default::default(),
            spec: project_dirs_builder::Spec::SystemDefault,
        })
        .unwrap()
    } else if let Some(manifest) = cli.manifest_file {
        match std::fs::read_to_string(&manifest) {
            Ok(content) => content,
            Err(error) => {
                eprintln!(
                    "\x1b[93mERROR: Failed to read manifest {:?} \x1b[0m",
                    manifest
                );
                eprintln!("   {}", error);
                std::process::exit(1);
            }
        }
    } else {
        let mut buffer = String::new();
        for line in std::io::stdin().lines() {
            buffer.push_str(&line.unwrap());
        }
        buffer.trim().to_string()
    };

    let mut builder_deserialized = serde_json::Deserializer::from_str(&content);
    let builder = serde_path_to_error::deserialize(&mut builder_deserialized);
    if let Err(error) = builder {
        eprintln!("\x1b[93mERROR: Failed to parse builder\x1b[0m");
        eprintln!("   serde errors: {}", error);

        let builder_json: serde_json::Value = serde_json::from_str(BUILDER_SCHEMA).unwrap();
        let content_json: serde_json::Value = serde_json::from_str(&content).unwrap();
        let validator = jsonschema::draft202012::new(&builder_json);

        if validator.is_err() {
            eprintln!("\n\x1b[91mUNEXPECTED ERROR: Invalid builder schema\x1b[0m");
            std::process::exit(127);
        }
        let validator = validator.unwrap();

        if let Err(schema_errors) = validator.validate(&content_json) {
            eprintln!("   schema errors: {}", schema_errors);
        }
        std::process::exit(1);
    }
    let mut builder: project_dirs_builder::Builder = builder.unwrap();

    if let Some(application) = cli.application {
        builder.application = application;
    }

    if let Some(organization) = cli.organization_name {
        builder.organization = organization;
    }

    if let Some(qualifier) = cli.qualifier {
        builder.qualifier = qualifier;
    }

    let result = serde_json::to_string_pretty(&builder.build());

    match result {
        Ok(r) => println!("{r}"),
        Err(e) => {
            eprintln!("UNEXPECTED ERROR: Failed to serialize result: {e:?}");
            std::process::exit(1);
        }
    }
}
