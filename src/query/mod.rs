//! Basic query functionality.
//!
//! In Yew Query, queries are defined as types which implement the [`Query`][] trait.
//! Such queries can be passed to the [`use_query`][crate::hooks::use_query::use_query]
//! hook, which will manage the lifecycle of the data associated with that query
//! automatically, or fetched manually using the
//! [`QueryClient`][] directly.
//!
//! A query must implement [`Eq`][] (and [`Hash`][]) as the query data is used
//! as the key under which the data is cached in the [`QueryClient`][]. When two queries
//! are equal according to the [`Eq`][] implementation, the [`QueryClient`][] will
//! assume that they are intended to fetch the same data, and deduplicate requests
//! accordingly.
//!
//! [`QueryClient`]: crate::query_client::QueryClient

use std::future::Future;
use std::hash::Hash;

/// Indicates a type that represents a query to be made.
pub trait Query: Hash + Eq + PartialEq {
    /// The result of performing this query.
    ///
    /// Yew Query makes no assumptions about error handling. If your query may
    /// fail, this type should likely be a [`Result`][].
    type Output;
    /// Future type for this query.
    type Future: Future<Output = Self::Output>;

    /// Perform the query.
    ///
    /// Queries may be performed multiple times from the same query object. The
    /// query client will determine when query data is invalid and refetch accordingly.
    fn query(&self) -> Self::Future;
}
