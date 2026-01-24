use runtil::{Context, RunLoop, RunLoopHandler, UserMessage};

struct Message();
impl UserMessage for Message {}
struct Handler();
impl RunLoopHandler<Message> for Handler {
    fn init(&mut self, cx: &Context) {
        let window = cx.create_window();
        window.show();
        // TODO
        // let wm = cx.window_manager();
        // let window = wm.create(marker);
        // window.show();
        cx.dispatch_main(|_marker| {
            println!("hello, world!");
        });
    }
}

fn main() {
    let mut runloop = RunLoop::new(Handler());
    runloop.run();
}
