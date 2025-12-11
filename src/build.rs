use std::{
    env, fs,
    path::{Path, PathBuf},
};
use toml::Table;

pub fn configure() {
    let manifest_path: PathBuf =
        Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("Cargo.toml");
    let manifest: String = fs::read_to_string(manifest_path).unwrap();
    let toml: Table = manifest.parse().unwrap();
    let java_package: &str = toml["package"]["metadata"]["java-package"]
        .as_str()
        .expect("Cargo Manifest is missing required property `package.metadata.java-package`");

    println!("cargo:rustc-env=JAVA_PACKAGE={}", java_package);
}
