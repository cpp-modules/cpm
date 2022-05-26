use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

extern crate cc;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() == 1 {
        println!("Helper: {} help", args[0]);
        std::process::exit(1);
    }

    let mut args = args.into_iter();
    let cpm = args.next().unwrap();
    let command = args.next().unwrap();
    let path = args.next();

    match command.as_str() {
        "help" => {
            println!("Usage: {} <command>", cpm);
            println!("Commands:");
            println!("  help");
            println!("  build");
        }
        "build" => {
            if let Some(path) = path {
                let outdir = std::env::current_dir()
                    .unwrap()
                    .join(&path)
                    .join("modules/target");
                std::env::set_var("OUT_DIR", outdir);
                std::env::set_var("TARGET", "x86_64-pc-windows-msvc");
                std::env::set_var("OPT_LEVEL", "3");
                std::env::set_var("HOST", "x86_64-pc-windows-msvc");
                build_module(&path, None);
            } else {
                let outdir = std::env::current_dir().unwrap().join("modules/target");
                std::env::set_var("OUT_DIR", outdir);
                std::env::set_var("TARGET", "x86_64-pc-windows-msvc");
                std::env::set_var("OPT_LEVEL", "3");
                std::env::set_var("HOST", "x86_64-pc-windows-msvc");
                build_module(&String::from("."), None);
            }
        }
        _ => {
            println!("Unknown command: {}", command);
            std::process::exit(1);
        }
    }
}

fn build_module(root_path: &String, submodule_path: Option<String>) {
    let mut builder = cc::Build::new();
    builder.cpp(true);

    let module_path = if let Some(submodule_path) = submodule_path {
        builder.static_flag(true);
        std::env::current_dir()
            .unwrap()
            .join(root_path)
            .join("modules")
            .join(&submodule_path)
    } else {
        std::env::current_dir().unwrap().join(root_path)
    };

    let module_toml = std::fs::read_to_string(module_path.join("module.toml"))
        .expect("Failed to read module.toml");
    let module: Module = toml::from_str(&module_toml).expect("Failed to parse module.toml");

    for (submodule, _) in module.dependencies {
        build_module(root_path, Some(submodule));
    }

    for filename in module.sources.source {
        builder.file(module_path.join(filename));
    }

    builder.compile(&module.project.name);
}

#[derive(Debug, Serialize, Deserialize)]
struct Module {
    pub project: Project,
    pub dependencies: Deps,
    pub flags: Flags,
    pub sources: Sources,
}

#[derive(Debug, Serialize, Deserialize)]
struct Project {
    pub name: String,
    pub version: String,
    pub module_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Sources {
    pub source: Vec<String>,
    pub header: Vec<String>,
}

type Deps = BTreeMap<String, String>;
type Flags = BTreeMap<String, String>;
