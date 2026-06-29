use std::{
    any::Any,
    error, fmt,
    fs::{self, File},
    io::{self, Write},
    path::{self, Path},
    process::{Command, Stdio},
};

use crate::cli;

#[derive(Debug)]
pub enum CMakeTargetType {
    Executable,
    StaticLib,
    SharedLib,
    InterfaceLib,
}
impl TryFrom<&str> for CMakeTargetType {
    type Error = ();

    fn try_from(value: &str) -> Result<CMakeTargetType, Self::Error> {
        match value {
            "executable" => Ok(CMakeTargetType::Executable),
            "static" => Ok(CMakeTargetType::StaticLib),
            "shared" => Ok(CMakeTargetType::SharedLib),
            "interface" => Ok(CMakeTargetType::InterfaceLib),
            _ => Err(()),
        }
    }
}

impl<'a> cli::FromCli<'a> for CMakeTargetType {
    fn value_hint() -> String {
        "executable, static, shared, interface".to_owned()
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
            "project({} VERSION {} LANGUAGES CXX)\n\n",
            config.project_name, config.project_version
        )
        .as_bytes(),
    )?;

    if let Some(target) = config.cmake_target {
        match target.target_type {
            CMakeTargetType::Executable => todo!(),
            CMakeTargetType::StaticLib => todo!(),
            CMakeTargetType::SharedLib => todo!(),
            CMakeTargetType::InterfaceLib => todo!(),
        }
    }

    Ok(())
}
