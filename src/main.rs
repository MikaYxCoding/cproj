mod cli {
    use std::io::{self, Write};

    #[must_use]
    pub fn input(text: &str, default: Option<&str>) -> String {
        let mut buffer = String::new();

        match default {
            Some(default) => {
                let default = default.to_owned();

                print!("{text} ({default}): ");
                io::stdout().flush().expect("failed to flush stdout");

                io::stdin().read_line(&mut buffer).expect("failed to read");
                buffer = buffer.trim().to_owned();

                if buffer.len() == 0 { default } else { buffer }
            }
            None => {
                while buffer.len() == 0 {
                    print!("{text}: ");
                    io::stdout().flush().expect("failed to flush stdout");

                    io::stdin().read_line(&mut buffer).expect("failed to read");
                    buffer = buffer.trim().to_owned();
                }

                buffer
            }
        }
    }
}

fn main() {
    println!("cproj - C/C++ project generator by Mikayeek");
    println!("v{}", env!("CARGO_PKG_VERSION"));

    let folder_name = cli::input("folder name", None);
    let project_name = cli::input("project name", Some(&folder_name));
    let project_version = cli::input("project version", Some("1.0.0"));
    let cmake_version = cli::input("CMake version", Some("3.31"));
}
