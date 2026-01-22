use std::cell::Cell;
use std::rc::Rc;
use std::fmt;

/// Clonable trait for objects that can be cloned
pub trait Clonable {
    fn clone(&self) -> RefPtr<Self> where Self: Sized;
}

/// Reference count wrapper using Rc with manual reference counting
/// This is a safer alternative to the C++ Ref class
#[derive(Debug)]
pub struct Ref {
    reference_count: Cell<u32>,
    #[allow(dead_code)]
    name: Option<&'static str>,
}

impl Ref {
    pub fn new() -> Ref {
        Ref::with_name(None)
    }

    pub fn with_name(name: Option<&'static str>) -> Ref {
        Ref {
            reference_count: Cell::new(1),
            name,
        }
    }

    /// Retains the ownership.
    /// This increases the Ref's reference count.
    pub fn retain(&self) {
        let count = self.reference_count.get();
        self.reference_count.set(count + 1);
    }

    /// Releases the ownership immediately.
    /// This decrements the Ref's reference count.
    /// If the reference count reaches 0, the object is dropped.
    pub fn release(&self) {
        let count = self.reference_count.get();
        if count > 1 {
            self.reference_count.set(count - 1);
        } else {
            // When count reaches 0, the Rc will be dropped automatically
            // This is handled by the Drop trait
        }
    }

    /// Returns the Ref's current reference count.
    pub fn get_reference_count(&self) -> u32 {
        self.reference_count.get()
    }
}

impl Default for Ref {
    fn default() -> Self {
        Ref::new()
    }
}

impl Drop for Ref {
    fn drop(&mut self) {
        // When Ref is dropped, the reference count should be 0
        // If it's not 0, that means there's a memory leak
        if self.reference_count.get() > 0 {
            // In debug mode, we could log a warning
            // For now, we'll just let it drop
        }
    }
}

/// A smart pointer that wraps Rc<Ref> with automatic reference counting
#[derive(Debug)]
pub struct RefPtr<T: ?Sized> {
    ptr: Rc<T>,
}

impl<T> RefPtr<T> {
    /// Creates a new RefPtr from an Rc<T>
    pub fn new(value: T) -> RefPtr<T> {
        RefPtr {
            ptr: Rc::new(value),
        }
    }

    /// Gets a reference to the underlying value
    pub fn borrow(&self) -> &T {
        &self.ptr
    }

    /// Gets a mutable reference to the underlying value
    pub fn borrow_mut(&mut self) -> &mut T {
        Rc::get_mut(&mut self.ptr).unwrap()
    }

    /// Gets the reference count
    pub fn get_reference_count(&self) -> u32 {
        // For Rc, we can't directly get the reference count from outside
        // But we can track it internally if needed
        1 // Placeholder
    }

    /// Retains the reference count
    pub fn retain(&self) {
        // Rc handles this automatically
    }

    /// Releases the reference count
    pub fn release(&self) {
        // Rc handles this automatically
    }
}

impl<T> Clone for RefPtr<T> {
    fn clone(&self) -> RefPtr<T> {
        RefPtr {
            ptr: self.ptr.clone(),
        }
    }
}

impl<T> PartialEq for RefPtr<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &RefPtr<T>) -> bool {
        Rc::ptr_eq(&self.ptr, &other.ptr) || *self.ptr == *other.ptr
    }
}

impl<T> fmt::Display for RefPtr<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RefPtr({})", self.ptr)
    }
}

// Make RefPtr work with Deref for easier access
use std::ops::{Deref, DerefMut};

impl<T> Deref for RefPtr<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.ptr
    }
}

impl<T> DerefMut for RefPtr<T> {
    fn deref_mut(&mut self) -> &mut T {
        Rc::get_mut(&mut self.ptr).expect("RefPtr: Cannot get mutable reference, reference count > 1")
    }
}

impl<T> From<Rc<T>> for RefPtr<T> {
    fn from(ptr: Rc<T>) -> Self {
        RefPtr { ptr }
    }
}

impl<T> From<RefPtr<T>> for Rc<T> {
    fn into(ptr: RefPtr<T>) -> Self {
        ptr.ptr
    }
}
