mod constants;
mod executor;
mod interface;
mod lease;
mod mode;
mod project;
mod stack;
pub mod stacks;

pub use executor::Executor;
pub use interface::{Builder, Validator};
pub use lease::{BuildConfig, Lease};
pub use mode::BuildMode;
pub use project::Project;
pub use stack::ProjectStack;

pub fn default_executor() -> Executor {
    let rust_interface = Box::from(stacks::RustBuilder::new());

    let mut ex = Executor::new();
    ex.add_builder(ProjectStack::Rust, rust_interface);

    ex
}
