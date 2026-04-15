//! Type-safe signal / slot system modelled after Chainlink signals.
//!
//! A `Signal<A>` is an observable event source.  Connect any number of
//! closures; they are all called (in connection order) when the signal is
//! emitted.
//!
//! # Example
//! ```rust
//! use chainlink_core::Signal;
//!
//! let health_changed: Signal<i32> = Signal::new();
//! health_changed.connect(|hp| println!("HP is now {hp}"));
//! health_changed.emit(42);  // prints "HP is now 42"
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
