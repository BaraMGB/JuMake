// scr/main.rs

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use jumake::{create_files::create_source_files, create_files::create_cmakelists, context::Context, initialize_git::initialize_git_repo};
use std::fs::{self};
use git2::{Repository, Signature, Error};

// CLI argument parsing using clap
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new JuMake project
    New {
        /// Name of the project
        project_name: String,
        /// Optional path for the project
        #[arg(short, long)]
        path: Option<String>,
    },
    /// Add a new class to the project
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
    /// Build and run the project
    Run,
    /// Build the project
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
        // Add other command implementations here later
        _ => todo!(), 
    }
}

// Create a new project
fn create_project(context: &Context) {
    if context.project_path.exists() {
        println!("Error: Project directory already exists: {}", context.project_path.display());
        return;
    }

    println!("Creating project '{}' at {}...", context.project_name, context.project_path.display());

    fs::create_dir_all(&context.project_path).expect("Failed to create project directory");

    if let Err(e) = create_cmakelists(context) {
        eprintln!("Failed to create CMakeLists.txt: {}", e);
    }

    if let Err(e) = create_source_files(context) {
        eprintln!("Failed to create source files: {}", e);
    }
    initialize_git_repo(context);
    if let Err(e) = create_initial_commit(context) {
        eprintln!("Failed to create initial commit: {}", e);
    }
}

fn create_initial_commit(context: &Context) -> Result<(), Error> {
    let repo = Repository::open(&context.project_path)?;
    let signature = Signature::now("JuMake", "jumake@example.com")?;
    let tree_id = {
        let mut index = repo.index()?;
        index.write_tree()?
    };
    let tree = repo.find_tree(tree_id)?;
    let commit_id = repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        "Initial commit by JuMake",
        &tree,
        &[],
    )?;
    println!("Initial commit created with id: {}", commit_id);
    Ok(())
}

