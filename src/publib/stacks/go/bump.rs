use anyhow::Result;

use semver::Version;

use crate::{Bumper, Project};

#[derive(Default)]
pub struct GoBumper {}

impl GoBumper {
    pub fn new() -> GoBumper {
        GoBumper {}
    }
}

impl Bumper for GoBumper {
    fn bump_all(&self, _: &Project, _: &Version, _: &Version) -> Result<()> {
        Ok(())
    }
}
