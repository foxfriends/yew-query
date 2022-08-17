use crate::query_client::QueryClient;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
struct Hidden(QueryClient);

#[derive(PartialEq, Properties)]
pub struct Props {
    pub client: QueryClient,
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
    use_context::<Hidden>().unwrap().0
}

/// Gets the [`QueryClient`][] from the context, if available.
///
/// Will be [`None`][] if no [`QueryClient`][] was provided.
pub fn use_opt_query_client() -> Option<QueryClient> {
    use_context::<Hidden>().map(|hidden| hidden.0)
}
