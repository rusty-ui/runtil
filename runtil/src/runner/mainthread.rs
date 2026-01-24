use crate::{driver::EventPumpImpl, task::MainTask, window::Window};

pub(crate) struct MainThreadRunner {
    pump: EventPumpImpl,
}

impl MainThreadRunner {
    pub fn new() -> Self {
        MainThreadRunner {
            pump: EventPumpImpl::new(),
        }
    }

    pub fn schedule_task(&self, task: MainTask) {
        self.pump.set_task_and_schedule(task);
    }

    pub fn create_window(&self) -> Window {
        self.pump.create_window()
    }

    pub fn run(&self) {
        self.pump.run();
    }

    pub fn quit(&self) {
        self.pump.quit();
    }
}
