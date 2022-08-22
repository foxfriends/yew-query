//! Provides a [`QueryClient`][] to the application.
//!
//! The [`QueryClientProvider`][] is expected to be used at the top level
//! of the application. Any component that wishes to use Yew Query to manage
//! queries (e.g. via the [`use_query`][crate::hooks::use_query::use_query] hook) must
//! be a descendant of the [`QueryClientProvider`][].
//!
//! This module additionally provides hooks that allow direct access to the
//! [`QueryClient`][] for your own abstractions.
use crate::query_client::QueryClient;
use std::rc::Rc;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
struct Hidden(Rc<QueryClient>);

/// Properties for [`QueryClientProvider`][]
#[derive(PartialEq, Properties)]
pub struct Props {
    /// The actual [`QueryClient`][].
    ///
    /// The `QueryClient` is likely to be kept alive for the lifetime of the providing
    /// component by using [`use_ref`][yew::functional::use_ref], so it expects to be
    /// passed in an [`Rc`][].
    pub client: Rc<QueryClient>,
    /// Children to which the [`QueryClient`][] is being provided.
    #[prop_or_default]
    pub children: Children,
}

/// Provides a [`QueryClient`][] to its children.
///
/// It is expected that this component is used somewhere near the top of
/// your application to provide the query client to all components that
/// need it.
#[function_component(QueryClientProvider)]
pub fn query_client_provider(props: &Props) -> Html {
    html! {
        <ContextProvider<Hidden> context={Hidden(props.client.clone())}>
            {for props.children.iter()}
        </ContextProvider<Hidden>>
    }
}

/// Gets the [`QueryClient`][] from the context.
///
/// # Panics
///
/// This function will panic if no [`QueryClient`][] is provided.
/// See [`use_opt_query_client`][] for a non-panicking variant.
pub fn use_query_client() -> QueryClient {
    (*use_context::<Hidden>().unwrap().0).clone()
}

/// Gets the [`QueryClient`][] from the context, if available.
///
/// Will be [`None`][] if no [`QueryClient`][] was provided.
pub fn use_opt_query_client() -> Option<QueryClient> {
    use_context::<Hidden>().map(|hidden| (*hidden.0).clone())
}
