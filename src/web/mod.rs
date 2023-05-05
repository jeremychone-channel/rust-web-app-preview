pub use self::error::{Error, Result};
use time::OffsetDateTime;
use uuid::Uuid;

pub mod error;
pub mod mw_auth;
pub mod mw_req_stamp;
pub mod mw_res_mapper;
pub mod routes_login;
pub mod routes_static;
pub mod rpc;

pub const AUTH_TOKEN: &str = "auth-token";

/// Request "Stamp" created at the begin of the http request flow by the `mw_req_stamp_resolver` with the following properties:
///
/// - uuid - Unique identifier of the request.
/// - time_in - Equivalent (close enough) of the beginning of the request.
///
#[derive(Debug, Clone)]
pub struct ReqStamp {
	pub uuid: Uuid,
	pub time_in: OffsetDateTime,
}
