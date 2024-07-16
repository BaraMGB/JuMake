// tests/integration_test.rs

use std::fs;
use std::path::PathBuf;
use jumake::{create_files::create_source_files, create_files::create_cmakelists, context::Context};
#[test]
fn test_create_cmakelists() {
    let context = Context {
        project_name: String::from("test_project"),
        project_path: PathBuf::from("/tmp/cmake_test_project"),
    };

    fs::create_dir_all(&context.project_path).expect("Failed to create test project directory");
    create_cmakelists(&context).expect("Failed to create CMakeLists.txt");

    let cmakelists_path = context.project_path.join("CMakeLists.txt");
    assert!(cmakelists_path.exists());

    let content = fs::read_to_string(cmakelists_path).expect("Failed to read CMakeLists.txt");
    assert!(content.contains("cmake_minimum_required(VERSION 3.24)"));
    assert!(content.contains("project(test_project)"));

    fs::remove_dir_all(&context.project_path).expect("Failed to clean up test project directory");
}

#[test]
fn test_create_source_files() {
    let context = Context {
        project_name: String::from("test_project"),
        project_path: PathBuf::from("/tmp/sourcefile_test_project"),
    };

    fs::create_dir_all(&context.project_path).expect("Failed to create test project directory");
    create_source_files(&context).expect("Failed to create source files");

    let src_path = context.project_path.join("src");
    assert!(src_path.exists());

    let main_cpp_path = src_path.join("main.cpp");
    assert!(main_cpp_path.exists());

    let content = fs::read_to_string(main_cpp_path).expect("Failed to read main.cpp");
    assert_eq!(content, "// Your code goes here!");

    fs::remove_dir_all(&context.project_path).expect("Failed to clean up test project directory");
}
