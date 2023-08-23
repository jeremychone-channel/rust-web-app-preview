//! `base` app library.
//!
//! Design:
//!
//! - The `base` lib crate provides foundational utility functions for other libraries and services.
//! - One of the key objectives of these utilities is to standardize basic tasks
//!   (e.g., b64 encode/decode, time parsing/formatting, and token manipulation)
//!   across all higher-level application libraries and services.
//! - By design, base utilities should remain as minimalist as possible,
//!   avoiding access to high-level constructs like `Config`, databases, and other remote services.
//! - Each utility sub-module has its own `Error`, allowing higher-level modules
//!   to only address the errors of the utility modules they interact with.
//! - If a utility requires such access, then the `core` lib crate is likely a more suitable location.

pub mod b64;
pub mod env;
pub mod time;
pub mod token;
