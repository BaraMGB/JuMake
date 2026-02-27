// src/create_files.rs
use crate::context::Context;
use indoc::indoc;
use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

const SOURCES_BEGIN_MARKER: &str = "# JUMAKE_SOURCES_BEGIN";
const SOURCES_END_MARKER: &str = "# JUMAKE_SOURCES_END";

pub fn create_source_files(context: &Context) -> Result<(), Box<dyn std::error::Error>> {
    let src_path = context.project_path.join("src");
    fs::create_dir(&src_path)?;

    match context.template_name.as_deref() {
        Some("GuiApplication") => {
            // Create GUI application files
            create_file_from_template(&src_path, "Main.cpp", MAIN_CPP_TEMPLATE)?;
            create_file_from_template(&src_path, "MainComponent.cpp", MAIN_COMPONENT_CPP_TEMPLATE)?;
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
            create_file_from_template(&src_path, "PluginProcessor.h", PLUGIN_PROCESSOR_H_TEMPLATE)?;
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
pub fn add_class(
    context: &Context,
    element_type: &str,
    element_name: &str,
) -> Result<(), Box<dyn Error>> {
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
        }
        _ => return Err(format!("Invalid element type: {}", element_type).into()),
    };

    // Construct file names for the header and source files.
    let header_file_name = format!("{}.h", adjusted_element_name);
    let cpp_file_name = format!("{}.cpp", adjusted_element_name);

    // Check if the header or source file already exists
    let header_path = src_path.join(&header_file_name);
    let cpp_path = src_path.join(&cpp_file_name);

    if header_path.exists() || cpp_path.exists() {
        return Err(format!(
            "{} '{}' already exists in the project.",
            element_type, adjusted_element_name
        )
        .into());
    }

    // Create files from templates with the adjusted names.
    create_classfile_from_template(
        &src_path,
        &header_file_name,
        header_template,
        &adjusted_element_name,
    )?;
    create_classfile_from_template(
        &src_path,
        &cpp_file_name,
        cpp_template,
        &adjusted_element_name,
    )?;

    // Add the newly created cpp file to CMakeLists.txt.
    let cmakelists_path = src_path.join("CMakeLists.txt");
    add_source_to_cmakelists(&cmakelists_path, &cpp_file_name)?;

    // Confirm the addition of the new class or component.
    println!(
        "{} '{}' added successfully!",
        element_type, adjusted_element_name
    );
    Ok(())
}

fn add_source_to_cmakelists(
    cmakelists_path: &Path,
    cpp_file_name: &str,
) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(cmakelists_path)?;
    if content.contains(cpp_file_name) {
        return Ok(());
    }

    if let Some(updated) = insert_using_markers(&content, cpp_file_name) {
        fs::write(cmakelists_path, updated)?;
        return Ok(());
    }

    if let Some(updated) = insert_into_target_sources_block(&content, cpp_file_name) {
        fs::write(cmakelists_path, updated)?;
        println!("Warning: CMake markers not found; used fallback parsing for source insertion.");
        return Ok(());
    }

    let appended = format!(
        "{content}\n\n# JUMAKE managed sources\ntarget_sources(${{PROJECT_NAME}}\n    PRIVATE\n        {cpp_file_name}\n)\n"
    );
    fs::write(cmakelists_path, appended)?;
    println!("Warning: Could not find target_sources block; appended a new JUMAKE managed block.");
    Ok(())
}

fn insert_using_markers(content: &str, cpp_file_name: &str) -> Option<String> {
    let mut lines: Vec<String> = content.lines().map(str::to_string).collect();
    let begin_idx = lines
        .iter()
        .position(|line| line.contains(SOURCES_BEGIN_MARKER))?;
    let end_idx = lines
        .iter()
        .position(|line| line.contains(SOURCES_END_MARKER))?;
    if end_idx <= begin_idx {
        return None;
    }

    let marker_indent = lines[end_idx]
        .chars()
        .take_while(|c| c.is_whitespace())
        .collect::<String>();
    lines.insert(end_idx, format!("{}{}", marker_indent, cpp_file_name));
    Some(lines.join("\n"))
}

fn insert_into_target_sources_block(content: &str, cpp_file_name: &str) -> Option<String> {
    let mut lines: Vec<String> = content.lines().map(str::to_string).collect();
    let target_idx = lines.iter().position(|line| {
        line.trim_start()
            .starts_with("target_sources(${PROJECT_NAME}")
    })?;

    let mut depth = paren_delta(&lines[target_idx]);
    let mut i = target_idx + 1;
    let mut private_idx: Option<usize> = None;
    let mut block_end = None;

    while i < lines.len() {
        depth += paren_delta(&lines[i]);
        if private_idx.is_none() && lines[i].trim_start().starts_with("PRIVATE") {
            private_idx = Some(i);
        }
        if depth <= 0 {
            block_end = Some(i);
            break;
        }
        i += 1;
    }

    let block_end = block_end?;

    if let Some(private_idx) = private_idx {
        let private_indent = leading_spaces(&lines[private_idx]);
        let source_indent = if private_idx + 1 < lines.len()
            && private_idx + 1 < block_end
            && !lines[private_idx + 1].trim().is_empty()
        {
            leading_spaces(&lines[private_idx + 1])
        } else {
            private_indent + 4
        };

        let mut insertion_idx = private_idx + 1;
        while insertion_idx < block_end {
            let trimmed = lines[insertion_idx].trim_start();
            if trimmed.starts_with(')')
                || trimmed.starts_with("PUBLIC")
                || trimmed.starts_with("INTERFACE")
                || trimmed.starts_with("PRIVATE")
            {
                break;
            }
            insertion_idx += 1;
        }

        lines.insert(
            insertion_idx,
            format!("{:indent$}{}", "", cpp_file_name, indent = source_indent),
        );
        return Some(lines.join("\n"));
    }

    let target_indent = leading_spaces(&lines[target_idx]);
    lines.insert(
        target_idx + 1,
        format!("{:indent$}PRIVATE", "", indent = target_indent + 4),
    );
    lines.insert(
        target_idx + 2,
        format!(
            "{:indent$}{}",
            "",
            cpp_file_name,
            indent = target_indent + 8
        ),
    );
    Some(lines.join("\n"))
}

fn paren_delta(line: &str) -> i32 {
    let opens = line.chars().filter(|&c| c == '(').count() as i32;
    let closes = line.chars().filter(|&c| c == ')').count() as i32;
    opens - closes
}

fn leading_spaces(line: &str) -> usize {
    line.chars().take_while(|c| c.is_whitespace()).count()
}

fn create_classfile_from_template(
    src_path: &Path,
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
    src_path: &Path,
    file_name: &str,
    template: &[u8],
) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(src_path.join(file_name))?;
    file.write_all(template)?;
    println!("Created file: {}", src_path.join(file_name).display());
    Ok(())
}
// Define the templates as byte slices
const MAIN_CPP_TEMPLATE: &[u8] =
    include_bytes!("../templates/GuiApplicationTemplate/Main.cpp.template");
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
const COMPONENT_H_TEMPLATE: &[u8] =
    include_bytes!("../templates/ClassTemplates/Component.h.template");
const COMPONENT_CPP_TEMPLATE: &[u8] =
    include_bytes!("../templates/ClassTemplates/Component.cpp.template");

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

    cmakelists_file.write_all(cmake_content.as_bytes())?;
    Ok(())
}
