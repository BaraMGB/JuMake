use std::{fs, io::Result, path::Path};
use clap::{Parser, Subcommand};


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
            println!("Creating new project '{}'...", project_name);
            // Here comes the logic for creating the project
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
