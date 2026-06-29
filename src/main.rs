mod cli;
mod generator {
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
}

fn main() {
    println!("cproj - C/C++ project generator by Mikayeek");
    println!("v{}", env!("CARGO_PKG_VERSION"));

    let folder_name = cli::input("folder name", None);
    let project_name = cli::input("project name", Some(&folder_name));
    let project_version = cli::input("project version", Some("1.0.0"));
    let cmake_version = cli::input("CMake version", Some("3.31"));
    let use_style = cli::question("use Mika's code formatting style", Some(true));
}
