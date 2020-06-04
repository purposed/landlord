use std::path::PathBuf;

use anyhow::Result;

use crate::{subprocess, Project};
use crate::{BuildConfig, BuildMode, Builder};

#[derive(Default)]
pub struct RustBuilder {}

impl RustBuilder {
    pub fn new() -> RustBuilder {
        RustBuilder {}
    }
}

impl Builder for RustBuilder {
    fn build(&self, project: &Project, _config: &BuildConfig, mode: &BuildMode) -> Result<PathBuf> {
        let mut cmd = vec!["cargo", "build", "--all-features"];

        let out_dir;

        match mode {
            BuildMode::Release => {
                out_dir = "release";
                cmd.push("--release")
            }
            BuildMode::Debug => {
                out_dir = "debug";
            }
        }
        subprocess::run_cmd(cmd, |l| eprintln!("{}", l))?;
        Ok(project.path.join("target").join(out_dir))
    }

    fn clean(&self) -> Result<()> {
        unimplemented!();
    }
}
