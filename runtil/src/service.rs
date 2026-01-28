use crate::runloop::UserMessage;

pub trait Service<M: UserMessage> {
    fn on_start(&mut self) {}
}
