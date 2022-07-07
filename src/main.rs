use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod build;
mod module_toml;
mod publish;

/// The cpm command line interface.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Build a project.
    Build {
        /// The project to build.
        #[clap(parse(from_os_str), default_value = ".")]
        path: PathBuf,
    },

    /// Publish a project.
    Publish {
        /// The project to publish.
        #[clap(parse(from_os_str), default_value = ".")]
        path: PathBuf,
    },

    /// Install dependencies.
    Install,
}

fn main() {
    let cli = Cli::parse();

    match cli.cmd {
        Commands::Build { path } => {
            match std::fs::create_dir((&path).join("modules/lib")) {
                Ok(_) => {}
                Err(e) => {
                    if e.kind() != std::io::ErrorKind::AlreadyExists {
                        panic!("{}", e);
                    }
                }
            }
            build::build_root(&path);
        }
        Commands::Publish { path } => {
            publish::publish(&path);
        }
        Commands::Install => {
            println!("install");
        }
    }
}