use crate::{
    EXE, FORM_BIN_PATH, FORM_PATH, INSTALL_DIR, JOG_BIN_PATH, JOG_PATH, PS, input,
    tools::{
        run_cmd,
        run_cmd_stdout,
        // run_cmd_stdin, spawn_cmd_stdout, wrap_exit
    },
};
use const_format::formatcp;
use std::{fs::OpenOptions, io::Write, path::Path, process::Stdio};

pub fn run() {
    std::fs::create_dir_all(INSTALL_DIR).unwrap();

    if std::fs::exists(JOG_BIN_PATH).unwrap() {
        let user_input: String = input!("Reinstall 'java-oxide-gen'?  [y/N]: ");
        if user_input.to_lowercase() == "y" {
            println!("Deleting previous 'java-oxide-gen' executable...");
            std::fs::remove_file(JOG_BIN_PATH).unwrap();
            install_jog();
        }
    } else {
        install_jog();
    }
    println!();

    if std::fs::exists(FORM_BIN_PATH).unwrap() {
        let user_input: String = input!("Reinstall 'form'?  [y/N]: ");
        if user_input.to_lowercase() == "y" {
            println!("Deleting previous 'form' executable...");
            std::fs::remove_file(FORM_BIN_PATH).unwrap();
            install_form();
        }
    } else {
        install_form();
    }
    println!();
}

fn fix_cargo_manifest(dir: &str) {
    let mut manifest: std::fs::File = OpenOptions::new()
        .read(true)
        .append(true)
        .open(Path::new(dir).join("Cargo.toml"))
        .expect("failed to open manifest");
    manifest.lock().expect("failed to lock manifest");
    manifest
        .write_all(b"\n[workspace]\n")
        .expect("failed to write to manifest");
}

fn install_form() {
    println!("Installing 'form'...");

    if std::fs::exists(FORM_PATH).unwrap() {
        println!("  - Removing old source path...");
        std::fs::remove_dir_all(FORM_PATH).unwrap();
    }

    println!("  - Cloning 'OxidizeMC/form'...");
    run_cmd_stdout(
        "git",
        &["clone", "https://github.com/OxidizeMC/form"],
        Some(INSTALL_DIR),
        Stdio::null(),
        Some(Stdio::null()),
    )
    .unwrap();
    fix_cargo_manifest(FORM_PATH);

    println!("  - Building 'form'...");
    run_cmd("cargo", &["build", "-p", "form"], Some(FORM_PATH)).unwrap();

    println!("  - Copying built executable...");
    std::fs::copy(
        formatcp!("{}{PS}target{PS}debug{PS}form{EXE}", FORM_PATH),
        FORM_BIN_PATH,
    )
    .unwrap();

    println!("  - Removing leftovers...");
    run_cmd("cargo", &["clean", "--quiet"], Some(FORM_PATH)).unwrap();
    std::fs::remove_dir_all(FORM_PATH).unwrap();

    println!("Done");
}

fn install_jog() {
    println!("Installing 'java-oxide-gen'...");

    if std::fs::exists(JOG_PATH).unwrap() {
        println!("  - Removing old source path...");
        std::fs::remove_dir_all(JOG_PATH).unwrap();
    }

    println!("  - Cloning 'OxidizeMC/java-oxide'...");
    run_cmd_stdout(
        "git",
        &["clone", "https://github.com/OxidizeMC/java-oxide"],
        Some(INSTALL_DIR),
        Stdio::null(),
        Some(Stdio::null()),
    )
    .unwrap();

    println!("  - Building 'java-oxide-gen'...");
    run_cmd(
        "cargo",
        &["build", "-p", "java-oxide-gen"],
        Some(JOG_PATH),
    )
    .unwrap();

    println!("  - Copying built executable...");
    std::fs::copy(
        formatcp!("{}{PS}target{PS}debug{PS}java-oxide-gen{EXE}", JOG_PATH),
        JOG_BIN_PATH,
    )
    .unwrap();

    println!("  - Removing leftovers...");
    run_cmd("cargo", &["clean", "--quiet"], Some(JOG_PATH)).unwrap();
    std::fs::remove_dir_all(JOG_PATH).unwrap();

    println!("Done");
}
