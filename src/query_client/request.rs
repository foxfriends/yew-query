use futures::future::Shared;
use futures::FutureExt;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;

#[derive(Debug)]
pub(crate) struct Request<T>(Shared<Pin<Box<dyn Future<Output = Rc<T>>>>>);

impl<T> Clone for Request<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Request<T> {
    pub fn new(future: impl Future<Output = Rc<T>> + 'static) -> Self {
        Self((Box::pin(future) as Pin<Box<dyn Future<Output = Rc<T>>>>).shared())
    }
}

impl<T> Future for Request<T> {
    type Output = Rc<T>;

    fn poll(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        Pin::new(&mut self.0).poll(cx)
    }
}
