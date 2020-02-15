use std::path::PathBuf;

use crate::Bumper;

#[derive(Default)]
pub struct GoBumper {}

impl GoBumper {
    pub fn new() -> GoBumper {
        GoBumper {}
    }
}

impl Bumper for GoBumper {
    fn get_files_with_version(&self) -> Vec<PathBuf> {
        Vec::new()
    }
}
