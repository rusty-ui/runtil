use std::sync::Mutex;

use futures::future::BoxFuture;

pub struct Task {
    fut: Mutex<Option<BoxFuture<'static, ()>>>,
}
