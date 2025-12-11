use clap::{Parser, Subcommand};
use const_format::formatcp;

mod setup;
mod genbinds;
mod tools;

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

/// Executable Extension
#[cfg(not(windows))]
pub const EXE: &str = "";
/// Executable Extension
#[cfg(windows)]
pub const EXE: &str = ".exe";
/// Path Separator
pub const PS: char = std::path::MAIN_SEPARATOR;
pub const INSTALL_DIR: &str = formatcp!("{}{PS}xtask-temp", env!("CARGO_MANIFEST_DIR"));
pub const JOG_PATH: &str = formatcp!("{}{PS}java-oxide", INSTALL_DIR);
pub const JOG_BIN_PATH: &str = formatcp!("{}{PS}java-oxide-bin{EXE}", INSTALL_DIR);
pub const FORM_PATH: &str = formatcp!("{}{PS}form", INSTALL_DIR);
pub const FORM_BIN_PATH: &str = formatcp!("{}{PS}form-bin{EXE}", INSTALL_DIR);

/// Tasks for binding development/generation
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Task to run
    #[clap(subcommand)]
    task: Tasks,
}

/// Task to run
#[derive(Subcommand, Debug)]
enum Tasks {
    /// Setup binding generation
    Setup,
    /// Generate Rust/Java bindings
    #[command(name = "gen-binds")]
    GenBinds {
        /// Bindings to generate
        #[arg(value_enum, default_value_t = genbinds::Binding::All)]
        binding: genbinds::Binding,
    },
}

impl Tasks {
    fn task_name(&self) -> &str {
        match self {
            Tasks::Setup => "setup",
            Tasks::GenBinds { binding: _ } => "gen-binds",
        }
    }
}

fn main() {
    unsafe {
        std::env::set_var("RUST_BACKTRACE", "1");
    }

    let cli: Cli = Cli::parse();
    println!("Running '{}' task...\n", cli.task.task_name());
    match cli.task {
        Tasks::Setup => setup::run(),
        Tasks::GenBinds { binding } => genbinds::run(binding),
    }
    println!("Finished '{}' task", cli.task.task_name())
}
