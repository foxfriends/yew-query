use std::any::Any;
use std::cmp::PartialEq;
use std::future::Future;
use std::hash::Hash;

/// Describes a query that can be made.
pub trait Query: Hash + Eq + PartialEq {
    /// The result of performing this query.
    type Output: Any;
    /// The future.
    type Future: Future<Output = Self::Output>;

    /// Perform the query.
    ///
    /// Queries may be performed multiple times.
    fn query(&self) -> Self::Future;
}
