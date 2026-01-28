use std::marker::PhantomData;

use crate::runloop::UserMessage;

pub trait RunLoopService<M: UserMessage> {
    fn on_start(&mut self, _id: usize) {}
    fn before_exit(&mut self) {}
}

pub struct Service<M: UserMessage, S: RunLoopService<M>> {
    inner: S,
    phantom: PhantomData<M>,
}
