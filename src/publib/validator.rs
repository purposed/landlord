use std::collections::HashMap;

use anyhow::Result;

use rood::cli::OutputManager;

use crate::lease::Validation;
use crate::stacks::go::GoValidator;
use crate::stacks::rust::RustValidator;
use crate::subprocess;
use crate::{Project, ProjectStack, Validator};

pub type IValidator = Box<dyn Validator>;

pub struct MetaValidator {
    default_validators: HashMap<ProjectStack, IValidator>,
}

impl MetaValidator {
    pub fn new(validators: HashMap<ProjectStack, IValidator>) -> MetaValidator {
        MetaValidator {
            default_validators: validators,
        }
    }

    pub fn default() -> MetaValidator {
        let mut hsh: HashMap<ProjectStack, IValidator> = HashMap::new();
        hsh.insert(ProjectStack::Rust, Box::from(RustValidator::new()));
        hsh.insert(ProjectStack::Go, Box::from(GoValidator::new()));

        MetaValidator::new(hsh)
    }

    fn run_validation(&self, validation: &Validation, output: &OutputManager) -> Result<()> {
        output.step(&format!("[{}]", validation.name));

        let arguments: Vec<&str> = validation.command.iter().map(|s| &**s).collect();
        let pushed = output.push();
        subprocess::run_cmd(arguments, |line| pushed.error(line))?;

        output.success(&format!("[{}] - OK", validation.name));
        Ok(())
    }

    pub fn validate(&self, project: &Project, output: &OutputManager) -> Result<()> {
        output.step("[Validate]");

        // First -- run default validations.
        let default_validator = self.default_validators.get(&project.lease.stack).unwrap();
        let mut default_validations = default_validator.get_default_validations();

        // Second -- If user validations are defined, run them too.
        if let Some(extra_validations) = &project.lease.additional_validations {
            default_validations.extend(extra_validations.iter().cloned());
        }

        for validation in default_validations.iter() {
            self.run_validation(validation, &output.push())?;
        }

        output.success("[Validate] - OK");

        Ok(())
    }
}
