// src/build.rs

use crate::context::Context;
use std::fs;
use std::process::{Command, Stdio};
use std::error::Error;
use std::str;

pub fn build_project(context: &Context) -> Result<(), Box<dyn std::error::Error>> {
println!("Building project '{}'...", context.project_name);
println!("in '{}'...", context.build_type);

// Create the build directory if it doesn't exist
let build_dir = context.project_path.join("jumake_build"); // New build directory name
fs::create_dir_all(&build_dir)?;

// Run CMake to generate the build files
let cmake_status = Command::new("cmake")
    .arg("..")
    .arg(format!("-DCMAKE_BUILD_TYPE={}", context.build_type))
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
    .arg("--config")
    .arg(format!("{}", context.build_type))
    .current_dir(&build_dir)
    .stdout(Stdio::inherit())
    .stderr(Stdio::inherit())
    .status()?;

if !build_status.success() {
    return Err("CMake build failed".into());
}

// Move compile_commands.json to the root of the project (only on non-Windows)
if !cfg!(target_os = "windows") {
    let compile_commands_path = build_dir.join("compile_commands.json");
    if compile_commands_path.exists() {
        let destination_path = context.project_path.join("compile_commands.json");
        fs::copy(&compile_commands_path, &destination_path)?;
        println!("Moved compile_commands.json to the project root.");
    } else {
        return Err("compile_commands.json not found".into());
    }
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
    if cfg!(target_os = "macos") {
        Command::new("open")
            .arg(executable_path)
            .status()?;
    } else {
        Command::new(executable_path)
            .current_dir(context.project_path.join("jumake_build"))
            .status()?;
    }

    println!("Execution completed.");
    Ok(())
}

fn find_executable(context: &Context) -> Result<String, Box<dyn Error>> {
    println!("Template name: {:?}", context.template_name);
    println!("Build type: {:?}", context.build_type);
    println!("Project name: {}", context.project_name);

    let build_dir = context.project_path.join("jumake_build");

    let output = if cfg!(target_os = "windows") {
        let find_command = format!(
            "Get-ChildItem -Recurse -Filter '{}.exe' -File | Select-Object -ExpandProperty FullName",
            context.project_name
        );
        Command::new("powershell")
            .arg("-Command")
            .arg(&find_command)
            .current_dir(&build_dir)
            .output()
            .expect("Failed to execute PowerShell command")
    } else {
        let find_command = if cfg!(target_os = "macos") {
            if context.template_name.as_deref() == Some("AudioPlugin") {
                format!(
                    "find {} -name {} -type f -perm +111 | grep Standalone",
                    build_dir.to_string_lossy(),
                    context.project_name
                )
            } else {
                format!(
                    "find {} -name {} -type f -perm +111",
                    build_dir.to_string_lossy(),
                    context.project_name
                )
            }
        } else {
            format!(
                "find {} -name {} -type f -executable",
                build_dir.to_string_lossy(),
                context.project_name
            )
        };
        Command::new("sh")
            .arg("-c")
            .arg(&find_command)
            .output()
            .expect("Failed to execute find command")
    };

    if !output.status.success() {
        return Err(format!("Find command failed with output: {:?}", output).into());
    }

    let output_str = str::from_utf8(&output.stdout)?;
    let paths: Vec<&str> = output_str.lines().collect();

    let executable_path = paths.into_iter()
        .find(|path| path.contains(&context.build_type))
        .ok_or_else(|| format!("Executable not found for build type: {}", context.build_type))?;

    let executable_path = if cfg!(target_os = "macos") {
        // cut off after ".app"
        if let Some(app_index) = executable_path.find(".app") {
            let end_index = app_index + 4;
            &executable_path[..end_index]
        } else {
            executable_path
        }
    } else {
        executable_path
    };

    println!("start executable: {}", executable_path);

    Ok(executable_path.to_string())
}
