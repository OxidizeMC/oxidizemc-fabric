use crate::{
    BINDING_PATHS, PS, JOG_BIN_PATH, FORM_BIN_PATH,
    tools::{pipe_cmd_output, run_cmd, run_cmd_stdout},
};
use clap::ValueEnum;
use std::{fs::File, path::Path, io::Read, process::Stdio};
use regex::Regex;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Binding {
    All,
}

pub fn run(binding: Binding) {
    match binding {
        Binding::All => {
            for path in BINDING_PATHS {
                generate_bindings(path);
            }
        },
    }
}

fn generate_bindings(binding_path: &str) {
    println!(
        "Generating bindings for '{}'",
        std::path::Path::new(binding_path)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
    );

    println!("  - Generating bindings...");
    pipe_cmd_output(
        JOG_BIN_PATH,
        &["-vv", "generate"],
        Some(binding_path),
        format!("{}{PS}java-oxide-output.txt", binding_path),
    ).unwrap();

    println!("    - Finding missing classes...");
    let mut file: File = File::open(Path::new(&format!("{}{PS}java-oxide-output.txt", binding_path))).unwrap();
    let mut buf: String = String::new();
    file.read_to_string(&mut buf).unwrap();
    drop(file);
    let re: Regex = Regex::new(r#"ERROR: missing class for field(?:\/argument)? type: "(.+)""#).unwrap();
    let mut classes: Vec<&str> = Vec::new();
    for (_, [class]) in re.captures_iter(&buf).map(|c| c.extract()) {
        if !classes.contains(&class) {
            classes.push(class);
        }
    }
    if classes.len() > 0 {
        classes.sort();
        println!("    Missing classes:");
        println!("      - {}", classes.join("\n      - "));
    }

    println!("  - Separating bindings...");
    run_cmd_stdout(
        FORM_BIN_PATH,
        &["--input", "full-bindings.rs", "--outdir", "src"],
        Some(binding_path),
        Stdio::null(),
        Some(Stdio::null()),
    ).unwrap();

    println!("  - Formatting bindings...");
    run_cmd(
        "cargo",
        &[
            "fmt",
            "--",
            "--config-path",
            format!("..{PS}rustfmt.toml").as_str(),
        ],
        Some(binding_path),
    ).unwrap();

    println!("  - Checking code...");
    pipe_cmd_output(
        "cargo",
        &["check"],
        Some(binding_path),
        format!("{}{PS}cargo-check-output.txt", binding_path),
    ).unwrap();

    println!("Done\n");
}
