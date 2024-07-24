// src/create_files.rs
use std::fs::{self, File};
use std::io::{Write, BufRead, BufReader};
use indoc::indoc;
use crate::context::Context;
use std::path::PathBuf;
use std::error::Error;

pub fn create_source_files(context: &Context) -> Result<(), Box<dyn std::error::Error>> {
    let src_path = context.project_path.join("src");
    fs::create_dir(&src_path)?;

    match context.template_name.as_deref() {
        Some("GuiApplication") => {
            // Create GUI application files
            create_file_from_template(&src_path, "Main.cpp", MAIN_CPP_TEMPLATE)?;
            create_file_from_template(
                &src_path,
                "MainComponent.cpp",
                MAIN_COMPONENT_CPP_TEMPLATE,
            )?;
            create_file_from_template(&src_path, "MainComponent.h", MAIN_COMPONENT_H_TEMPLATE)?;
            create_file_from_template(&src_path, "CMakeLists.txt", GUI_APP_CMAKE_TEMPLATE)?;
        }
        Some("AudioPlugin") => {
            // Create audio plugin files
            create_file_from_template(
                &src_path,
                "PluginProcessor.cpp",
                PLUGIN_PROCESSOR_CPP_TEMPLATE,
            )?;
            create_file_from_template(
                &src_path,
                "PluginProcessor.h",
                PLUGIN_PROCESSOR_H_TEMPLATE,
            )?;
            create_file_from_template(&src_path, "PluginEditor.cpp", PLUGIN_EDITOR_CPP_TEMPLATE)?;
            create_file_from_template(&src_path, "PluginEditor.h", PLUGIN_EDITOR_H_TEMPLATE)?;
            create_file_from_template(&src_path, "CMakeLists.txt", AUDIO_PLUGIN_CMAKE_TEMPLATE)?;
        }
        Some("ConsoleApp") => {
            // Create console application files
            create_file_from_template(&src_path, "Main.cpp", CONSOLE_APP_MAIN_CPP_TEMPLATE)?;
            create_file_from_template(&src_path, "CMakeLists.txt", CONSOLE_APP_CMAKE_TEMPLATE)?;
        }
        _ => {
            return Err(format!("Unknown template: {:?}", context.template_name).into());
        }
    }
    Ok(())
}

// Function to add a class or component file to a project based on the given context and element type.
pub fn add_class(context: &Context, element_type: &str, element_name: &str) -> Result<(), Box<dyn Error>> {
    // Construct the source directory path from the context's project path.
    let src_path = context.project_path.join("src");

    // Determine the correct templates for the header and source files based on the element type.
    let mut adjusted_element_name = element_name.to_string();
    let (header_template, cpp_template) = match element_type {
        "class" => (CLASS_H_TEMPLATE, CLASS_CPP_TEMPLATE),
        "component" => {
            // Append "Component" to the name for component types to differentiate from regular classes.
            adjusted_element_name.push_str("Component");
            (COMPONENT_H_TEMPLATE, COMPONENT_CPP_TEMPLATE)
        },
        _ => return Err(format!("Invalid element type: {}", element_type).into()),
    };

    // Construct file names for the header and source files.
    let header_file_name = format!("{}.h", adjusted_element_name);
    let cpp_file_name = format!("{}.cpp", adjusted_element_name);

    // Check if the header or source file already exists
    let header_path = src_path.join(&header_file_name);
    let cpp_path = src_path.join(&cpp_file_name);

    if header_path.exists() || cpp_path.exists() {
        return Err(format!("{} '{}' already exists in the project.", element_type, adjusted_element_name).into());
    }

    // Construct file names for the header and source files.
    let header_file_name = format!("{}.h", adjusted_element_name);
    let cpp_file_name = format!("{}.cpp", adjusted_element_name);

    // Create files from templates with the adjusted names.
    create_classfile_from_template(&src_path, &header_file_name, header_template, &adjusted_element_name)?;
    create_classfile_from_template(&src_path, &cpp_file_name, cpp_template, &adjusted_element_name)?;

    // Attempt to add the newly created cpp file to the CMakeLists.txt
    let cmakelists_path = src_path.join("CMakeLists.txt");
    let file = File::open(&cmakelists_path)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    let mut new_lines = Vec::new();
    let mut found_target_sources = false;
    let mut added = false;

    // Iterate through each line of CMakeLists.txt to find the target_sources block.
    for line in lines.iter() {
        new_lines.push(line.to_string());
        if line.trim_start().starts_with("target_sources(${PROJECT_NAME}") {
            found_target_sources = true;
        }
        if found_target_sources && line.trim_start().starts_with("PRIVATE") && !added {
            // Calculate indentation and insert the cpp file name under the PRIVATE specifier.
            let indentation = line.chars().take_while(|c| c.is_whitespace()).count() + 4;
            let new_cpp_line = format!("{:indent$}{}", "", cpp_file_name, indent = indentation);
            new_lines.push(new_cpp_line);
            added = true;
        }
    }

    // Handle the case where the PRIVATE specifier was not found after target_sources.
    if !added {
        return Err("Could not find 'PRIVATE' after 'target_sources' in CMakeLists.txt".into());
    }

    // Write the updated content back to CMakeLists.txt.
    let new_content = new_lines.join("\n");
    let mut cmakelists_file = File::create(&cmakelists_path)?;
    cmakelists_file.write_all(new_content.as_bytes())?;

    // Confirm the addition of the new class or component.
    println!("{} '{}' added successfully!", element_type, adjusted_element_name);
    Ok(())
}

