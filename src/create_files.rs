// src/create_files.rs
use std::fs::{self, File};
use std::io::Write;
use indoc::indoc;
use crate::context::Context;

pub fn create_source_files(context: &Context) -> Result<(), Box<dyn std::error::Error>> {
    let src_path = context.project_path.join("src");
    fs::create_dir(&src_path)?;

    // Lade die Main.cpp-Vorlage
    let main_cpp_template = include_str!("../templates/GuiApplicationTemplate/Main.cpp.template");
    let main_component_cpp_template = include_str!("../templates/GuiApplicationTemplate/MainComponent.cpp.template");
    let main_component_h_template = include_str!("../templates/GuiApplicationTemplate/MainComponent.h.template");
    let cmake_lists_template = include_str!("../templates/GuiApplicationTemplate/CMakeLists.txt.template");

    // Schreibe die Datei
    let main_cpp_path = src_path.join("Main.cpp");
    let main_component_h_path = src_path.join("MainComponent.h");
    let main_component_cpp_path = src_path.join("MainComponent.cpp");
    let cmake_lists_path = src_path.join("CMakeLists.txt");

    let mut main_cpp_file = fs::File::create(main_cpp_path)?;
    let mut main_component_cpp_file = fs::File::create(main_component_cpp_path)?;
    let mut main_component_h_file = fs::File::create(main_component_h_path)?;
    let mut cmake_lists_file = fs::File::create(cmake_lists_path)?;

    main_cpp_file.write_all(main_cpp_template.as_bytes())?;
    main_component_cpp_file.write_all(main_component_cpp_template.as_bytes())?;
    main_component_h_file.write_all(main_component_h_template.as_bytes())?;
    cmake_lists_file.write_all(cmake_lists_template.as_bytes())?;

    Ok(())
}
pub fn create_cmakelists(context: &Context) -> Result<(), Box<dyn std::error::Error>> {
    let cmakelists_path = context.project_path.join("CMakeLists.txt");
    let mut cmakelists_file = File::create(cmakelists_path)?;

    let cmake_content = format!(
        indoc! {
            "cmake_minimum_required(VERSION 3.24)
             project({} VERSION 0.0.1)
             add_subdirectory(modules/JUCE)
             add_subdirectory(src)"
        },
        context.project_name
    );

    cmakelists_file
        .write_all(cmake_content.as_bytes())?;
    Ok(())
}
