use std::collections::HashMap;
use std::path::PathBuf;

use anyhow::{bail, Result};

use rood::cli::OutputManager;
use rood::sys::file;

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
        let mut new_version = project.lease.version.clone();
        match level {
            "major" => new_version.major = new_version.major + 1,
            "minor" => new_version.minor = new_version.minor + 1,
            "patch" => new_version.patch = new_version.patch + 1,
            "none" => {
                pushed.success("No version bump requested");
                return Ok(());
            }
            _ => bail!("Invalid version increment level"),
        }

        if !pushed.prompt_yn(
            &format!("Really Publish {} => {} ?", current_version, new_version),
            true,
        )? {
            bail!("Aborted");
        }
        project.lease.version = new_version.clone();

        if !dry {
            project.lease.save()?;
            pushed.step("Saved new lease version");

            // Bump stack-specific files.
            self.bumpers.get(&project.lease.stack).unwrap().bump_all(
                &project,
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
