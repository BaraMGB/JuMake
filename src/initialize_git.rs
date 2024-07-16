use git2::{Repository, Signature, IndexAddOption, Error};
use crate::context::Context;
// Initialize the Git repository
pub fn initialize_git_repo(context: &Context) {
    println!("Initializing Git repository...");
    match Repository::init(&context.project_path) {
        Ok(repo) => {
            println!("Git repository initialized successfully.");
            // Add all files to the repository
            if let Err(e) = add_all_files_to_repo(&repo) {
                eprintln!("Error adding files to Git index: {}", e);
                return;
            }
            // Create the initial commit
            if let Err(e) = create_initial_commit(&repo) {
                eprintln!("Error creating initial commit: {}", e);
            } else {
                println!("Initial commit created.");
            }
        }
        Err(e) => eprintln!("Failed to initialize Git repository: {}", e),
    }
}

// Add all files to the Git repository index
fn add_all_files_to_repo(repo: &Repository) -> Result<(), Error> {
    let mut index = repo.index()?;
    index.add_all(["*"].iter(), IndexAddOption::DEFAULT, None)?;
    index.write()?;
    Ok(())
}

// Create the initial commit
fn create_initial_commit(repo: &Repository) -> Result<(), Error> {
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

