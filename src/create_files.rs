// src/create_files.rs
use std::fs::{self, File};
use std::io::Write;
use indoc::indoc;
use crate::context::Context;
use std::path::PathBuf;

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
