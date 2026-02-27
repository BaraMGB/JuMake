// scr/initialize_git.rs
use crate::context::Context;
use git2::{
    build::CheckoutBuilder, Error, FetchOptions, IndexAddOption, RemoteCallbacks, Repository,
    Signature,
};
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
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
            writeln!(gitignore_file, "compile_commands.json").expect("Failed to write to .gitignore");
            writeln!(gitignore_file, ".jumake").expect("Failed to write to .gitignore");
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
            index
                .add_path(Path::new(".gitmodules"))
                .expect("Failed to add .gitmodules to index");
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

        // Clone JUCE directly using git2::build::RepoBuilder
        // This bypasses the submodule mechanism and git's URL rewriting
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
                std::io::stdout().flush().unwrap();
            }
            true
        });

        remote_callbacks.credentials(|_url, username_from_url, allowed_types| {
            let username = username_from_url.unwrap_or("git");

            // For SSH (if git config rewrites HTTPS to SSH)
            if allowed_types.contains(git2::CredentialType::SSH_KEY) {
                // Try SSH agent first
                if let Ok(cred) = git2::Cred::ssh_key_from_agent(username) {
                    return Ok(cred);
                }

                // Try default SSH key locations
                if let Some(home) = std::env::var_os("HOME") {
                    let home_path = std::path::Path::new(&home);
                    let key_paths = [
                        home_path.join(".ssh").join("id_ed25519"),
                        home_path.join(".ssh").join("id_rsa"),
                        home_path.join(".ssh").join("id_ecdsa"),
                    ];

                    for key_path in &key_paths {
                        if key_path.exists() {
                            if let Ok(cred) = git2::Cred::ssh_key(username, None, key_path, None) {
                                return Ok(cred);
                            }
                        }
                    }
                }
            }

            // For HTTPS
            if allowed_types.contains(git2::CredentialType::DEFAULT) {
                return git2::Cred::default();
            }

            git2::Cred::default()
        });

        let mut fetch_options = FetchOptions::new();
        fetch_options.remote_callbacks(remote_callbacks);

        let mut builder = git2::build::RepoBuilder::new();
        builder.fetch_options(fetch_options);

        let submodule_repo = builder.clone(submodule_url, &juce_path)?;
        println!("\nFetched all branches");

        // Checkout the master branch explicitly
        let (object, reference) = submodule_repo.revparse_ext("refs/remotes/origin/master")?;
        submodule_repo.checkout_tree(&object, Some(CheckoutBuilder::default().force()))?;
        submodule_repo.set_head(reference.unwrap().name().unwrap())?;

        // Now add it to parent repo as a submodule by creating .gitmodules
        let gitmodules_path = context.project_path.join(".gitmodules");
        let gitmodules_content = format!(
            "[submodule \"modules/JUCE\"]\n\tpath = modules/JUCE\n\turl = {}\n",
            submodule_url
        );
        fs::write(gitmodules_path, gitmodules_content)?;

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

pub fn create_initial_commit(context: &Context) -> Result<(), Error> {
    let repo = Repository::open(&context.project_path)?;
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
