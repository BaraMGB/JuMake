// scr/initialize_git.rs
use git2::{Repository, RemoteCallbacks, IndexAddOption, Error, FetchOptions, build::CheckoutBuilder};
use crate::context::Context;
use std::path::Path;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
// Initialize the Git repository
pub fn initialize_git_repo(context: &Context) {
    println!("Initializing Git repository...");
    match Repository::init(&context.project_path) {
        Ok(repo) => {
            println!("Git repository initialized successfully.");

            let mut gitignore_file = OpenOptions::new()
                .append(true)
                .create(true)
                .open(context.project_path.join(".gitignore"))
                .expect("Failed to open .gitignore file");

            writeln!(gitignore_file, "modules/").expect("Failed to write to .gitignore");
            writeln!(gitignore_file, "jumake_build/").expect("Failed to write to .gitignore");
            writeln!(gitignore_file, "build/").expect("Failed to write to .gitignore");
            // Add all files to the repository (excluding the submodule)
            if let Err(e) = add_all_files_to_repo(&repo) {
                eprintln!("Error adding files to Git index: {}", e);
                return;
            }

            // Add JUCE as a submodule
            if let Err(e) = add_juce_submodule(context) {
                eprintln!("Error adding JUCE submodule: {}", e);
                return;
            }

            // Stage the .gitmodules file
            let mut index = repo.index().expect("Failed to get Git index");
            index.add_path(Path::new(".gitmodules")).expect("Failed to add .gitmodules to index");
            index.write().expect("Failed to write Git index");
        }
        Err(e) => eprintln!("Failed to initialize Git repository: {}", e),
    }
}
fn add_juce_submodule(context: &Context) -> Result<(), Box<dyn std::error::Error>> {
    let modules_path = context.project_path.join("modules");
    fs::create_dir_all(&modules_path)?;

    let juce_path = modules_path.join("JUCE");
    let submodule_url = "https://github.com/juce-framework/JUCE.git";

    if !juce_path.exists() {
        println!("Cloning JUCE from GitHub... this may take some minutes. Please be patient!");

        let repo = Repository::open(&context.project_path)?;

        // Set checkout to false to prevent automatic checkout of main branch
        let mut submodule = repo.submodule(submodule_url, Path::new("modules/JUCE"), false)?;
        submodule.init(true)?;

        let submodule_repo = submodule.open()?;

        // Set up remote callbacks for verbose output
        let mut remote_callbacks = RemoteCallbacks::new();
        let mut last_progress = 0;

        remote_callbacks.transfer_progress(move |stats| {
            if stats.received_objects() != last_progress {
                last_progress = stats.received_objects();
                print!(
                    "\rReceived {}/{} objects ({} bytes)",
                    stats.received_objects(),
                    stats.total_objects(),
                    stats.received_bytes()
                );
                use std::io::Write;
                std::io::stdout().flush().unwrap();
            }
            true
        });

        // Fetch all branches with verbose output
        let mut fetch_options = FetchOptions::new();
        fetch_options.remote_callbacks(remote_callbacks);
        fetch_options.download_tags(git2::AutotagOption::All);

        submodule_repo.find_remote("origin")?.fetch(&["+refs/heads/*:refs/remotes/origin/*"], Some(&mut fetch_options), None)?;
        println!("\nFetched all branches");

        // Checkout the master branch explicitly
        let (object, reference) = submodule_repo.revparse_ext("refs/remotes/origin/master")?;
        submodule_repo.checkout_tree(&object, Some(CheckoutBuilder::default().force()))?;
        submodule_repo.set_head(reference.unwrap().name().unwrap())?;

        println!("JUCE cloned successfully");
    } else {
        println!("JUCE already exists, skipping clone.");
    }

    Ok(())
   }


// Add all files to the Git repository index
fn add_all_files_to_repo(repo: &Repository) -> Result<(), Error> {
let mut index = repo.index()?;
    index.add_all(["*"].iter(), IndexAddOption::DEFAULT, None)?;
    index.write()?;
    Ok(())
}

// Create the initial commit
