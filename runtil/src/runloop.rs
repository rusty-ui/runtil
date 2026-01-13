use std::{
    sync::{Arc, OnceLock},
    thread::{self, ThreadId},
};

use crate::{actor::MainMarker, runner::mainthread::MainThreadRunner, task::MainTask};

pub(crate) static MAIN_THREAD_ID: OnceLock<ThreadId> = OnceLock::new();

pub struct Context {
    main_runner: Arc<MainThreadRunner>,
}

impl Context {
    pub(crate) fn new(main_runner: Arc<MainThreadRunner>) -> Self {
        Context { main_runner }
    }

    pub fn spawn(&self, _fut: impl Future<Output = ()>) {
        unimplemented!();
    }

    pub fn dispatch_main(&self, f: impl Fn(MainMarker) -> () + 'static) {
        let task = MainTask { f: Box::new(f) };
        self.main_runner.schedule_task(task);
    }
}

pub trait UserMessage {}

#[allow(async_fn_in_trait)]
pub trait RunLoopHandler<M: UserMessage>
where
    Self: Send + Sync,
{
    fn init(&mut self, _cx: &Context) {}
    async fn handle_event(&mut self, _cx: &mut Context) {}
    fn quit(&mut self, _cx: &Context) {}
}

pub struct RunLoop<M, H>
where
    M: UserMessage,
    H: RunLoopHandler<M>,
{
    main_runner: Arc<MainThreadRunner>,
    // worker_runner: ParallelRunner,
    handler: H,
    phantom: std::marker::PhantomData<M>,
    initialized: bool,
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
            main_runner: Arc::new(MainThreadRunner::new()),
            handler,
            phantom: std::marker::PhantomData,
            initialized: true,
        }
    }

    fn create_context(&self) -> Context {
        Context::new(self.main_runner.clone())
    }

    pub fn run(&mut self) {
        let context = self.create_context();
        self.handler.init(&context);
        self.main_runner.run();
    }

    pub fn quit(&mut self) {
        self.main_runner.quit();
        self.initialized = false;
    }
}

impl<M, H> Drop for RunLoop<M, H>
where
    M: UserMessage,
    H: RunLoopHandler<M>,
{
    fn drop(&mut self) {
        if self.initialized {
            self.quit();
        }
    }
}
