use async_trait::async_trait;
use std::convert::TryFrom;
use std::io;
use url::Url;

use crate::connection::Connection;

/// A `bb8::ManageConnection` for `memcache_async::ascii::Protocol`.
#[derive(Clone, Debug)]
pub struct ConnectionManager {
    url: Url,
}

impl ConnectionManager {
    pub fn new(url: Url) -> ConnectionManager {
        ConnectionManager { url }
    }
}

impl TryFrom<&str> for ConnectionManager {
    type Error = url::ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self::new(Url::parse(value)?))
    }
}

#[async_trait]
impl bb8::ManageConnection for ConnectionManager {
    type Connection = Connection;
    type Error = io::Error;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        Connection::connect(&*self.url.socket_addrs(|| None)?).await
    }

    async fn is_valid(
        &self,
        conn: &mut bb8::PooledConnection<'_, Self>,
    ) -> Result<(), Self::Error> {
        todo!()
        // conn.version().await.map(|_| ())
    }

    fn has_broken(&self, _: &mut Self::Connection) -> bool {
        false
    }
}
