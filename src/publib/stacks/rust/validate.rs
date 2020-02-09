use crate::lease::Validation;
use crate::Validator;
use rood::CausedResult;
use std::path::PathBuf;

pub struct RustValidator {}

impl RustValidator {
    pub fn new() -> RustValidator {
        RustValidator {}
    }
}

impl Validator for RustValidator {
    fn get_default_validations(&self) -> Vec<Validation> {
        let mut validations = Vec::new();

        validations.push(Validation {
            name: String::from("Rust Tests"),
            command: vec![String::from("cargo"), String::from("test")],
        });
        validations.push(Validation {
            name: String::from("Clippy"),
            command: vec![
                String::from("cargo"),
                String::from("clippy"),
                String::from("--"),
                String::from("-Dwarnings"),
            ],
        });

        validations
    }
}
