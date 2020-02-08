mod builder;
mod constants;
mod executor;
mod lease;
mod project;
mod stack;
pub mod stacks;

pub use builder::{BuildMode, Builder};
pub use executor::Executor;
pub use lease::Lease;
pub use project::Project;
pub use stack::ProjectStack;

pub fn default_executor() -> Executor {
    let rust_interface = Box::from(stacks::RustBuilder::new());

    let mut ex = Executor::new();
    ex.add_builder(ProjectStack::Rust, rust_interface);

    ex
}
