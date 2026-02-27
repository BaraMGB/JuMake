// // tests/integration_test.rs
//
// use std::fs;
// use std::path::PathBuf;
// use jumake::{create_files::create_source_files, create_files::create_cmakelists, context::Context};
// #[test]
// fn test_create_cmakelists() {
//     let context = Context {
//         project_name: String::from("test_project"),
//         project_path: PathBuf::from("/tmp/cmake_test_project"),
//         template_name: None,
//     };
//
//     fs::create_dir_all(&context.project_path).expect("Failed to create test project directory");
//     create_cmakelists(&context).expect("Failed to create CMakeLists.txt");
//
//     let cmakelists_path = context.project_path.join("CMakeLists.txt");
//     assert!(cmakelists_path.exists());
//
//     let content = fs::read_to_string(cmakelists_path).expect("Failed to read CMakeLists.txt");
//     assert!(content.contains("cmake_minimum_required(VERSION 3.24)"));
//     assert!(content.contains("project(test_project)"));
//
//     fs::remove_dir_all(&context.project_path).expect("Failed to clean up test project directory");
// }
//
// #[test]
// fn test_create_source_files() {
//     let context = Context {
//         project_name: String::from("test_project"),
//         project_path: PathBuf::from("/tmp/sourcefile_test_project"),
//         template_name: None,
//     };
//
//     fs::create_dir_all(&context.project_path).expect("Failed to create test project directory");
//     create_source_files(&context).expect("Failed to create source files");
//
//     let src_path = context.project_path.join("src");
//     assert!(src_path.exists());
//
//     let main_cpp_path = src_path.join("main.cpp");
//     assert!(main_cpp_path.exists());
//
//     let content = fs::read_to_string(main_cpp_path).expect("Failed to read main.cpp");
//     assert_eq!(content, "// Your code goes here!");
//
//     fs::remove_dir_all(&context.project_path).expect("Failed to clean up test project directory");
// }
//

// tests/integration_test.rs

use jumake::{
    context::Context,
    create_files::{create_cmakelists, create_source_files},
};
use lazy_static::lazy_static;
use std::fs;
use std::path::PathBuf;

// Test data structure
struct TestData {
    template_name: &'static str,
    expected_cmake_content: &'static str,
    expected_source_files: Vec<&'static str>,
}

// Define test data for each template
lazy_static! {
    static ref TEST_DATA: [TestData; 3] = [
        TestData {
            template_name: "GuiApplication",
            expected_cmake_content:
                "cmake_minimum_required(VERSION 3.24)\nproject(test_project VERSION 0.0.1)\n",
            expected_source_files: vec!["Main.cpp", "MainComponent.cpp", "MainComponent.h"],
        },
        TestData {
            template_name: "AudioPlugin",
            expected_cmake_content:
                "cmake_minimum_required(VERSION 3.24)\nproject(test_project VERSION 0.0.1)\n",
            expected_source_files: vec![
                "PluginProcessor.cpp",
                "PluginProcessor.h",
                "PluginEditor.cpp",
                "PluginEditor.h",
            ],
        },
        TestData {
            template_name: "ConsoleApp",
            expected_cmake_content:
                "cmake_minimum_required(VERSION 3.24)\nproject(test_project VERSION 0.0.1)\n",
            expected_source_files: vec!["Main.cpp"],
        },
    ];
}

#[test]
fn test_create_cmakelists() {
    for data in &*TEST_DATA {
        let context = Context {
            project_name: String::from("test_project"),
            project_path: PathBuf::from("/tmp/cmake_test_project"),
            template_name: Some(String::from(data.template_name)),
            build_type: String::from("Release"),
        };
        let _ = fs::remove_dir_all(&context.project_path); // Clean up before running the test
        fs::create_dir_all(&context.project_path).expect("Failed to create test project directory");
        create_cmakelists(&context).expect("Failed to create CMakeLists.txt");

        let cmakelists_path = context.project_path.join("CMakeLists.txt");
        assert!(cmakelists_path.exists());

        let content = fs::read_to_string(cmakelists_path).expect("Failed to read CMakeLists.txt");
        assert!(
            content.contains(data.expected_cmake_content),
            "Content does not match. Expected:\n{}\nFound:\n{}",
            data.expected_cmake_content,
            content
        );
        fs::remove_dir_all(&context.project_path)
            .expect("Failed to clean up test project directory");
    }
}

#[test]
fn test_create_source_files() {
    for data in &*TEST_DATA {
        let context = Context {
            project_name: String::from("test_project"),
            project_path: PathBuf::from("/tmp/sourcefile_test_project"),
            template_name: Some(String::from(data.template_name)),
            build_type: String::from("Release"),
        };
        let _ = fs::remove_dir_all(&context.project_path); // Clean up before running the test
        fs::create_dir_all(&context.project_path).expect("Failed to create test project directory");
        create_source_files(&context).expect("Failed to create source files");

        let src_path = context.project_path.join("src");
        assert!(src_path.exists());

        for file_name in &data.expected_source_files {
            let file_path = src_path.join(file_name);
            assert!(
                file_path.exists(),
                "File {} does not exist in directory {}",
                file_name,
                src_path.display()
            );
        }

        fs::remove_dir_all(&context.project_path)
            .expect("Failed to clean up test project directory");
    }
}
