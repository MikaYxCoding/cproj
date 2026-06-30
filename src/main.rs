use std::process::ExitCode;

use crate::generator::{
    CMakeTargetConfig, CMakeTargetType, GeneratorConfig, LanguageStandard, Languages,
};

mod cli;
mod generator;

fn main() -> ExitCode {
    println!("cproj - C/C++ project generator by Mikayeek");
    println!("v{}", env!("CARGO_PKG_VERSION"));

    let folder_name = cli::input("folder name", None);
    let project_name = cli::input("project name", Some(&folder_name));
    let project_version = cli::input("project version", Some("1.0.0"));

    let languages = cli::input_as("project languages", Some(Languages::CXX));
    let standard = match &languages {
        Languages::C => LanguageStandard {
            c: Some(cli::input_as("c standard", Some(23))),
            cxx: None,
        },
        Languages::CXX => LanguageStandard {
            c: None,
            cxx: Some(cli::input_as("cxx standard", Some(23))),
        },
        Languages::CXXC => LanguageStandard {
            c: Some(cli::input_as("c standard", Some(23))),
            cxx: Some(cli::input_as("cxx standard", Some(23))),
        },
    };

    let main_file = match &languages {
        Languages::C => cli::input("main file", Some("main.c")),
        Languages::CXX | Languages::CXXC => cli::input("main file", Some("main.cpp")),
    };

    let cmake_version = cli::input("CMake version", Some("3.31"));
    let use_style = cli::question("use Mika's code formatting style", Some(true));

    let cmake_target = if cli::question("create a cmake target", Some(true)) {
        let name = cli::input("target name", Some(&folder_name));
        let target_type = cli::input_as("target type", Some(CMakeTargetType::Executable));

        Some(CMakeTargetConfig { name, target_type })
    } else {
        None
    };

    let run_git_init = cli::question("run git init", Some(true));

    let config = GeneratorConfig {
        folder_name,
        project_name,
        project_version,
        languages,
        standard,
        main_file,
        cmake_version,
        use_style,
        cmake_target,
        run_git_init,
    };
    println!("\n----------\n{config}");

    if cli::question("is this okay", Some(true)) {
        println!("generating...");
        if let Err(err) = generator::generate(config) {
            eprintln!("failed to generate project: {}", err);
            return ExitCode::FAILURE;
        }

        println!("all done!");
    }

    ExitCode::SUCCESS
}
