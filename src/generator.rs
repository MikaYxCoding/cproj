use std::fmt;

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

        Ok(())
    }
}
