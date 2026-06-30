use std::{
    fmt,
    io::{self, Write},
};

pub trait FromCli: Sized + fmt::Display {
    fn value_hint() -> String;
    fn try_from(buf: &str) -> Option<Self>;
}

impl FromCli for u32 {
    fn value_hint() -> String {
        "int".to_owned()
    }

    fn try_from(buf: &str) -> Option<Self> {
        str::parse(buf).ok()
    }
}

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

#[must_use]
pub fn question(text: &str, default: Option<bool>) -> bool {
    let mut buffer = String::new();

    match default {
        Some(default) => loop {
            buffer.clear();
            print!("{text} ({})? ", if default { "yes" } else { "no" });
            io::stdout().flush().expect("failed to flush stdout");

            io::stdin().read_line(&mut buffer).expect("failed to read");
            match buffer.to_lowercase().trim() {
                "yes" => break true,
                "no" => break false,
                input if input.len() == 0 => break default,
                _ => println!("expected 'yes' or 'no'"),
            }
        },
        None => loop {
            buffer.clear();
            print!("{text}? ");
            io::stdout().flush().expect("failed to flush stdout");

            io::stdin().read_line(&mut buffer).expect("failed to read");
            match buffer.to_lowercase().trim() {
                "yes" => break true,
                "no" => break false,
                _ => println!("expected 'yes' or 'no'"),
            }
        },
    }
}

#[must_use]
pub fn input_as<T>(text: &str, default: Option<T>) -> T
where
    T: FromCli,
{
    let mut buffer = String::new();

    match default {
        Some(default) => loop {
            print!("{text} [{}] ({default}): ", T::value_hint());
            io::stdout().flush().expect("failed to flush stdout");

            io::stdin().read_line(&mut buffer).expect("failed to read");
            buffer = buffer.to_lowercase();
            let trimmed_buffer = buffer.trim();

            if trimmed_buffer.len() == 0 {
                break default;
            }

            match T::try_from(trimmed_buffer) {
                Some(value) => break value,
                None => {}
            }

            buffer.clear();
        },
        None => loop {
            print!("{text} [{}]: ", T::value_hint());
            io::stdout().flush().expect("failed to flush stdout");

            io::stdin().read_line(&mut buffer).expect("failed to read");

            match T::try_from(buffer.to_lowercase().trim()) {
                Some(value) => break value,
                None => {}
            }

            buffer.clear();
        },
    }
}
