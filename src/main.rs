// scr/main.rs

use clap::{Parser, Subcommand};
use std::path::PathBuf;
mod build;
mod context;
use build::{build_project, run_project};
use context::Context;
mod create_project;
use create_project::create_project;
mod create_files;
mod initialize_git;
// CLI argument parsing using clap
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    New {
        /// Name of the project
        project_name: String,
        /// Optional path for the project
        #[arg(short, long)]
        path: Option<String>,
    },
    Add {
        /// Name of the class
        class_name: String,
        /// Create only a header file
        #[arg(long)]
        header_only: bool,
        /// Optional output directory
        #[arg(short, long)]
        output: Option<String>,
    },
    Run,
    Build,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::New { project_name, path } => {
            let project_path = match path {
                Some(p) => PathBuf::from(p).join(&project_name),
                None => PathBuf::from(&project_name),
            };

            let context = Context {
                project_name,
                project_path,
            };
            create_project(&context);
        }
        Commands::Build => {
            // Get the current directory as the project path
            let project_path = std::env::current_dir().expect("Failed to get current directory");
            let context = Context {
                project_name: project_path
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .to_string(),
                project_path,
            };

            if let Err(e) = build_project(&context) {
                eprintln!("Build failed: {}", e);
            }
        }
        Commands::Run => {
            let project_path = std::env::current_dir().expect("Failed to get current directory");
            let context = Context {
                project_name: project_path
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .to_string(),
                project_path,
            };

            if let Err(e) = run_project(&context) {
                eprintln!("Failed run: {}", e);
            }
        }
        // Add other command implementations here later
        _ => todo!(),
    }
}
