//! APIs for building postgresql compatible servers.

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;

pub use postgres_types::Type;

pub mod auth;
pub mod portal;
pub mod query;
pub mod results;
pub mod stmt;
pub mod store;

pub const DEFAULT_NAME: &str = "POSTGRESQL_DEFAULT_NAME";

#[derive(Debug, Clone, Copy, Default)]
pub enum PgWireConnectionState {
    #[default]
    AwaitingStartup,
    AuthenticationInProgress,
    ReadyForQuery,
    QueryInProgress,
    AwaitingSync,
}

/// Describe a client information holder
pub trait ClientInfo {
    fn socket_addr(&self) -> SocketAddr;

    fn is_secure(&self) -> bool;

    fn state(&self) -> PgWireConnectionState;

    fn set_state(&mut self, new_state: PgWireConnectionState);

    fn metadata(&self) -> &HashMap<String, String>;

    fn metadata_mut(&mut self) -> &mut HashMap<String, String>;
}

/// Client Portal Store
pub trait ClientPortalStore {
    type PortalStore;

    fn portal_store(&self) -> &Self::PortalStore;
}

pub const METADATA_USER: &str = "user";
pub const METADATA_DATABASE: &str = "database";

#[non_exhaustive]
#[derive(Debug)]
pub struct DefaultClient<S, PS> {
    pub socket_addr: SocketAddr,
    pub is_secure: bool,
    pub state: PgWireConnectionState,
    pub metadata: HashMap<String, String>,
    portal_store: store::MemPortalStore<S, PS>,
}

impl<S, PS> ClientInfo for DefaultClient<S, PS> {
    fn socket_addr(&self) -> SocketAddr {
        self.socket_addr
    }

    fn is_secure(&self) -> bool {
        self.is_secure
    }

    fn state(&self) -> PgWireConnectionState {
        self.state
    }

    fn set_state(&mut self, new_state: PgWireConnectionState) {
        self.state = new_state;
    }

    fn metadata(&self) -> &HashMap<String, String> {
        &self.metadata
    }

    fn metadata_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.metadata
    }
}

impl<S, PS> DefaultClient<S, PS> {
    pub fn new(socket_addr: SocketAddr, is_secure: bool) -> DefaultClient<S, PS> {
        DefaultClient {
            socket_addr,
            is_secure,
            state: PgWireConnectionState::default(),
            metadata: HashMap::new(),
            portal_store: store::MemPortalStore::new(),
        }
    }
}

impl<S, PS> ClientPortalStore for DefaultClient<S, PS> {
    type PortalStore = store::MemPortalStore<S, PS>;

    fn portal_store(&self) -> &Self::PortalStore {
        &self.portal_store
    }
}

pub trait MakeHandler {
    type Handler;

    fn make(&self) -> Self::Handler;
}

#[derive(new)]
pub struct StatelessMakeHandler<H>(Arc<H>);

impl<H> MakeHandler for StatelessMakeHandler<H> {
    type Handler = Arc<H>;

    fn make(&self) -> Self::Handler {
        self.0.clone()
    }
}
