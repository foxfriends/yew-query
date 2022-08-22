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
//! #[derive(Clone, Eq, PartialEq, Hash)]
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
//!             if let Some(data) = todos.data() {
//!                 {format!("There are {} things to do", data.len())}
//!             } else {
//!                 {"Loading..."}
//!             }
//!         </div>
//!     }
//! }
//! ```
//!
use std::ops::Deref;

use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

/// # Notes
///
/// [`use_query`][] relies on a [`QueryClient`][crate::query_client::QueryClient] being provided via the
/// [`QueryClientProvider`][crate::components::query_client_provider::QueryClientProvider]. If
/// none is provided, the query will never be made, and you will be left with just
/// an empty [`QueryResult`][].
use crate::components::query_client_provider::use_opt_query_client;
use crate::prelude::QueryClient;
use crate::query::Query;
use crate::query_client::Cached;

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
#[derive(Clone)]
pub struct QueryResult<Q>
where
    Q: Query,
{
    data: Option<Cached<Q>>,
}

impl<Q> QueryResult<Q>
where
    Q: Query,
{
    fn new(cached: Cached<Q>) -> Self {
        Self { data: Some(cached) }
    }

    pub fn data(&self) -> Option<&Q::Output> {
        self.data.as_ref().and_then(|cached| cached.data())
    }
}

impl<Q> Default for QueryResult<Q>
where
    Q: Query,
{
    fn default() -> Self {
        Self { data: None }
    }
}

/// Makes a query.
///
/// See the [module-level documentation][self] for more information.
pub fn use_query_with_options<Q>(query: Q, options: Options) -> impl Deref<Target = QueryResult<Q>>
where
    Q: Query + Clone + 'static,
{
    let client = use_opt_query_client();
    let query_result = use_state({
        let client = client.clone();
        let query = &query;
        move || {
            client
                .and_then(|client| client.get_query_data(query))
                .map(QueryResult::new)
                .unwrap_or_default()
        }
    });

    use_effect_with_deps(
        {
            let query_result = query_result.clone();
            move |(client, query, options): &(Option<QueryClient>, Q, Options)| {
                if options.enabled {
                    if let Some(client) = client.clone() {
                        let query = query.clone();
                        spawn_local(async move {
                            let cached = client.fetch_query(query).await;
                            query_result.set(QueryResult::new(cached));
                        });
                    }
                }
                || ()
            }
        },
        (client, query, options),
    );

    query_result
}

/// Makes a query using the default options.
///
/// See the [module-level documentation][self] for more information.
pub fn use_query<Q>(query: Q) -> impl Deref<Target = QueryResult<Q>>
where
    Q: Query + Clone + 'static,
{
    use_query_with_options(query, Options::default())
}
