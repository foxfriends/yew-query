pub mod components;
pub mod hooks;
pub mod query;
pub mod query_client;

pub mod prelude {
    pub use crate::components::query_client_provider::{
        use_opt_query_client, use_query_client, QueryClientProvider,
    };
    pub use crate::hooks::use_query::{
        use_query, use_query_with_options, Options as QueryOptions, QueryResult,
    };
    pub use crate::query_client::QueryClient;
}
