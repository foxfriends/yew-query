use crate::query::Query;
use futures::FutureExt;
use std::cell::RefCell;
use std::rc::Rc;

mod cache;
mod client;
mod request;

use cache::Cache;
use client::Client;
use request::Request;

pub use cache::Cached;

/// Provides a backing for the query hooks. Must be provided to
/// the app via [`QueryClientProvider`][crate::components::query_client_provider::QueryClientProvider].
#[derive(Clone, Debug, Default)]
pub struct QueryClient(Rc<RefCell<Client>>);

impl QueryClient {
    /// Create a new (default) `QueryClient`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Manually set the output data for a particular query.
    ///
    /// This is particularly useful when you already have the data as a result of
    /// doing something else. If you do not yet have the data, you will be better
    /// served by [`fetch_query`][QueryClient::fetch_query]
    pub fn set_query_data<Q: Query + 'static>(&self, query: Q, data: Q::Output) {
        let mut client = self.0.borrow_mut();
        client.cache_mut().insert(query, data);
    }

    /// Fetches a query and stores its data in the cache.
    ///
    /// The returned future will complete when the fetching is done.
    ///
    /// If this query was previously in the cache, it will be completely removed before
    /// querying. If you wish to preserve previously cached data see [`refetch_query`][].
    pub async fn fetch_query<Q: Query + 'static>(&self, query: Q) -> Cached<Q> {
        let query = Rc::new(query);
        let client = self.0.clone();
        let request = Request::new(query.query().map({
            let query = query.clone();
            let client = client.clone();
            move |data| {
                let data = Rc::new(data);
                client
                    .borrow_mut()
                    .cache_mut()
                    .insert::<Q>(query, data.clone());
                data
            }
        }));

        let mut client_mut = client.borrow_mut();
        let cache = client_mut.cache_mut();
        let state = cache.entry::<Q>(query.clone()).or_default();
        if state.is_loading() {
            let data = state.pending_data().unwrap();
            std::mem::drop(client_mut);
            data
        } else {
            state.set_loading(request.clone());
            std::mem::drop(client_mut);
            request
        }
        .await;

        let client = client.borrow();
        client.cache().get(query.as_ref()).unwrap()
    }

    /// Retrieves cached query data.
    pub fn get_query_data<Q: Query + 'static>(&self, query: &Q) -> Option<Cached<Q>> {
        let client = self.0.borrow();
        let cache = client.cache();
        cache.get(query)
    }

    /// Invalidate cached query data, without refetching.
    ///
    /// The invalid data will continue to be accessible.
    ///
    /// If you also want to refetch the data, see [`refetch_query`][].
    pub fn invalidate_query<Q: Query + 'static>(&self, query: &Q) {
        let mut client = self.0.borrow_mut();
        let cache = client.cache_mut();
        cache.invalidate(query)
    }

    /// Invalidate and refetch cached query data.
    ///
    /// The invalid data will continue to be accessible until the refetch is
    /// completed.
    ///
    /// If you want to invalidate the query without refetching, see `invalidate_query`.
    pub fn refetch_query<Q: Query + 'static>(&self, query: &Q) {
        let mut client = self.0.borrow_mut();
        let cache = client.cache_mut();
        cache.invalidate(query);
        // let _query = cache.entry(query);
    }

    /// Completely remove cached query data without triggering a refresh.
    pub fn clear_query<Q: Query + 'static>(&self, query: &Q) {
        let mut client = self.0.borrow_mut();
        let cache = client.cache_mut();
        cache.remove(query)
    }
}

impl PartialEq for QueryClient {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}
