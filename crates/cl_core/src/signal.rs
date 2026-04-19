//! Type-safe signal / slot system modelled after Chainlink signals.
//!
//! A `Signal<A>` is an observable event source.  Connect any number of
//! closures; they are all called (in connection order) when the signal is
//! emitted.
//!
//! # Example
//! ```rust
//! use cl_core::Signal;
//!
//! let health_changed: Signal<i32> = Signal::new();
//! health_changed.connect(|hp| println!("HP is now {hp}"));
//! health_changed.emit(&42);  // prints "HP is now 42"
//! ```

use std::cell::RefCell;
use std::rc::Rc;

type Listener<A> = Box<dyn Fn(&A)>;

/// A multicast observable event.
///
/// `A` is the argument type passed to every connected handler.
/// Use `()` for a signal that carries no data.
pub struct Signal<A> {
    listeners: Rc<RefCell<Vec<Listener<A>>>>,
}

impl<A> Signal<A> {
    pub fn new() -> Self {
        Self { listeners: Rc::new(RefCell::new(Vec::new())) }
    }

    /// Add a listener closure.  The closure must be `'static` because
    /// signals may outlive the caller's stack frame.
    pub fn connect<F: Fn(&A) + 'static>(&self, f: F) {
        self.listeners.borrow_mut().push(Box::new(f));
    }

    /// Fire the signal, calling every connected listener in order.
    pub fn emit(&self, args: &A) {
        for f in self.listeners.borrow().iter() {
            f(args);
        }
    }

    /// Remove all listeners.
    pub fn disconnect_all(&self) {
        self.listeners.borrow_mut().clear();
    }

    pub fn listener_count(&self) -> usize {
        self.listeners.borrow().len()
    }
}

impl<A> Default for Signal<A> {
    fn default() -> Self { Self::new() }
}

impl<A> Clone for Signal<A> {
    /// Cloning a signal shares the same underlying listener list.
    fn clone(&self) -> Self {
        Self { listeners: Rc::clone(&self.listeners) }
    }
}

impl<A> std::fmt::Debug for Signal<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Signal({} listeners)", self.listener_count())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    #[test]
    fn signal_emit_calls_listener() {
        let sig: Signal<i32> = Signal::new();
        let received = Rc::new(Cell::new(0_i32));
        let r = Rc::clone(&received);
        sig.connect(move |&v| r.set(v));
        sig.emit(&42);
        assert_eq!(received.get(), 42);
    }

    #[test]
    fn signal_multiple_listeners_all_called() {
        let sig: Signal<()> = Signal::new();
        let count = Rc::new(Cell::new(0_u32));
        for _ in 0..3 {
            let c = Rc::clone(&count);
            sig.connect(move |_| c.set(c.get() + 1));
        }
        sig.emit(&());
        assert_eq!(count.get(), 3);
    }

    #[test]
    fn signal_listener_count() {
        let sig: Signal<()> = Signal::new();
        assert_eq!(sig.listener_count(), 0);
        sig.connect(|_| {});
        sig.connect(|_| {});
        assert_eq!(sig.listener_count(), 2);
    }

    #[test]
    fn signal_disconnect_all_removes_listeners() {
        let sig: Signal<()> = Signal::new();
        sig.connect(|_| {});
        sig.disconnect_all();
        assert_eq!(sig.listener_count(), 0);
    }

    #[test]
    fn signal_clone_shares_listener_list() {
        let sig: Signal<i32> = Signal::new();
        let clone = sig.clone();
        let received = Rc::new(Cell::new(0_i32));
        let r = Rc::clone(&received);
        // connect on clone, emit on original
        clone.connect(move |&v| r.set(v));
        sig.emit(&99);
        assert_eq!(received.get(), 99);
    }

    #[test]
    fn signal_emit_passes_value_by_ref() {
        let sig: Signal<String> = Signal::new();
        let out = Rc::new(std::cell::RefCell::new(String::new()));
        let o = Rc::clone(&out);
        sig.connect(move |s| *o.borrow_mut() = s.clone());
        sig.emit(&"hello".to_string());
        assert_eq!(*out.borrow(), "hello");
    }
}
