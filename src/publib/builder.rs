use crate::{BuildConfig, BuildMode, Builder, Project, ProjectStack};

use crate::stacks::go::GoBuilder;
use crate::stacks::rust::RustBuilder;
use rood::cli::OutputManager;
use rood::CausedResult;
use std::collections::HashMap;
use std::path::PathBuf;

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
    ) -> CausedResult<HashMap<PathBuf, BuildConfig>> {
        let builder = self.builders.get(&project.lease.stack).unwrap();
        output.step(&format!("[Build/{:?}] - {}", mode, project.lease.name));

        let mut hsh_map = HashMap::new();

        for config in project.lease.builds.iter() {
            let stack_output = output.push();

            stack_output.step(&format!("[{}-{}]", config.platform, config.architecture));
            let build_path = builder.build(&project, config, &mode)?;
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
