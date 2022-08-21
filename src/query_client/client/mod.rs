use super::Cache;
use std::fmt::{self, Debug};

pub(crate) struct Client {
    cache: Cache,
}

impl Default for Client {
    fn default() -> Self {
        Self {
            cache: Cache::new(),
        }
    }
}

impl Debug for Client {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Client {{ cache: _ }}")
    }
}

impl Client {
    pub fn cache_mut(&mut self) -> &mut Cache {
        &mut self.cache
    }

    pub fn cache(&self) -> &Cache {
        &self.cache
    }
}
