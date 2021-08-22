use crate::lease::Validation;
use crate::Validator;

#[derive(Default)]
pub struct RustValidator {}

impl RustValidator {
    pub fn new() -> RustValidator {
        RustValidator {}
    }
}

impl Validator for RustValidator {
    fn get_default_validations(&self) -> Vec<Validation> {
        vec![
            Validation {
                name: String::from("Rust Tests"),
                command: vec![
                    String::from("cargo"),
                    String::from("test"),
                    String::from("--all-features"),
                ],
            },
            Validation {
                name: String::from("Clippy"),
                command: vec![
                    String::from("cargo"),
                    String::from("clippy"),
                    String::from("--"),
                    String::from("-Dwarnings"),
                ],
            },
        ]
    }
}
