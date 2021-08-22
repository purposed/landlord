use std::collections::HashMap;
use std::path::PathBuf;

use anyhow::{bail, ensure, Result};

use rood::cli::OutputManager;
use rood::sys::file;
use semver::Version;

use crate::stacks::go::GoBumper;
use crate::stacks::rust::RustBumper;
use crate::{Bumper, Project, ProjectStack};

#[derive(Default)]
pub struct MetaBumper {
    bumpers: HashMap<ProjectStack, IBumper>,
}

type IBumper = Box<dyn Bumper>;

impl MetaBumper {
    pub fn new(bumpers: HashMap<ProjectStack, IBumper>) -> MetaBumper {
        MetaBumper { bumpers }
    }

    pub fn default() -> MetaBumper {
        let mut bump_map: HashMap<ProjectStack, IBumper> = HashMap::new();

        bump_map.insert(ProjectStack::Rust, Box::from(RustBumper::new()));
        bump_map.insert(ProjectStack::Go, Box::from(GoBumper::new()));

        MetaBumper::new(bump_map)
    }

    fn get_default_version_bumps(&self) -> Vec<PathBuf> {
        vec![PathBuf::from("README.md")]
    }

    fn get_new_version_number(mut version: Version, level: &str) -> Result<Version> {
        if level == "none" {
            return Ok(version);
        }

        if level == "patch" {
            version.patch += 1;
            return Ok(version);
        }

        version.patch = 0;

        if level == "minor" {
            version.minor += 1;
            return Ok(version);
        }

        version.minor = 0;

        if level == "major" {
            version.major += 1;
            return Ok(version);
        }

        bail!("Invalid version increment level");
    }

    pub fn bump_version(
        &self,
        project: &mut Project,
        level: &str,
        dry: bool,
        output: &OutputManager,
    ) -> Result<()> {
        output.step("[Version]");

        let pushed = output.push();

        // TODO: Use enum instead of &str for bump level.
        let current_version = project.lease.version.clone();
        let new_version = Self::get_new_version_number(current_version.clone(), level)?;

        ensure!(
            pushed.prompt_yn(
                &format!("Really publish {} => {} ?", current_version, new_version),
                true
            )?,
            "Aborted"
        );

        project.lease.version = new_version.clone();

        if !dry {
            project.lease.save()?;
            pushed.step("Saved new lease version");

            // Bump stack-specific files.
            self.bumpers.get(&project.lease.stack).unwrap().bump_all(
                project,
                &current_version,
                &new_version,
            )?;
            pushed.step("Bumped stack-specific files");

            // Bump generic files.
            let files_to_replace = self.get_default_version_bumps();
            let pattern = format!("{}", current_version);
            let pattern_to = format!("{}", new_version);
            for file_name in files_to_replace {
                let full_path = project.path.join(&file_name);
                file::replace_all(&full_path, &pattern, &pattern_to)?;
                pushed.step(&format!(
                    "Bumped version in {}",
                    full_path.to_str().unwrap()
                ));
            }
        }

        output.success("[Version] - OK");
        Ok(())
    }
}
