# JuMake: CMake Project Initialization for JUCE

## Overview

JuMake is a command-line tool designed to simplify and accelerate the process of setting up new JUCE projects using CMake. It automates the creation of project structures, initializes Git repositories, clones the JUCE framework as a submodule, allowing developers to focus on their core audio application development rather than project setup logistics.

## Features

* **Quick Project Initialization:** Create a new project structure with a single command.
* **CMake Integration:** Automatically generate a `CMakeLists.txt` file tailored for audio development.
* **Source File Templates:** Set up initial C++ files, including `Main.cpp`, `MainComponent.cpp`, and `MainComponent.h`.
* **Git Integration:** Initialize a Git repository for version control and add the JUCE submodule.
* **JUCE Submodule:** Automatically clones the JUCE framework as a submodule and integrates it into the project.
* **Cross-Platform Compatibility:** Designed to work on Windows, macOS, and Linux.

## Installation

1. **Install Rust:** If you don't have Rust installed, download and install it from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).
2. **Install JuMake:**
```bashV
cargo install jumake
```

## Usage

### Create a New Project

```bash
jumake new <project_name>
```

This command will:
* Ask you, what kind of project do you want to create: GUI Application, Audio Plugin or Console App?
* Create a new directory with the specified `project_name`.
* Generate a basic `CMakeLists.txt` file.
* Create a `src` directory with template C++ files.
* Initialize a Git repository.
* Clone the JUCE framework as a submodule.
* Add the JUCE submodule to the `CMakeLists.txt`.

### Build the Project

```bash
jumake build
```

This command will:
* Create a `jumake_build` directory.
* Run CMake to generate the build files.
* Run CMake to build the project.

### Run the Project

```bash
jumake run
```

This command will:
* Build the project (if it hasn't been built already).
* Run the executable or open the application bundle, depending on the platform and project type.

### Add new Class to your project

```bash
jumake add <class_type> <class_name>
```
<class_type> can be `class` or `component`. Where `class` will be a simple c++-class and `component` will be a JuceComponent.

This command will:
* Add new `<class_name>.cpp` and `<class_name>.h` files in the src directory
* Add the cpp to the `CMakeLists.txt` ready for use.

## Examples

**Create a new JUCE GUI application:**

```bash
jumake new MyJuceApp
```

**Build the project:**

```bash
cd MyJuceApp
jumake build
```

**Run the application:**

```bash
jumake run
```

## Contributing

Contributions are welcome! Please see the `CONTRIBUTING.md` file for guidelines.

This project is licensed under the MIT License - see the `LICENSE.md` file for details.
