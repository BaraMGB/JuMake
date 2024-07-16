// src/create_files.rs
use std::fs::{self, File};
use std::io::Write;
use indoc::indoc;
use crate::context::Context;

pub fn create_source_files(context: &Context) -> Result<(), Box<dyn std::error::Error>> {
    let src_path = context.project_path.join("src");
    fs::create_dir(&src_path)?;

    let main_cpp_path = src_path.join("main.cpp");
    let mut main_cpp_file = File::create(main_cpp_path)?;
    main_cpp_file.write_all(b"// Your code goes here!")?;
    Ok(())
}
pub fn create_cmakelists(context: &Context) -> Result<(), Box<dyn std::error::Error>> {
    let cmakelists_path = context.project_path.join("CMakeLists.txt");
    let mut cmakelists_file = File::create(cmakelists_path)?;

    let cmake_content = format!(
        indoc! {
            "cmake_minimum_required(VERSION 3.24)
             project({})

             # Add your project's source files and targets here"
        },
        context.project_name
    );

    cmakelists_file
        .write_all(cmake_content.as_bytes())?;
    Ok(())
}
