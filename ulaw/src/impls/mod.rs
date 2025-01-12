#[path = "core.rs"]
mod impls_core;

#[cfg(feature = "alloc")]
#[path = "alloc.rs"]
mod impls_alloc;
