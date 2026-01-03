use crossbeam::queue::SegQueue;

use crate::task::Task;

pub struct UIThreadRunner {
    tasks: SegQueue<Task>,
}

impl UIThreadRunner {
    pub fn new() -> Self {
        UIThreadRunner {
            tasks: SegQueue::new(),
        }
    }
}
