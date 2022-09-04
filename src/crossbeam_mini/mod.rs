use std::io;
use std::thread;

mod atomic_option;
mod scoped;

pub use scoped::{scope, Scope, ScopedJoinHandle};

#[doc(hidden)]
trait FnBox {
	fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
	fn call_box(self: Box<Self>) {
		(*self)()
	}
}

/// Like `std::thread::spawn`, but without the closure bounds.
pub unsafe fn spawn_unsafe<'a, F>(f: F) -> thread::JoinHandle<()>
where
	F: FnOnce() + Send + 'a,
{
	let builder = thread::Builder::new();
	builder_spawn_unsafe(builder, f).unwrap()
}

/// Like `std::thread::Builder::spawn`, but without the closure bounds.
pub unsafe fn builder_spawn_unsafe<'a, F>(builder: thread::Builder, f: F) -> io::Result<thread::JoinHandle<()>>
where
	F: FnOnce() + Send + 'a,
{
	use std::mem;

	let closure: Box<dyn FnBox + 'a> = Box::new(f);
	let closure: Box<dyn FnBox + Send> = mem::transmute(closure);
	builder.spawn(move || closure.call_box())
}
