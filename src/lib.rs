pub mod simple_engine; // Ensure this line is present

// Define the prelude module
pub mod prelude {
    pub use super::simple_engine::engine::Engine;
}
