//! Vinted Rust memcache

extern crate byteorder;
#[cfg(feature = "tls")]
extern crate openssl;
extern crate r2d2;
extern crate rand;
extern crate url;

mod client;
mod connection;
mod error;
mod parser;
mod protocol;
mod stream;
mod value;

pub use crate::client::Client;
pub use crate::connection::ConnectionManager;
pub use crate::error::{ClientError, CommandError, MemcacheError, ServerError};
pub use crate::stream::Stream;
pub use crate::value::{FromMemcacheValue, FromMemcacheValueExt, ToMemcacheValue};
pub use r2d2::Error as PoolError;

/// R2D2 connection pool
pub type Pool = r2d2::Pool<connection::ConnectionManager>;
