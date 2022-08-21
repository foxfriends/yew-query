use crate::query_client::request::Request;
use std::rc::Rc;

#[derive(Debug)]
pub(crate) enum State<T> {
    Invalid(Rc<T>),
    Valid(Rc<T>),
    Loading(Option<Rc<T>>, Request<T>),
    Idle,
}

impl<T> Default for State<T> {
    fn default() -> Self {
        Self::Idle
    }
}

impl<T> Clone for State<T> {
    fn clone(&self) -> Self {
        match self {
            Self::Invalid(data) => Self::Invalid(data.clone()),
            Self::Valid(data) => Self::Valid(data.clone()),
            Self::Loading(data, req) => Self::Loading(data.clone(), req.clone()),
            Self::Idle => Self::Idle,
        }
    }
}

impl<T> State<T> {
    pub fn valid(data: impl Into<Rc<T>>) -> Self {
        Self::Valid(data.into())
    }

    pub fn set_invalid(&mut self) {
        *self = match std::mem::take(self) {
            Self::Valid(data) => Self::Invalid(data),
            other => other,
        }
    }

    pub fn set_loading(&mut self, req: Request<T>) {
        *self = match std::mem::take(self) {
            Self::Valid(data) | Self::Invalid(data) => Self::Loading(Some(data), req),
            Self::Idle => Self::Loading(None, req),
            Self::Loading(data, _) => Self::Loading(data, req),
        }
    }

    pub fn is_loading(&self) -> bool {
        matches!(self, Self::Loading(..))
    }

    pub fn pending_data(&self) -> Option<Request<T>> {
        match self {
            Self::Loading(.., req) => Some(req.clone()),
            _ => None,
        }
    }

    pub fn data(&self) -> Option<&T> {
        match self {
            Self::Valid(data) | Self::Invalid(data) => Some(data.as_ref()),
            _ => None,
        }
    }

    pub fn is_valid(&self) -> bool {
        matches!(self, Self::Valid(..))
    }

    pub fn is_idle(&self) -> bool {
        matches!(self, Self::Idle)
    }
}
