// scr/main.rs

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::process::{Command, Stdio};
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
        Commands::Build => {
            // Get the current directory as the project path
            let project_path = std::env::current_dir().expect("Failed to get current directory");
            let context = Context {
                project_name: project_path.file_name().unwrap().to_string_lossy().to_string(),
                project_path,
            };

            if let Err(e) = build_project(&context) {
                eprintln!("Build failed: {}", e);
            }
        }
        Commands::Run => {
            let project_path = std::env::current_dir().expect("Fehler beim Abrufen des aktuellen Verzeichnisses");
            let context = Context {
                project_name: project_path.file_name().unwrap().to_string_lossy().to_string(),
                project_path,
            };

            if let Err(e) = run_project(&context) {
                eprintln!("Ausführung fehlgeschlagen: {}", e);
            }
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

fn build_project(context: &Context) -> Result<(), Box<dyn std::error::Error>> {
    println!("Building project '{}'...", context.project_name);

    // Create the build directory if it doesn't exist
    let build_dir = context.project_path.join("jumake_build"); // New build directory name
    fs::create_dir_all(&build_dir)?;

    // Run CMake to generate the build files
    let cmake_status = Command::new("cmake")
        .arg("..")
        .current_dir(&build_dir)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?;
    if !cmake_status.success() {
        return Err("CMake configure failed".into());
    }

    // Run CMake to build the project
    let build_status = Command::new("cmake")
        .arg("--build")
        .arg(".")
        .current_dir(&build_dir)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?;

    if !build_status.success() {
        return Err("CMake build failed".into());
    }

    println!("Build successful!");
    Ok(())
}

fn run_project(context: &Context) -> Result<(), Box<dyn std::error::Error>> {
    // First, build the project
    if let Err(e) = build_project(context) {
        return Err(format!("Failed to build the project: {}", e).into());
    }

    println!("Running project '{}'...", context.project_name);

    // Find the path to the executable (need to consider JUCE here)
    let executable_path = find_executable(context)?;

    // Run the executable
    Command::new(executable_path)
        .current_dir(context.project_path.join("jumake_build"))
        .status()?;

    println!("Execution completed.");
    Ok(())
}

fn find_executable(context: &Context) -> Result<String, Box<dyn std::error::Error>> {
    // Determine the operating system
    if cfg!(target_os = "linux") {
        // Construct the path to the executable for Linux systems
        let executable_path = context
            .project_path
            .join(format!(
                "jumake_build/src/{}_artefacts/{}"
                , context.project_name
                , context.project_name
            ));

        // Check if the executable exists and return the path
        if executable_path.exists() {
            return Ok(executable_path.to_string_lossy().to_string());
        } else {
            return Err(format!("Executable not found at {:?}", executable_path).into());
        }
    }

    Err("Unsupported operating system".into())
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

