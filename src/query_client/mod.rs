use std::rc::Rc;

mod client;

use client::Client;

/// Provides a backing for the query hooks. Must be provided to
/// the app via [`QueryClientProvider`][crate::components::query_client_provider::QueryClientProvider].
#[derive(Clone, Debug, Default)]
pub struct QueryClient(Rc<Client>);

impl QueryClient {
    /// Create a new (default) `QueryClient`.
    pub fn new() -> Self {
        Self::default()
    }
}

impl PartialEq for QueryClient {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}
