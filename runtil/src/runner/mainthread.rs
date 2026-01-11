use crossbeam::queue::SegQueue;

use crate::{driver::EventPumpImpl, task::MainTask};

pub(crate) struct MainThreadRunner {
    tasks: SegQueue<MainTask>,
    pump: EventPumpImpl,
}

impl MainThreadRunner {
    pub fn new() -> Self {
        MainThreadRunner {
            pump: EventPumpImpl::new(),
            tasks: SegQueue::new(),
        }
    }

    pub(crate) fn schedule_task(&self, task: MainTask) {
        self.pump.set_task_and_schedule(task);
    }
}
