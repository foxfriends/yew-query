use super::State;
use crate::query::Query;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
pub(super) struct Bucket<Q>
where
    Q: Query,
{
    pub values: HashMap<Rc<Q>, State<Q::Output>>,
}

impl<Q> Default for Bucket<Q>
where
    Q: Query,
{
    fn default() -> Self {
        Self {
            values: HashMap::with_capacity(1),
        }
    }
}
