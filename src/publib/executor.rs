use crate::{BuildMode, Builder, Project, ProjectStack};

use rood::cli::OutputManager;
use rood::CausedResult;
use std::collections::HashMap;

type IBuilder = Box<dyn Builder>;

#[derive(Default)]
pub struct Executor {
    builders: HashMap<ProjectStack, IBuilder>,
}

impl Executor {
    pub fn new() -> Executor {
        Executor {
            builders: HashMap::new(),
        }
    }

    pub fn add_builder(&mut self, stack: ProjectStack, b: IBuilder) {
        self.builders.insert(stack, b);
    }

    pub fn build(
        &self,
        project: &Project,
        mode: BuildMode,
        output: &OutputManager,
    ) -> CausedResult<()> {
        let builder = self.builders.get(&project.lease.stack).unwrap();
        output.step(&format!("Building project [{}]", project.lease.name));

        for config in project.lease.builds.iter() {
            let stack_output = output.push();

            stack_output.step(&format!(
                "Building {}-{}...",
                config.platform, config.architecture
            ));
            let build_path = builder.build(&project.path, config, &mode)?;
            stack_output.push().debug(&format!(
                "Build can be found in {}",
                build_path.to_str().unwrap()
            ));
            stack_output.step("OK");
        }

        output.success(&format!("Build of [{}] complete", project.lease.name));

        Ok(())
    }
}
