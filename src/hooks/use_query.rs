use crate::components::query_client_provider::use_opt_query_client;
use std::future::Future;

/// Options for customizing the behaviour of this query.
///
/// This gets merged with the options set at the client level.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Options {
    /// Whether this query should be attempted at all right now.
    ///
    /// Default: `true`
    pub enabled: bool,
}

impl Default for Options {
    fn default() -> Self {
        Options { enabled: true }
    }
}

/// Reflects the state of a query.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct QueryResult<T> {
    /// The retrieved data.
    ///
    /// A [`None`][] indicates that the data has not yet been queried (or has been cleared),
    /// while a some indicates that the data has been retrieved.
    pub data: Option<T>,
    /// Whether a query is currently happening.
    pub is_loading: bool,
}

impl<T> Default for QueryResult<T> {
    fn default() -> Self {
        Self {
            data: None,
            is_loading: false,
        }
    }
}

/// Makes a query.
///
/// TODO: fully document this feature.
///
/// # Notes
///
/// This relies on a [`QueryClient`][] being provided via the
/// [`QueryClientProvider`][crate::components::query_client_provider::QueryClientProvider]. If
/// none is provided, the query will never be made, and you will be left with just
/// an empty [`QueryResult`][].
pub fn use_query_with_options<K, P, F, T>(_key: K, _query: F, _options: Options) -> QueryResult<T>
where
    // K: QueryKey,
    P: Future<Output = T>,
    F: Fn(&K) -> P,
{
    match use_opt_query_client() {
        None => QueryResult::default(),
        Some(_client) => {
            // TODO: make queries
            QueryResult::default()
        }
    }
}

/// Makes a query using the default options.
///
/// See [`use_query_with_options`][] for the more information.
pub fn use_query<K, P, F, T>(key: K, query: F) -> QueryResult<T>
where
    // K: QueryKey,
    P: Future<Output = T>,
    F: Fn(&K) -> P,
{
    use_query_with_options(key, query, Options::default())
}
