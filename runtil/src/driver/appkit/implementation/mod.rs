use super::binding::runtilappkit_init;

pub struct AppkitEventPump {}

impl AppkitEventPump {
    pub fn new() -> Self {
        // SAFETY:
        unsafe { runtilappkit_init() };

        AppkitEventPump {}
    }
}
