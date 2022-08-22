//! Yew hook to fetch queries.
//!
//! The [`use_query`][] hook (or equivalently, [`use_query_with_options`][]) will
//! likely be the main way you interact with Yew Query.
//!
//! Provided a [`Query`][], `use_query` will fetch the query in the background,
//! returning a [`QueryResult`][] that represents the state of that query as it changes.
//! When the query changes, the old data is invalidated and the new data is fetched
//! automatically (putting the [`QueryResult`][] back into the loading state).
//!
//! ```no_run
//! use std::future::Future;
//! use std::pin::Pin;
//! use yew::prelude::*;
//! use yew_query::prelude::*;
//!
//! struct Todo;
//!
//! #[derive(Eq, PartialEq, Hash)]
//! struct GetTodos;
//!
//! impl Query for GetTodos {
//!     type Output = Vec<Todo>;
//!     type Future = Pin<Box<dyn Future<Output = Self::Output>>>;
//!
//!     fn query(&self) -> Self::Future {
//!         Box::pin(async move {
//!             todo!("...Making some API calls here...")
//!         })
//!     }
//! }
//!
//! #[function_component(TodoList)]
//! fn todo_list() -> Html {
//!     let todos = use_query(GetTodos);
//!     html! {
//!         <div>
//!             if let Some(data) = todos.data {
//!                 {format!("There are {} todos" , data.len())}
//!             } else {
//!                 {"Todos are still loading"}
//!             }
//!         </div>
//!     }
//! }
//! ```
//!
/// # Notes
///
/// [`use_query`][] relies on a [`QueryClient`][crate::query_client::QueryClient] being provided via the
/// [`QueryClientProvider`][crate::components::query_client_provider::QueryClientProvider]. If
/// none is provided, the query will never be made, and you will be left with just
/// an empty [`QueryResult`][].

use crate::components::query_client_provider::use_opt_query_client;
use crate::query::Query;

/// Options for customizing the behaviour of the query lifecycle.
///
/// This gets merged with the options set at the [`QueryClient`][crate::query_client::QueryClient] level.
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
/// See the [module-level documentation][self] for more information.
pub fn use_query_with_options<Q>(_query: Q, _options: Options) -> QueryResult<Q::Output>
where
    Q: Query,
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
/// See the [module-level documentation][self] for more information.
pub fn use_query<Q>(query: Q) -> QueryResult<Q::Output>
where
    Q: Query,
{
    use_query_with_options(query, Options::default())
}
