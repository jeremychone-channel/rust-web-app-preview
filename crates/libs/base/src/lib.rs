//! Design:
//!
//! - The `base` lib crate encompasses the essential utilities for other libs and services.
//! - Each utility sub-module has its own `Error`, enabling higher-level modules to manage only the errors of the utility modules they utilize.
//! - By design, base utilities should remain as minimalist as possible, avoiding access to high-level constructs like `Config`, databases, and other remote services.
//! - If a utility requires such access, then the `core` lib crate is likely a more suitable location.

pub mod b64;
pub mod env;
pub mod time;
pub mod token;
