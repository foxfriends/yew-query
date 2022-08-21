use crate::query::Query;
use std::borrow::Borrow;
use std::hash::Hash;
use std::rc::Rc;
use type_map::TypeMap;

mod bucket;
mod cached;
mod entry;
mod state;

use bucket::Bucket;
pub use cached::Cached;
use entry::Entry;
use state::State;

#[derive(Debug)]
pub(crate) struct Cache {
    cache: TypeMap,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            cache: TypeMap::default(),
        }
    }

    pub fn insert<Q>(&mut self, query: impl Into<Rc<Q>>, data: impl Into<Rc<Q::Output>>)
    where
        Q: Query + 'static,
    {
        let bucket = self
            .cache
            .entry::<Bucket<Q>>()
            .or_insert_with(Default::default);
        bucket.values.insert(query.into(), State::valid(data));
    }

    pub fn remove<Q>(&mut self, query: &Q)
    where
        Q: Query + 'static,
    {
        if let Some(bucket) = self.cache.get_mut::<Bucket<Q>>() {
            bucket.values.remove(query);
        }
    }

    pub fn invalidate<Q>(&mut self, query: &Q)
    where
        Q: Query + 'static,
    {
        if let Some(bucket) = self.cache.get_mut::<Bucket<Q>>() {
            if let Some(state) = bucket.values.get_mut(query) {
                state.set_invalid();
            }
        }
    }

    pub fn get<Q, K>(&self, key: &K) -> Option<Cached<Q>>
    where
        Q: Query + 'static,
        Rc<Q>: Borrow<K>,
        K: Hash + Eq,
    {
        let bucket = self.cache.get::<Bucket<Q>>()?;
        let (query, data) = bucket.values.get_key_value(key)?;
        Some(Cached {
            query: query.clone(),
            data: data.clone(),
        })
    }

    pub fn entry<Q>(&mut self, query: impl Into<Rc<Q>>) -> Entry<'_, Q>
    where
        Q: Query + 'static,
    {
        let query = query.into();
        let bucket = self
            .cache
            .entry::<Bucket<Q>>()
            .or_insert_with(Default::default);
        Entry::from(bucket.values.entry(query))
    }
}
