use crate::generator::{CMakeTargetConfig, CMakeTargetType, GeneratorConfig};

mod cli;
mod generator;

fn main() {
    println!("cproj - C/C++ project generator by Mikayeek");
    println!("v{}", env!("CARGO_PKG_VERSION"));

    let folder_name = cli::input("folder name", None);
    let project_name = cli::input("project name", Some(&folder_name));
    let project_version = cli::input("project version", Some("1.0.0"));
    let cmake_version = cli::input("CMake version", Some("3.31"));
    let use_style = cli::question("use Mika's code formatting style", Some(true));

    let cmake_target = if cli::question("create a cmake target", Some(true)) {
        let name = cli::input("target name", Some(&folder_name));
        let target_type = cli::input_as::<CMakeTargetType>("target type", None);

        Some(CMakeTargetConfig { name, target_type })
    } else {
        None
    };

    let config = GeneratorConfig {
        folder_name,
        project_name,
        project_version,
        cmake_version,
        use_style,
        cmake_target,
    };
    println!("\n----------\n{config}");

    cli::question("is this okay", Some(true));
}
