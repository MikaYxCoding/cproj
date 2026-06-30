use std::{
    error, fmt,
    fs::{self, File},
    io::{self, Write},
    path::{self, Path},
    process::Command,
};

use crate::cli;

#[derive(Debug)]
pub enum Languages {
    // C
    C,
    // C++
    CXX,
    // both C and C++
    CXXC,
}
impl cli::FromCli for Languages {
    fn value_hint() -> String {
        "C, CXX, or both".to_owned()
    }

    fn try_from(buf: &str) -> Option<Self> {
        match buf {
            "c" => Some(Languages::C),
            "cxx" => Some(Languages::CXX),
            "c cxx" => Some(Languages::CXXC),
            "cxx c" => Some(Languages::CXXC),
            _ => None,
        }
    }
}

impl fmt::Display for Languages {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Languages::C => write!(f, "C"),
            Languages::CXX => write!(f, "CXX"),
            Languages::CXXC => write!(f, "C CXX"),
        }
    }
}

#[derive(Debug)]
pub struct LanguageStandard {
    pub c: Option<u32>,
    pub cxx: Option<u32>,
}
impl fmt::Display for LanguageStandard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.c.is_none() && self.cxx.is_none() {
            return write!(f, "undefined");
        }

        if let Some(c) = self.c {
            write!(f, "C STANDARD: {c}")?;
        }

        if let Some(cxx) = self.cxx {
            write!(
                f,
                "{}CXX STANDARD: {cxx}",
                if self.c.is_some() { ", " } else { "" }
            )?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub enum CMakeTargetType {
    Executable,
    StaticLib,
    SharedLib,
    InterfaceLib,
}
impl cli::FromCli for CMakeTargetType {
    fn value_hint() -> String {
        "executable, static, shared, interface".to_owned()
    }

    fn try_from(buf: &str) -> Option<Self> {
        match buf {
            "executable" => Some(CMakeTargetType::Executable),
            "static" => Some(CMakeTargetType::StaticLib),
            "shared" => Some(CMakeTargetType::SharedLib),
            "interface" => Some(CMakeTargetType::InterfaceLib),
            _ => None,
        }
    }
}

impl fmt::Display for CMakeTargetType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CMakeTargetType::Executable => write!(f, "executable"),
            CMakeTargetType::StaticLib => write!(f, "static"),
            CMakeTargetType::SharedLib => write!(f, "shared"),
            CMakeTargetType::InterfaceLib => write!(f, "interface"),
        }
    }
}

#[derive(Debug)]
pub struct CMakeTargetConfig {
    pub name: String,
    pub target_type: CMakeTargetType,
}
impl fmt::Display for CMakeTargetConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "target name: {}", self.name)?;
        write!(f, "target type: {}", self.target_type)
    }
}

#[derive(Debug)]
pub struct GeneratorConfig {
    pub folder_name: String,
    pub project_name: String,
    pub project_version: String,
    pub languages: Languages,
    pub standard: LanguageStandard,
    pub main_file: String,
    pub cmake_version: String,
    pub use_style: bool,
    pub cmake_target: Option<CMakeTargetConfig>,
    pub run_git_init: bool,
}
impl fmt::Display for GeneratorConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "folder name: {}", self.folder_name)?;
        writeln!(f, "project name: {}", self.project_name)?;
        writeln!(f, "project version: {}", self.project_version)?;
        writeln!(f, "languages: {}", self.languages)?;
        writeln!(f, "language standard: {}", self.standard)?;
        writeln!(f, "main file: {}", self.main_file)?;
        writeln!(f, "CMake version: {}", self.cmake_version)?;
        writeln!(
            f,
            "use Mika's code formatting style: {}",
            if self.use_style { "yes" } else { "no" }
        )?;
        if let Some(target) = &self.cmake_target {
            writeln!(f, "{target}")?;
        }
        writeln!(
            f,
            "run git init: {}",
            if self.run_git_init { "yes" } else { "no" }
        )?;

        Ok(())
    }
}

#[derive(Debug)]
pub enum GenerationError {
    IoError(io::Error),
    GitError,
}
impl error::Error for GenerationError {}
impl fmt::Display for GenerationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GenerationError::IoError(error) => write!(f, "{error}"),
            GenerationError::GitError => write!(f, "git did not exit successfully"),
        }
    }
}

impl From<io::Error> for GenerationError {
    fn from(value: io::Error) -> Self {
        GenerationError::IoError(value)
    }
}

