use crate::driver::EventPumpImpl;

pub struct RunLoop {
    pump: EventPumpImpl,
}

impl RunLoop {
    pub fn new() -> Self {
        RunLoop {
            pump: EventPumpImpl::new(),
        }
    }
}

pub trait RunLoopHandler {
    fn on_init(&mut self) {}
    fn on_quit(&mut self) {}
}
