use std::cell::{Cell, RefCell};
use std::fmt;
use std::rc::Rc;

/// Clonable trait for objects that can be cloned
pub trait Clonable {
    fn clone(&self) -> RefPtr<Self>
    where
        Self: Sized;
}

/// Reference count wrapper using Rc with manual reference counting
/// This is a safer alternative to the C++ Ref class
#[derive(Debug, Clone)]
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

    /// Pointer equality check
    pub fn ptr_eq(a: &Ref, b: &Ref) -> bool {
        std::ptr::eq(a, b)
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

/// A smart pointer that wraps Rc<RefCell<T>> with automatic reference counting
#[derive(Debug)]
pub struct RefPtr<T: ?Sized> {
    ptr: Rc<RefCell<T>>,
}

impl<T> RefPtr<T> {
    /// Creates a new RefPtr from an Rc<RefCell<T>> or value
    pub fn new(value: T) -> RefPtr<T> {
        RefPtr {
            ptr: Rc::new(RefCell::new(value)),
        }
    }

    /// Pointer equality check
    pub fn ptr_eq(a: &RefPtr<T>, b: &RefPtr<T>) -> bool {
        Rc::ptr_eq(&a.ptr, &b.ptr)
    }

    /// Gets a reference to the underlying value
    pub fn borrow(&self) -> std::cell::Ref<T> {
        self.ptr.borrow()
    }

    /// Gets a mutable reference to the underlying value
    pub fn borrow_mut(&self) -> std::cell::RefMut<T> {
        self.ptr.borrow_mut()
    }

    /// Gets the reference count
    pub fn get_reference_count(&self) -> u32 {
        // For Rc, we can't directly get the reference count from outside
        // But we can track it internally if needed
        Rc::strong_count(&self.ptr) as u32
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
        // Check pointer equality first
        if Rc::ptr_eq(&self.ptr, &other.ptr) {
            return true;
        }
        // Then check value equality
        *self.ptr.borrow() == *other.ptr.borrow()
    }
}

impl<T> fmt::Display for RefPtr<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RefPtr({})", self.ptr.borrow())
    }
}

// Make RefPtr work with Deref for easier access?
// We CANNOT implement Deref<Target=T> because we can't return &T from RefCell.
// We can implement Deref<Target=RefCell<T>>? No, usage expects T methods.
// Users must use borrow() or borrow_mut().

/*
use std::ops::{Deref, DerefMut};

impl<T> Deref for RefPtr<T> {
    type Target = T;
    fn deref(&self) -> &T {
        // Impossible with RefCell
        unsafe { &*self.ptr.as_ptr() } // This is UNSAFE and violates aliasing rules if mutable borrow exists
    }
}
*/

// Remove From<Rc<T>> as it is not compatible with Rc<RefCell<T>> easily unless we wrap
// But we can add From<Rc<RefCell<T>>>

impl<T> From<Rc<RefCell<T>>> for RefPtr<T> {
    fn from(ptr: Rc<RefCell<T>>) -> Self {
        RefPtr { ptr }
    }
}

impl<T> From<RefPtr<T>> for Rc<RefCell<T>> {
    fn from(ptr: RefPtr<T>) -> Self {
        ptr.ptr
    }
}
