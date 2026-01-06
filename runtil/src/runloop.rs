use std::{
    sync::OnceLock,
    thread::{self, ThreadId},
};

use crate::driver::EventPumpImpl;

pub(crate) static MAIN_THREAD_ID: OnceLock<ThreadId> = OnceLock::new();

pub struct Context {}

impl Context {
    pub fn new() -> Self {
        Context {}
    }

    pub fn spawn(&mut self, fut: impl Future<Output = ()>) {}

    pub fn dispatch_main(&mut self, fut: impl Future<Output = ()>) {}
}

pub trait UserMessage {}

pub trait RunLoopHandler<M: UserMessage>
where
    Self: Send + Sync,
{
    fn init(&mut self, _cx: &mut Context) {}
    /*fn handle_event(&mut self, _cx: &mut Context) -> impl Future<Output = ()> + Send {
        async {}
    }*/
    fn quit(&mut self, _cx: &mut Context) {}
}

pub struct RunLoop<M, H>
where
    M: UserMessage,
    H: RunLoopHandler<M>,
{
    // main_runner: MainThreadRunner,
    // worker_runner: ParallelRunner,
    pump: EventPumpImpl,
    handler: H,
    phantom: std::marker::PhantomData<M>,
}

impl<M, H> RunLoop<M, H>
where
    M: UserMessage,
    H: RunLoopHandler<M>,
{
    pub fn new(handler: H) -> Self {
        let thread_id = thread::current().id();
        let _ = MAIN_THREAD_ID.set(thread_id);

        RunLoop {
            pump: EventPumpImpl::new(),
            handler,
            phantom: std::marker::PhantomData,
        }
    }

    pub fn run(&mut self) {}
}
