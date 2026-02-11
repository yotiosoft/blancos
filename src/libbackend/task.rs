extern crate alloc;
use core::{ future::Future, pin::Pin };
use alloc::boxed::Box;

/// タスク
pub struct Task {
    future: Pin<Box<dyn Future<Output = ()>>>,
}
