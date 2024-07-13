use clap::{Parser, Subcommand};
use std::fs;
use std::fs::File;
use std::path::Path;
use std::io::Write;
use indoc::indoc;


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

    match &cli.command {
        Commands::New { project_name, path } => {
            create_project(project_name.clone(), path.clone());
        }
        Commands::Add { class_name, header_only, output } => {
            println!("Adding class '{}'...", class_name);
            // Here comes the logic for adding the class
        }
        Commands::Run => {
            println!("Building and running the project...");
            // Here comes the logic for building and running the project
        }
        Commands::Build => {
            println!("Building the project...");
            // Here comes the logic for building the project
        }
    }
}

fn create_project(projectname: String, path: Option<String>) {
    let project_path = match path {
        Some(p) => Path::new(&p).join(&projectname),
        None => Path::new(&projectname).to_path_buf(),
    };

    if project_path.exists() {
        println!("Error: Project directory already exists: {}", project_path.display());
        return;
    }

    println!("Creating project '{}' at {}...", projectname, project_path.display());

    // Create project directory
    fs::create_dir_all(&project_path).expect("Failed to create project directory");


    // TODO:
    // 1. create CMakeLists.txt

    let cmakelists_path = project_path.join("CMakeLists.txt");
    let mut cmakelists_file = File::create(cmakelists_path).expect("Failed to create CMakeLists.txt");
    let cmake_content = format!(
        indoc! {
            "cmake_minimum_required(VERSION 3.24)
             project({})

             # Add your project's source files and targets here"
        },
        projectname
    );
    cmakelists_file
        .write_all(cmake_content.as_bytes())
        .expect("Failed to write to CMakeLists.txt");
    // 2. create source directory and template filed
    let src_path = project_path.join("src");
    fs::create_dir(&src_path).expect("Failed to create src directory");

    // Create main.cpp
    let main_cpp_path = src_path.join("main.cpp");
    let mut main_cpp_file = File::create(main_cpp_path).expect("Failed to create main.cpp");
    main_cpp_file.write_all(b"// Your code goes here!").expect("Failed to write to main.cpp");
    // TODO: Add more template files
    // 3. Initialize Git repo
}
