pub mod traits;
pub use traits::Scaffold;
pub mod projects;
pub use projects::*;

mod rust_project;
mod kotlin_project;
mod cpp_project;
mod go_project;