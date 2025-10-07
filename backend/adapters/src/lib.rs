pub mod adapter;
pub mod detector;
pub mod x11;
pub mod wayland;
pub mod desktop;

pub use adapter::{Adapter, AdapterTrait};
pub use detector::{detect_environment, Environment};
