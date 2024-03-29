use std::collections::HashMap;
use std::path::PathBuf;

use anyhow::Result;

use rood::cli::OutputManager;
use rood::sys::{Architecture, Platform};

use crate::stacks::go::GoBuilder;
use crate::stacks::rust::RustBuilder;
use crate::{BuildConfig, BuildMode, Builder, Project, ProjectStack};

type IBuilder = Box<dyn Builder>;

#[derive(Default)]
pub struct MetaBuilder {
    builders: HashMap<ProjectStack, IBuilder>,
}

impl MetaBuilder {
    pub fn new(map: HashMap<ProjectStack, IBuilder>) -> MetaBuilder {
        MetaBuilder { builders: map }
    }

    pub fn default() -> MetaBuilder {
        let mut hsh: HashMap<ProjectStack, Box<dyn Builder>> = HashMap::new();
        hsh.insert(ProjectStack::Rust, Box::from(RustBuilder::new()));
        hsh.insert(ProjectStack::Go, Box::from(GoBuilder::new()));

        MetaBuilder::new(hsh)
    }

    pub fn build(
        &self,
        project: &Project,
        mode: BuildMode,
        output: &OutputManager,
    ) -> Result<HashMap<PathBuf, BuildConfig>> {
        let builder = self.builders.get(&project.lease.stack).unwrap();
        output.step(&format!("[Build/{:?}] - {}", mode, project.lease.name));

        let mut hsh_map = HashMap::new();

        let current_arch = Architecture::detect();
        let current_platform = Platform::detect();

        for config in project.lease.builds.iter() {
            let stack_output = output.push();

            stack_output.step(&format!("[{}-{}]", config.platform, config.architecture));
            if config.architecture != current_arch || config.platform != current_platform {
                stack_output.push().step("Skip");
                continue;
            }
            let build_path = builder.build(project, config, &mode)?;
            stack_output.push().debug(&format!(
                "Build can be found in {}",
                build_path.to_str().unwrap()
            ));
            hsh_map.insert(build_path, config.clone());
            stack_output.step(&format!(
                "[{}-{}] - OK",
                config.platform, config.architecture
            ));
        }

        output.success("[Build] - OK");

        Ok(hsh_map)
    }
}
