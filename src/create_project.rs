// scr/create_projec.rs

use crate::context::Context;
use crate::create_files::create_cmakelists;
use crate::create_files::create_source_files;
use crate::initialize_git::create_initial_commit;
use crate::initialize_git::initialize_git_repo;
use std::fs;
pub fn create_project(context: &Context) {
    if context.project_path.exists() {
        println!(
            "Error: Project directory already exists: {}",
            context.project_path.display()
        );
        return;
    }

    println!(
        "Creating project '{}' at {}...",
        context.project_name,
        context.project_path.display()
    );

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
