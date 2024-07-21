use crate::context::Context;
use std::fs;
use std::process::{Command, Stdio};

pub fn build_project(context: &Context) -> Result<(), Box<dyn std::error::Error>> {
println!("Building project '{}'...", context.project_name);

// Create the build directory if it doesn't exist
let build_dir = context.project_path.join("jumake_build"); // New build directory name
fs::create_dir_all(&build_dir)?;

// Run CMake to generate the build files
let cmake_status = Command::new("cmake")
    .arg("..")
    .arg("-DCMAKE_EXPORT_COMPILE_COMMANDS=ON")
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

// Move compile_commands.json to the root of the project
let compile_commands_path = build_dir.join("compile_commands.json");
if compile_commands_path.exists() {
    let destination_path = context.project_path.join("compile_commands.json");
    fs::copy(&compile_commands_path, &destination_path)?;
    println!("Moved compile_commands.json to the project root.");
} else {
    return Err("compile_commands.json not found".into());
}

println!("Build successful!");
Ok(())
}

pub fn run_project(context: &Context) -> Result<(), Box<dyn std::error::Error>> {
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
    let executable_path = context.project_path.join(format!(
        "jumake_build/src/{}_artefacts/{}",
        context.project_name, context.project_name
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
