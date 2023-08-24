//! `base` app library.
//!
//! Design:
//!
//! - The `base` lib crate provides primitive utilities for other libraries and services.
//! - Its purpose is to standardize basic encoding, parsing, and fundamental application types
//!   across all higher-level application libraries and services.
//!   Examples include b64 encode/decode, time parsing/formatting, and token manipulation.
//! - By design, base utilities should remain as minimalist as possible,
//!   avoiding access to high-level constructs like `Config`, databases, and other resources.
//! - Each utility sub-module has its own `Error`, allowing higher-level modules
//!   to address only the errors of the utility modules they work with.
//! - The `core` lib crate is an appropriate location for high-level functions.
//!

pub mod b64;
pub mod env;
pub mod time;
pub mod token;
