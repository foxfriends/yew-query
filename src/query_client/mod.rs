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
    /// If the same query was previously being fetched, a second query will not be
    /// triggered. Instead, the result of the already-in-progress query will be
    /// returned when it is completed.
    ///
    /// If this query was previously in the cache, its data will be marked as
    /// invalid but remain accessible until the new query is complete.
    ///
    /// If you wish to remove previously cached data before fetching, see
    /// [`clear_query`][QueryClient::clear_query].
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
    ///
    /// If the query has never been fetched, this will return `None`. This is a different
    /// result than requesting a query that is mid-fetch or invalid, as such queries
    /// will return a result that indicates the state of the query at the time of requesting.
    ///
    /// The returned data is a snapshot of the state of the query, and will not update
    /// automatically when the query is completed. For that, see [`fetch_query`][QueryClient::fetch_query]
    pub fn get_query_data<Q: Query + 'static>(&self, query: &Q) -> Option<Cached<Q>> {
        let client = self.0.borrow();
        let cache = client.cache();
        cache.get(query)
    }

    /// Invalidate cached query data, without refetching.
    ///
    /// The invalid data will continue to be accessible, and may be refetched
    /// manually at a later time.
    ///
    /// If you also want to refetch the data, use [`fetch_query`][QueryClient::fetch_query].
    /// If you want to completely remove the data and query, see [`clear_query`][QueryClient::clear_query].
    pub fn invalidate_query<Q: Query + 'static>(&self, query: &Q) {
        let mut client = self.0.borrow_mut();
        let cache = client.cache_mut();
        cache.invalidate(query)
    }

    /// Completely remove a query and its associated data from the cache.
    ///
    /// After this, calling [`get_query_data`][QueryClient::get_query_data] will
    /// return `None` (as if this query had never been made).
    pub fn remove_query<Q: Query + 'static>(&self, query: &Q) {
        let mut client = self.0.borrow_mut();
        let cache = client.cache_mut();
        cache.remove(query)
    }

    /// Removed cached query data without triggering a refresh, but leaving the
    /// empty entry in the cache.
    pub fn clear_query<Q: Query + 'static>(&self, query: &Q) {
        let mut client = self.0.borrow_mut();
        let cache = client.cache_mut();
        if let Some(state) = cache.get_mut(query) {
            state.clear();
        }
    }
}

impl PartialEq for QueryClient {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}
