pub use self::error::{Error, Result};

pub mod error;
pub mod mw_auth;
pub mod mw_res_mapper;
pub mod routes_login;
pub mod routes_static;
pub mod rpc;

pub const AUTH_TOKEN: &str = "auth-token";
