use semver::Version;

use rood::sys::file::ensure_exists;
use rood::CausedResult;

use super::cargo;
use crate::{Bumper, Project};

#[derive(Default)]
pub struct RustBumper {}

impl RustBumper {
    pub fn new() -> RustBumper {
        RustBumper {}
    }
}

impl Bumper for RustBumper {
    fn bump_all(
        &self,
        project: &Project,
        _current_version: &Version,
        next_version: &Version,
    ) -> CausedResult<()> {
        // Bump Cargo.toml
        let cargo_toml_path = project.path.join("Cargo.toml");
        ensure_exists(&cargo_toml_path)?;
        cargo::set_package_version(&cargo_toml_path, next_version)?;
        cargo::update_lock(&cargo_toml_path)?;
        Ok(())
    }
}
