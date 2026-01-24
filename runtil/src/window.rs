use crate::driver::WindowImpl;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct WindowId(usize);

#[derive(Clone, Debug)]
pub struct Window {
    inner: WindowImpl,
}

impl Window {
    pub(crate) fn new(inner: WindowImpl) -> Self {
        Window { inner }
    }

    pub fn show(&self) {
        self.inner.show();
    }
}
