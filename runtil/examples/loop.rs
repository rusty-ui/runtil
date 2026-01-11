use runtil::{Context, RunLoop, RunLoopHandler, UserMessage};

struct Message();
impl UserMessage for Message {}
struct Handler();
impl RunLoopHandler<Message> for Handler {
    fn init(&mut self, cx: &Context) {
        cx.dispatch_main(|_marker| {
            println!("hello world");
        });
    }
}

fn main() {
    let mut runloop = RunLoop::new(Handler());
    runloop.run();
}
