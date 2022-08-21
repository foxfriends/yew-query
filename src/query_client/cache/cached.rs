use super::State;
use crate::query::Query;
use std::rc::Rc;

#[derive(Debug)]
pub struct Cached<Q>
where
    Q: Query,
{
    pub(super) query: Rc<Q>,
    pub(super) data: State<Q::Output>,
}

impl<Q> Clone for Cached<Q>
where
    Q: Query,
{
    fn clone(&self) -> Self {
        Self {
            query: self.query.clone(),
            data: self.data.clone(),
        }
    }
}

impl<Q> Cached<Q>
where
    Q: Query,
{
    pub fn query(&self) -> &Q {
        self.query.as_ref()
    }

    pub fn data(&self) -> Option<&Q::Output> {
        self.data.data()
    }

    pub fn is_loading(&self) -> bool {
        self.data.is_loading()
    }

    pub fn is_valid(&self) -> bool {
        self.data.is_valid()
    }

    pub fn is_idle(&self) -> bool {
        self.data.is_idle()
    }
}
