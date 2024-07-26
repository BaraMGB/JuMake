// src/context.rs
use std::path::PathBuf;


pub struct Context {
    pub project_name: String,
    pub project_path: PathBuf,
    pub template_name: Option<String>,
    pub build_type: String,
}
