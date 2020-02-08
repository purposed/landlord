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

        let mut actual_cmd = cmd_base
            //.stderr(Stdio::piped())
            //.stdout(Stdio::piped())
            .spawn()?;

        /*
        let stdout = actual_cmd.stdout.ok_or_else(|| {
            Error::new(
                Cause::GeneralError("SubprocessError".to_string()),
                "Could not attach to stdout",
            )
        })?;

        //TODO: Properly attach stderr.
        let stderr = actual_cmd.stderr.ok_or_else(|| Error::new(
            Cause::GeneralError("SubprocessError".to_string()),
            "Could not attach to stderr",
        ))?;

        let reader = BufReader::new(stdout);
        reader
            .lines()
            .filter_map(|line| line.ok())
            .for_each(|line| self.output.progress(&line, 3));

        let errreader = BufReader::new(stderr);
        errreader
            .lines()
            .filter_map(|line| line.ok())
            .for_each(|line| self.output.error(&line, 3));
        */
        let status = actual_cmd.wait()?;
        if status.success() {
            Ok(path.join("target").join(&out_dir))
        } else {
            let code = status.code().unwrap_or(1);
            Err(Error::new(Cause::GeneralError("SubprocessError".to_string()), &format!("Status: {}", code)))
        }

    }

    fn clean(&self) -> CausedResult<()> {
        Ok(())
    }
}
