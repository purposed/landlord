use std::path::PathBuf;

use crate::Bumper;

#[derive(Default)]
pub struct RustBumper {}

impl RustBumper {
    pub fn new() -> RustBumper {
        RustBumper {}
    }
}

impl Bumper for RustBumper {
    fn get_files_with_version(&self) -> Vec<PathBuf> {
        let mut v = Vec::new();

        v.push(PathBuf::from("Cargo.toml"));

        v
    }
}
