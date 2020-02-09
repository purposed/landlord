use std::path::PathBuf;
use std::process::{Command, Stdio};

use rood::{Cause, CausedResult, Error};

use crate::subprocess;
use crate::{BuildConfig, BuildMode, Builder, Validator};

#[derive(Default)]
pub struct RustBuilder {}

impl RustBuilder {
    pub fn new() -> RustBuilder {
        RustBuilder {}
    }
}

impl Builder for RustBuilder {
    fn build(
        &self,
        path: &PathBuf,
        _config: &BuildConfig,
        mode: &BuildMode,
    ) -> CausedResult<PathBuf> {
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
        Ok(path.join("target").join(out_dir))
    }

    fn clean(&self) -> CausedResult<()> {
        unimplemented!();
    }
}
