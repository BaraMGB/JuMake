// scr/create_projec.rs

use crate::context::Context;
use crate::create_files::create_cmakelists;
use crate::create_files::create_source_files;
use crate::initialize_git::create_initial_commit;
use crate::initialize_git::initialize_git_repo;
use std::fs;
pub fn create_project(context: &Context) -> Result<(), Box<dyn std::error::Error>> {
    if context.project_path.exists() {
        return Err(format!(
            "Error: Project directory already exists: {}",
            context.project_path.display()
        )
        .into());
    }

    println!(
        "Creating project '{}' at {}...",
        context.project_name,
        context.project_path.display()
    );

    fs::create_dir_all(&context.project_path)?;

    create_cmakelists(context)?;
    create_source_files(context)?;
    initialize_git_repo(context)?;
    create_initial_commit(context)?;

    Ok(())
}
