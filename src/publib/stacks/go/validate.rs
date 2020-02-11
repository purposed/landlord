use crate::lease::Validation;
use crate::Validator;

#[derive(Default)]
pub struct GoValidator {}

impl GoValidator {
    pub fn new() -> GoValidator {
        GoValidator {}
    }
}

impl Validator for GoValidator {
    fn get_default_validations(&self) -> Vec<Validation> {
        let mut validations = Vec::new();

        validations.push(Validation {
            name: String::from("Go Test"),
            command: vec![
                String::from("go"),
                String::from("test"),
                String::from("-race"),
                String::from("./..."),
            ],
        });

        validations
    }
}
