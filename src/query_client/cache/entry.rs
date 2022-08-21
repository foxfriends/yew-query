use super::State;
use crate::query::Query;
use std::collections::hash_map::{
    Entry as MapEntry, OccupiedEntry as OccupiedMapEntry, VacantEntry as VacantMapEntry,
};
use std::rc::Rc;

pub(crate) enum Entry<'cache, Q>
where
    Q: Query,
{
    Occupied(OccupiedEntry<'cache, Q>),
    Vacant(VacantEntry<'cache, Q>),
}

pub(crate) struct OccupiedEntry<'cache, Q>(OccupiedMapEntry<'cache, Rc<Q>, State<Q::Output>>)
where
    Q: Query;

pub(crate) struct VacantEntry<'cache, Q>(VacantMapEntry<'cache, Rc<Q>, State<Q::Output>>)
where
    Q: Query;

impl<'cache, Q> Entry<'cache, Q>
where
    Q: Query,
{
    pub fn or_default(self) -> &'cache mut State<Q::Output> {
        match self {
            Entry::Occupied(entry) => entry.0.into_mut(),
            Entry::Vacant(entry) => entry.0.insert(Default::default()),
        }
    }
}

impl<'cache, Q> From<MapEntry<'cache, Rc<Q>, State<Q::Output>>> for Entry<'cache, Q>
where
    Q: Query,
{
    fn from(entry: MapEntry<'cache, Rc<Q>, State<Q::Output>>) -> Self {
        match entry {
            MapEntry::Occupied(entry) => Self::Occupied(OccupiedEntry(entry)),
            MapEntry::Vacant(entry) => Self::Vacant(VacantEntry(entry)),
        }
    }
}
