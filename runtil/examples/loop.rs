use runtil::{RunLoop, RunLoopHandler, UserMessage};

struct Message();
impl UserMessage for Message {}
struct Handler();
impl RunLoopHandler<Message> for Handler {}

fn main() {
    let _runloop = RunLoop::new(Handler());
}
