// src/build.rs

use crate::context::Context;
use std::error::Error;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

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
        .arg(&context.build_type)
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
    if cfg!(target_os = "macos") && context.template_name.as_deref() != Some("ConsoleApp") {
        Command::new("open").arg(executable_path).status()?;
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
    let mut paths = Vec::new();
    collect_paths_recursively(&build_dir, &mut paths)?;

    if cfg!(target_os = "macos") && context.template_name.as_deref() != Some("ConsoleApp") {
        let mut app_candidates: Vec<PathBuf> = paths
            .iter()
            .filter(|p| p.extension().and_then(|ext| ext.to_str()) == Some("app"))
            .filter(|p| {
                let file_name = p.file_name().and_then(|n| n.to_str()).unwrap_or_default();
                file_name.contains(&context.project_name)
            })
            .cloned()
            .collect();

        if context.template_name.as_deref() == Some("AudioPlugin") {
            app_candidates.retain(|p| p.to_string_lossy().to_lowercase().contains("standalone"));
        }

        let executable_path = pick_best_match(app_candidates.into_iter(), &context.build_type)
            .ok_or_else(|| {
                format!(
                    "Executable not found for build type: {}",
                    context.build_type
                )
            })?;

        println!("start executable: {}", executable_path.display());
        return Ok(executable_path.to_string_lossy().to_string());
    }

    let binary_name = if cfg!(target_os = "windows") {
        format!("{}.exe", context.project_name)
    } else {
        context.project_name.clone()
    };

    let executable_candidates: Vec<PathBuf> = paths
        .into_iter()
        .filter(|p| p.file_name().and_then(|n| n.to_str()) == Some(binary_name.as_str()))
        .filter(|p| is_executable_file(p))
        .collect();

    let executable_path = pick_best_match(executable_candidates.into_iter(), &context.build_type)
        .ok_or_else(|| {
        format!(
            "Executable not found for build type: {}",
            context.build_type
        )
    })?;

    println!("start executable: {}", executable_path.display());
    Ok(executable_path.to_string_lossy().to_string())
}

fn collect_paths_recursively(dir: &Path, output: &mut Vec<PathBuf>) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        output.push(path.clone());
        if path.is_dir() {
            collect_paths_recursively(&path, output)?;
        }
    }
    Ok(())
}

fn pick_best_match<I>(candidates: I, build_type: &str) -> Option<PathBuf>
where
    I: Iterator<Item = PathBuf>,
{
    let mut paths: Vec<PathBuf> = candidates.collect();
    paths.sort_by_key(|p| p.to_string_lossy().len());
    paths
        .iter()
        .find(|p| p.to_string_lossy().contains(build_type))
        .cloned()
        .or_else(|| paths.first().cloned())
}

#[cfg(unix)]
fn is_executable_file(path: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;

    path.is_file()
        && fs::metadata(path)
            .map(|meta| meta.permissions().mode() & 0o111 != 0)
            .unwrap_or(false)
}

#[cfg(windows)]
fn is_executable_file(path: &Path) -> bool {
    path.is_file()
        && path
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.eq_ignore_ascii_case("exe"))
            .unwrap_or(false)
}

#[cfg(all(not(unix), not(windows)))]
fn is_executable_file(path: &Path) -> bool {
    path.is_file()
}