pub fn generate(config: GeneratorConfig) -> Result<(), GenerationError> {
    let folder_path = path::absolute(Path::new("./").join(config.folder_name))?;
    fs::create_dir(&folder_path)?;

    if config.run_git_init {
        if !Command::new("git")
            .current_dir(&folder_path)
            .arg("init")
            .status()?
            .success()
        {
            return Err(GenerationError::GitError);
        }

        let gitignore = include_str!("../template/.gitignore");
        fs::write(folder_path.join(".gitignore"), gitignore)?;
    }

    if config.use_style {
        let format = include_str!("../template/.clang-format");
        fs::write(folder_path.join(".clang-format"), format)?;
    }

    let mut cmakelists = File::create(folder_path.join("CMakeLists.txt"))?;
    cmakelists.write_all(
        format!(
            "cmake_minimum_required(VERSION {})\n\n",
            config.cmake_version
        )
        .as_bytes(),
    )?;

    cmakelists.write_all(
        format!(
            "project({} VERSION {} LANGUAGES {})\n",
            config.project_name, config.project_version, config.languages
        )
        .as_bytes(),
    )?;
    if let Some(c) = config.standard.c {
        cmakelists.write_all(format!("set(CMAKE_C_STANDARD {c})\n").as_bytes())?;
        cmakelists.write_all(format!("set(CMAKE_C_STANDARD_REQUIRED ON)\n").as_bytes())?;
    }

    if let Some(cxx) = config.standard.cxx {
        cmakelists.write_all(format!("set(CMAKE_CXX_STANDARD {cxx})\n").as_bytes())?;
        cmakelists.write_all(format!("set(CMAKE_CXX_STANDARD_REQUIRED ON)\n").as_bytes())?;
    }

    cmakelists
        .write_all(format!("\nset(sources\n    src/{}\n)\n\n", config.main_file).as_bytes())?;

    if let Some(target) = config.cmake_target {
        match target.target_type {
            CMakeTargetType::Executable => {
                cmakelists.write_all(
                    format!("add_executable({} ${{sources}})\n", target.name).as_bytes(),
                )?;
                cmakelists
                    .write_all(format!("set_target_properties({}\n    PROPERTIES\n       EXPORT_COMPILE_COMMANDS ON\n)\n", target.name).as_bytes())?;
                cmakelists
                    .write_all(format!("if(NOT MSVC)\n    target_compile_options({} PRIVATE -Wall -Wextra)\nelse()\n    target_compile_options({} PRIVATE /W4)\nendif()\n", target.name, target.name).as_bytes())?;
                cmakelists.write_all(
                    format!("# target_compile_definitions({} ...)\n", target.name).as_bytes(),
                )?;
                cmakelists.write_all(
                    format!("# target_include_directories({} ...)\n", target.name).as_bytes(),
                )?;
                cmakelists.write_all(
                    format!("# target_link_libraries({} ...)\n", target.name).as_bytes(),
                )?;
            }
            CMakeTargetType::StaticLib => {
                cmakelists.write_all(
                    format!("add_library({} STATIC ${{sources}})\n", target.name).as_bytes(),
                )?;
                cmakelists
                    .write_all(format!("set_target_properties({}\n    PROPERTIES\n       EXPORT_COMPILE_COMMANDS ON\n)\n", target.name).as_bytes())?;
                cmakelists
                    .write_all(format!("if(NOT MSVC)\n    target_compile_options({} PUBLIC -Wall -Wextra)\nelse()\n    target_compile_options({} PUBLIC /W4)\nendif()\n", target.name, target.name).as_bytes())?;
                cmakelists.write_all(
                    format!("# target_compile_definitions({} ...)\n", target.name).as_bytes(),
                )?;
                cmakelists.write_all(
                    format!("# target_include_directories({} ...)\n", target.name).as_bytes(),
                )?;
                cmakelists.write_all(
                    format!("# target_link_libraries({} ...)\n", target.name).as_bytes(),
                )?;
            }
            CMakeTargetType::SharedLib => {
                cmakelists.write_all(
                    format!("add_library({} SHARED ${{sources}})\n", target.name).as_bytes(),
                )?;
                cmakelists
                    .write_all(format!("set_target_properties({}\n    PROPERTIES\n       EXPORT_COMPILE_COMMANDS ON\n)\n", target.name).as_bytes())?;
                cmakelists
                    .write_all(format!("if(NOT MSVC)\n    target_compile_options({} PUBLIC -Wall -Wextra)\nelse()\n    target_compile_options({} PUBLIC /W4)\nendif()\n", target.name, target.name).as_bytes())?;
                cmakelists.write_all(
                    format!("# target_compile_definitions({} ...)\n", target.name).as_bytes(),
                )?;
                cmakelists.write_all(
                    format!("# target_include_directories({} ...)\n", target.name).as_bytes(),
                )?;
                cmakelists.write_all(
                    format!("# target_link_libraries({} ...)\n", target.name).as_bytes(),
                )?;
            }
            CMakeTargetType::InterfaceLib => {
                cmakelists
                    .write_all(format!("add_library({} INTERFACE)\n", target.name).as_bytes())?;
                cmakelists.write_all(
                    format!("# target_include_directories({} ...)\n", target.name).as_bytes(),
                )?;
                cmakelists.write_all(
                    format!("# target_link_libraries({} ...)\n", target.name).as_bytes(),
                )?;
            }
        }
    }

    fs::create_dir(folder_path.join("src"))?;
    let main_file = folder_path.join("src").join(config.main_file);

    match main_file.extension() {
        Some(ext) => {
            if ext == "c" {
                fs::write(main_file, include_str!("../template/main.c"))?
            } else if ext == "cpp" || ext == "cxx" {
                fs::write(main_file, include_str!("../template/main.cpp"))?
            } else {
                fs::write(main_file, "")?
            }
        }
        None => fs::write(main_file, "")?,
    }

    Ok(())
}
