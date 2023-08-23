//! `core` app library.
//!
//! Design:
//!
//! 1. The `core` library provides core functionalities for application services (e.g., Web Service and Job Service).
//! 2. Key sub-modules within this library include the `Ctx`, `Model`, and potentially the `Event` layer,
//!    which offer essential implementations for accessing application data and services.
//!
//!
//! Notes:
//!
//! 1. The `core` library also houses the `config` module, which is presently shared across all service codes.
//! 2. In the future, the configuration may be divided into distinct modules per primary library and service,
//!    based on each service's needs.
//! 3. Currently, `pwd` exists as a sub-module of the `core` library. However, it might be separated into
//!    its individual module if required.
//!

mod config;
pub mod ctx;
pub mod model;
pub mod pwd;

// #[cfg(test)] // Commented during early development.
pub mod _dev_utils;

pub use config::config;
