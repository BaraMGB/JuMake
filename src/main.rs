// scr/main.rs

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use dialoguer::{Select, theme::ColorfulTheme};
use regex::Regex;
use std::fs;
mod build;
mod context;
use build::{build_project, run_project};
use context::Context;
mod create_project;
use create_project::create_project;
mod create_files;
mod initialize_git;
use create_files::add_class;
// CLI argument parsing using clap
#[derive(Parser)]
#[command(author, version, about = "A CLI tool for creating and managing JUCE projects.", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
      /// Create a new JUCE project.
    New {
    /// The name of the project.
    #[arg(name = "project_name")]
    project_name: String,
    /// The path to create the project at (optional).
    #[arg(short, long, name = "path")]
    path: Option<String>,
    /// The template to use (optional).
    #[arg(short, long, name = "template")]
    template: Option<String>,
    },
    /// Add a new c++ class or a JUCE component to the project.
    Add {
        /// The type of element to add (simple c++ class or JUCE component).
        #[arg(value_enum, name = "class type", help = "Specify the type of class to add, 'component' or 'class'.")]
        element_type: String,
        /// The name of the class or component.
        #[arg(name = "name", help = "Specify the name of the class to add. ")]
        element_name: String,
    },
    /// Build and Run the project.
    Run,
    /// Build the project.
    Build,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::New { project_name, path, template } => {
            let project_path = match path {
                Some(p) => PathBuf::from(p).join(&project_name),
                None => PathBuf::from(&project_name),
            };
            // Determine template name
            let template_name = match template {
            Some(t) => Some(t), // Wrap the String in Some()
            None => {
                // Display menu and get user selection
                let selections = ["GuiApplication", "AudioPlugin", "ConsoleApp"];
                let selection = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("Select a template:")
                    .default(0)
                    .items(&selections[..])
                    .interact()
                    .expect("Failed to get template selection");

                Some(selections[selection].to_string()) // Wrap the result in Some()
                }
            };

            let context = Context {
                project_name,
                project_path,
                template_name,
            };

            create_project(&context);
        }
         Commands::Build | Commands::Run => {
            // Get the current directory as the project path
            let project_path = std::env::current_dir().expect("Failed to get current directory");

            // Determine the template name from the project
            let template_name  = determine_template_name(&project_path);

            let context = Context {
                project_name: project_path.file_name().unwrap().to_string_lossy().to_string(),
                project_path,
                template_name,
            };

            match cli.command {
                Commands::Build => {
                    if let Err(e) = build_project(&context) {
                        eprintln!("Build failed: {}", e);
                    }
                }
                Commands::Run => {
                    if let Err(e) = run_project(&context) {
                        eprintln!("Failed run: {}", e);
                    }
                }
                _ => unreachable!(), // Build and Run are the only options here
            }
        }
        Commands::Add { element_type, element_name } => {
            let project_path = std::env::current_dir().expect("Failed to get current directory");
            let context = Context {
                project_name: project_path.file_name().unwrap().to_string_lossy().to_string(),
                project_path,
                template_name: None, // We don't need the template name for this command
            };

            if let Err(e) = add_class(&context, &element_type, &element_name) {
                eprintln!("Failed to add {}: {}", element_type, e);
            }
        }
        // Add other command implementations here later
    }
}

fn determine_template_name(project_path: &PathBuf) -> Option<String> {
    let cmakelists_path = project_path.join("src").join("CMakeLists.txt");

    if cmakelists_path.exists() {
        // Read the CMakeLists.txt content
        let content = fs::read_to_string(&cmakelists_path).unwrap_or_default();

        // Define a regular expression to extract the JUMAKE_TEMPLATE value
        let re = Regex::new(r#"set\(JUMAKE_TEMPLATE\s+"([^"]+)"\)"#).unwrap();

        // Find the match
        if let Some(captures) = re.captures(&content) {
            // Extract the template name from the first capture group
            return Some(captures.get(1).unwrap().as_str().to_string());
        }
    }

    // Default to GuiApplication if no template is found
    Some("GuiApplication".to_string())
}