fn create_classfile_from_template(
    src_path: &PathBuf,
    file_name: &str,
    template: &[u8],
    element_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(src_path.join(file_name))?;
    let content = String::from_utf8_lossy(template).replace("Template", element_name);
    file.write_all(content.as_bytes())?;
    println!("Created file: {}", src_path.join(file_name).display());
    Ok(())
}

fn create_file_from_template(
    src_path: &PathBuf,
    file_name: &str,
    template: &[u8],
) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(src_path.join(file_name))?;
    file.write_all(template)?;
    println!("Created file: {}", src_path.join(file_name).display());
    Ok(())
}
// Define the templates as byte slices
const MAIN_CPP_TEMPLATE: &[u8] = include_bytes!("../templates/GuiApplicationTemplate/Main.cpp.template");
const MAIN_COMPONENT_CPP_TEMPLATE: &[u8] =
    include_bytes!("../templates/GuiApplicationTemplate/MainComponent.cpp.template");
const MAIN_COMPONENT_H_TEMPLATE: &[u8] =
    include_bytes!("../templates/GuiApplicationTemplate/MainComponent.h.template");
const GUI_APP_CMAKE_TEMPLATE: &[u8] =
    include_bytes!("../templates/GuiApplicationTemplate/CMakeLists.txt.template");

const PLUGIN_PROCESSOR_CPP_TEMPLATE: &[u8] =
    include_bytes!("../templates/AudioPluginTemplate/PluginProcessor.cpp.template");
const PLUGIN_PROCESSOR_H_TEMPLATE: &[u8] =
    include_bytes!("../templates/AudioPluginTemplate/PluginProcessor.h.template");
const PLUGIN_EDITOR_CPP_TEMPLATE: &[u8] =
    include_bytes!("../templates/AudioPluginTemplate/PluginEditor.cpp.template");
const PLUGIN_EDITOR_H_TEMPLATE: &[u8] =
    include_bytes!("../templates/AudioPluginTemplate/PluginEditor.h.template");
const AUDIO_PLUGIN_CMAKE_TEMPLATE: &[u8] =
    include_bytes!("../templates/AudioPluginTemplate/CMakeLists.txt.template");

const CONSOLE_APP_CMAKE_TEMPLATE: &[u8] =
    include_bytes!("../templates/ConsoleAppTemplate/CMakeLists.txt.template");

const CONSOLE_APP_MAIN_CPP_TEMPLATE: &[u8] =
    include_bytes!("../templates/ConsoleAppTemplate/Main.cpp.template");

const CLASS_H_TEMPLATE: &[u8] = include_bytes!("../templates/ClassTemplates/Class.h.template");
const CLASS_CPP_TEMPLATE: &[u8] = include_bytes!("../templates/ClassTemplates/Class.cpp.template");
const COMPONENT_H_TEMPLATE: &[u8] = include_bytes!("../templates/ClassTemplates/Component.h.template");
const COMPONENT_CPP_TEMPLATE: &[u8] = include_bytes!("../templates/ClassTemplates/Component.cpp.template");

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

