use std::collections::HashMap;
use std::env;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::{Command, Stdio};

use rood::cli::OutputManager;
use rood::{Cause, CausedResult, Error};

use crate::builder::{BuildMode, Builder};
use crate::lease::BuildConfig;
use crate::Project;

pub struct RustBuilder {
    output: OutputManager,
}

impl RustBuilder {
    pub fn new() -> RustBuilder {
        RustBuilder {
            // TODO: Take output manager as param.
            output: OutputManager::new(true),
        }
    }
}

impl Builder for RustBuilder {
    fn build(
        &self,
        path: &PathBuf,
        config: &BuildConfig,
        mode: &BuildMode,
    ) -> CausedResult<PathBuf> {
        let mut cmd = Command::new("cargo");
        let mut cmd_base = cmd.arg("build").arg("--all-features");

        let out_dir: String;
        match mode {
            BuildMode::Release => {
                cmd_base = cmd_base.arg("--release");
                out_dir = String::from("release");
            }
            BuildMode::Debug => {
                out_dir = String::from("debug");
            }
        }

        let actual_cmd = cmd_base
            .stdout(Stdio::piped())
            //.stderr(Stdio::piped())
            .spawn()?;

        let stdout = actual_cmd.stdout.ok_or_else(|| {
            Error::new(
                Cause::GeneralError("SubprocessError".to_string()),
                "Could not attach to stdout",
            )
        })?;

        /*
        TODO: Properly attach stderr.
        let stderr = actual_cmd.stdout.ok_or_else(Error::new(
            Cause::GeneralError("SubprocessError".to_string()),
            "Could not attach to stderr",
        ))?;
        */

        let reader = BufReader::new(stdout);
        reader
            .lines()
            .filter_map(|line| line.ok())
            .for_each(|line| self.output.progress(&line, 3));

        Ok(path.join("target").join(&out_dir))
    }

    fn clean(&self) -> CausedResult<()> {
        Ok(())
    }
}
