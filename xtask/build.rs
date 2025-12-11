use std::{
    fs::File,
    io::{BufWriter, Write},
    path::{Path, PathBuf},
};

/// Path Separator
pub const PS: char = std::path::MAIN_SEPARATOR;

fn bindpath(name: &str) -> String {
    std::fs::canonicalize(format!("{}{PS}..{PS}{}", env!("CARGO_MANIFEST_DIR"), name))
        .unwrap()
        .to_string_lossy()
        .into_owned()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path: PathBuf = Path::new(&std::env::var("OUT_DIR").unwrap()).join("codegen.rs");
    let mut file: BufWriter<File> = BufWriter::new(File::create(path).unwrap());
    writeln!(
        &mut file,
        "pub const BINDPATH_JSYS: &str = {:?};
        pub const BINDING_PATHS: [&str; 1] = [BINDPATH_JSYS];",
        bindpath("jsys"),
    )
    .unwrap();
    file.flush().unwrap();
    Ok(())
}
