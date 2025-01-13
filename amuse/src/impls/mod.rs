#[cfg(feature = "alloc")]
pub mod alloc;
#[path = "core.rs"]
pub mod core;
#[cfg(feature = "std")]
pub mod std;
