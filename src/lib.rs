//! Asynchronous state management, for maintaining server-side state in a [Yew][] application.
//!
//! Inspired by [TanStack Query][] for Javascript applications, Yew Query provides a
//! [`QueryClient`][query_client::QueryClient] and familiar hooks like [`use_query`][hooks::use_query::use_query].
//! Not all the same features are provided, as this project is in very early stages. Expect
//! possible major changes if planning to use this (particularly before Yew itself is stable).
//!
//! Notable features you might expect but are not planned for development any time soon:
//! *   Automatic retry
//! *   Stale time/automatic refresh
//! *   Mutations
//!
//! [Yew]: https://yew.rs/
//! [TanStack Query]: https://tanstack.com/query/v4
//!
//! # Usage
//!
//! Yew Query expects that you are using [Function Components][]. No considerations have
//! been made for usage with trait-based components.
//!
//! [Function Components]: https://yew.rs/docs/concepts/function-components/introduction
//!
//! Basic usage begins with creating a [`QueryClient`][query_client::QueryClient]
//! and providing it to the application with the [`QueryClientProvider`][components::query_client_provider::QueryClientProvider].
//!
//! ```no_run
//! use yew::prelude::*;
//! use yew_query::prelude::*;
//!
//! # #[function_component(Main)] fn main() -> Html { html!{"Hello world"} }
//! #[function_component(App)]
//! fn app() -> Html {
//!     let client = use_ref(QueryClient::new);
//!
//!     html! {
//!         <QueryClientProvider client={client}>
//!             <Main />
//!         </QueryClientProvider>
//!     }
//! }
//!
//! fn main() {
//!     yew::start_app::<App>();
//! }
//! ```
//!

pub mod components;
pub mod hooks;
pub mod query;
pub mod query_client;

pub mod prelude {
    //! Includes most commonly used types.
    pub use crate::components::query_client_provider::{
        use_opt_query_client, use_query_client, QueryClientProvider,
    };
    pub use crate::hooks::use_query::{
        use_query, use_query_with_options, Options as QueryOptions, QueryResult,
    };
    pub use crate::query_client::QueryClient;
    pub use crate::query::Query;
}
