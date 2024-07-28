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
use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::io::{BufRead, BufReader};
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
    /// Build the project.
    Build {
        #[arg(short = 't', long = "build-type", default_value_t = String::from("Release"))]
        build_type: String,
    },
    /// Build and Run the project.
    Run {
        #[arg(short = 't', long = "build-type", default_value = "LastUsed")]
        build_type: String,
    },
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
                build_type: String::from("Release"),
            };

            create_project(&context);
        },
        Commands::Build { build_type } => {
            if let Err(error_message) = validate_build_type(&build_type) {
                eprintln!("{}", error_message);
                return;
            }
            let project_path = std::env::current_dir().expect("Failed to get current directory");
            let context = Context {
                project_name: project_path.file_name().unwrap().to_string_lossy().to_string(),
                project_path: project_path.clone(),
                template_name: determine_template_name(&project_path),
                build_type: build_type.clone(),
            };

            if let Err(e) = build_project(&context) {
                eprintln!("Build failed: {}", e);
            } else {
                if let Err(e) = save_build_type(&context) {
                    eprintln!("Failed to save last build type {}", e);
                }
            }
        },
        Commands::Run { build_type } => {
            let project_path = std::env::current_dir().expect("Failed to get current directory");
            let effective_build_type = if build_type == "LastUsed" {
                read_last_build_type(&project_path).unwrap_or_else(|| String::from("Release"))
            } else {
                build_type
            };
            if let Err(error_message) = validate_build_type(&effective_build_type) {
                eprintln!("{}", error_message);
                return;
            }
            let project_name = match extract_project_name(project_path.join("CMakeLists.txt")) {
                Ok(name) => name,
                Err(e) => {
                    eprintln!("Failed to extract project name: {}", e);
                    return;
                }
            };
            let context = Context {
                project_name: project_name.clone(),
                project_path: project_path.clone(),
                template_name: determine_template_name(&project_path),
                build_type: effective_build_type,
            };
            if let Err(e) = run_project(&context) {
                eprintln!("Failed to run: {}", e);
            }
        },
        Commands::Add { element_type, element_name } => {
            let project_path = std::env::current_dir().expect("Failed to get current directory");
            let context = Context {
                project_name: project_path.file_name().unwrap().to_string_lossy().to_string(),
                project_path,
                template_name: None, // We don't need the template name for this command
                build_type: String::from("Release"), // Default build type
            };

            if let Err(e) = add_class(&context, &element_type, &element_name) {
                eprintln!("Failed to add {}: {}", element_type, e);
            }
        },
        // Add other command implementations here later
    }
}

fn validate_build_type(build_type: &str) -> Result<(), String> {
    match build_type {
        "Debug" | "Release" | "RelWithDebInfo" | "MinSizeRel" => Ok(()),
        _ => Err(format!("Invalid build type: {}. Use one of: Debug, Release, RelWithDebInfo, MinSizeRel", build_type)),
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
fn save_build_type(context: &Context) -> std::io::Result<()> {
    fs::write(context.project_path.join(".jumake"), &context.build_type)
}

fn read_last_build_type(project_path: &PathBuf) -> Option<String> {
    fs::read_to_string(project_path.join(".jumake")).ok()
}
fn extract_project_name<P: AsRef<Path>>(cmake_file_path: P) -> Result<String, Box<dyn Error>> {
    // Open the file
    let file = File::open(cmake_file_path)?;
    let reader = BufReader::new(file);

    // Read the file line by line
    for line in reader.lines() {
        let line = line?;
        // Look for the line that starts with "project("
        if line.trim_start().starts_with("project(") {
            // Extract the project name between the parentheses
            if let Some(start) = line.find('(') {
                if let Some(end) = line.find(')') {
                    return Ok(line[start + 1..end].trim().to_string());
                }
            }
        }
    }

    Err("Project name not found".into())
}
