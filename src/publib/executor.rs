use crate::{BuildMode, Builder, Project, ProjectStack};

use rood::cli::OutputManager;
use rood::CausedResult;
use std::collections::HashMap;

type IBuilder = Box<dyn Builder>;

pub struct Executor {
    builders: HashMap<ProjectStack, IBuilder>,

    output: OutputManager,
}

impl Executor {
    pub fn new() -> Executor {
        Executor {
            builders: HashMap::new(),
            output: OutputManager::new(true), // TODO: Parametrize
        }
    }

    pub fn add_builder(&mut self, stack: ProjectStack, b: IBuilder) {
        self.builders.insert(stack, b);
    }

    pub fn build(&self, project: &Project, mode: BuildMode) -> CausedResult<()> {
        let builder = self.builders.get(&project.lease.stack).unwrap();
        self.output
            .step(&format!("Building project [{}]", project.lease.name), 0);

        for config in project.lease.builds.iter() {
            self.output.step(
                &format!("Building {}-{}...", config.platform, config.architecture),
                1,
            );
            let build_path = builder.build(&project.path, config, &mode)?;
            self.output.debug(
                &format!("Build can be found in {}", build_path.to_str().unwrap()),
                2,
            );
            self.output.step("OK", 1);
        }

        self.output
            .success(&format!("Build of [{}] complete", project.lease.name), 0);

        Ok(())
    }
}
