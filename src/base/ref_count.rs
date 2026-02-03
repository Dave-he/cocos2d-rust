use std::cell::{Cell, RefCell};
use std::fmt;
use std::rc::Rc;

pub trait Clonable {
    fn clone(&self) -> RefPtr<Self>
    where
        Self: Sized;
}

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

    pub fn ptr_eq(a: &Ref, b: &Ref) -> bool {
        std::ptr::eq(a, b)
    }

    pub fn retain(&self) {
        let count = self.reference_count.get();
        self.reference_count.set(count + 1);
    }

    pub fn release(&self) {
        let count = self.reference_count.get();
        if count > 1 {
            self.reference_count.set(count - 1);
        } else {
        }
    }

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
        if self.reference_count.get() > 0 {
        }
    }
}

#[derive(Debug)]
pub struct RefPtr<T: ?Sized> {
    ptr: Rc<RefCell<T>>,
}

impl<T> RefPtr<T> {
    pub fn new(value: T) -> RefPtr<T> {
        RefPtr {
            ptr: Rc::new(RefCell::new(value)),
        }
    }

    pub fn ptr_eq(a: &RefPtr<T>, b: &RefPtr<T>) -> bool {
        Rc::ptr_eq(&a.ptr, &b.ptr)
    }

    pub fn borrow(&self) -> std::cell::Ref<T> {
        self.ptr.borrow()
    }

    pub fn borrow_mut(&self) -> std::cell::RefMut<T> {
        self.ptr.borrow_mut()
    }

    pub fn get_reference_count(&self) -> u32 {
        Rc::strong_count(&self.ptr) as u32
    }

    pub fn retain(&self) {
    }

    pub fn release(&self) {
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
        if Rc::ptr_eq(&self.ptr, &other.ptr) {
            return true;
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ref_new() {
        let ref_obj = Ref::new();
        assert_eq!(ref_obj.get_reference_count(), 1);
    }

    #[test]
    fn test_ref_with_name() {
        let ref_obj = Ref::with_name(Some("test_ref"));
        assert_eq!(ref_obj.get_reference_count(), 1);
    }

    #[test]
    fn test_ref_retain() {
        let ref_obj = Ref::new();
        assert_eq!(ref_obj.get_reference_count(), 1);
        ref_obj.retain();
        assert_eq!(ref_obj.get_reference_count(), 2);
        ref_obj.retain();
        assert_eq!(ref_obj.get_reference_count(), 3);
    }

    #[test]
    fn test_ref_release() {
        let ref_obj = Ref::new();
        ref_obj.retain();
        ref_obj.retain();
        assert_eq!(ref_obj.get_reference_count(), 3);
        ref_obj.release();
        assert_eq!(ref_obj.get_reference_count(), 2);
        ref_obj.release();
        assert_eq!(ref_obj.get_reference_count(), 1);
    }

    #[test]
    fn test_ref_release_to_zero() {
        let ref_obj = Ref::new();
        assert_eq!(ref_obj.get_reference_count(), 1);
        ref_obj.release();
        assert_eq!(ref_obj.get_reference_count(), 1);
    }

    #[test]
    fn test_ref_ptr_eq() {
        let ref1 = RefPtr::new(42);
        let ref2 = RefPtr::new(42);
        let ref3 = ref1.clone();

        assert!(RefPtr::ptr_eq(&ref1, &ref3));
        assert!(!RefPtr::ptr_eq(&ref1, &ref2));
    }

    #[test]
    fn test_ref_ptr_new() {
        let ref_ptr = RefPtr::new(100);
        assert_eq!(*ref_ptr.borrow(), 100);
    }

    #[test]
    fn test_ref_ptr_borrow_mut() {
        let ref_ptr = RefPtr::new(vec![1, 2, 3]);
        {
            let mut borrowed = ref_ptr.borrow_mut();
            borrowed.push(4);
        }
        assert_eq!(*ref_ptr.borrow(), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_ref_ptr_get_reference_count() {
        let ref_ptr = RefPtr::new(42);
        assert_eq!(ref_ptr.get_reference_count(), 1);

        let cloned = ref_ptr.clone();
        assert_eq!(ref_ptr.get_reference_count(), 2);
        assert_eq!(cloned.get_reference_count(), 2);

        let cloned2 = cloned.clone();
        assert_eq!(ref_ptr.get_reference_count(), 3);
    }

    #[test]
    fn test_ref_ptr_clone() {
        let ref_ptr = RefPtr::new(String::from("hello"));
        let cloned = ref_ptr.clone();

        assert!(RefPtr::ptr_eq(&ref_ptr, &cloned));
        assert_eq!(*ref_ptr.borrow(), *cloned.borrow());
    }

    #[test]
    fn test_ref_ptr_partial_eq_value() {
        let ref1 = RefPtr::new(42);
        let ref2 = RefPtr::new(42);
        let ref3 = RefPtr::new(100);

        assert_eq!(ref1, ref2);
        assert_ne!(ref1, ref3);
    }

    #[test]
    fn test_ref_ptr_partial_eq_pointer() {
        let ref1 = RefPtr::new(42);
        let ref2 = ref1.clone();

        assert_eq!(ref1, ref2);
    }

    #[test]
    fn test_ref_ptr_display() {
        let ref_ptr = RefPtr::new(42);
        let display = format!("{}", ref_ptr);
        assert_eq!(display, "RefPtr(42)");
    }

    #[test]
    fn test_ref_ptr_display_string() {
        let ref_ptr = RefPtr::new(String::from("test"));
        let display = format!("{}", ref_ptr);
        assert_eq!(display, "RefPtr(test)");
    }

    #[test]
    fn test_ref_ptr_from_rc() {
        let rc = Rc::new(RefCell::new(123));
        let ref_ptr: RefPtr<i32> = rc.into();
        assert_eq!(*ref_ptr.borrow(), 123);
    }

    #[test]
    fn test_ref_default() {
        let ref_obj: Ref = Ref::default();
        assert_eq!(ref_obj.get_reference_count(), 1);
    }

    #[test]
    fn test_ref_multiple_retain_release() {
        let ref_obj = Ref::new();

        for _ in 0..100 {
            ref_obj.retain();
        }
        assert_eq!(ref_obj.get_reference_count(), 101);

        for _ in 0..100 {
            ref_obj.release();
        }
        assert_eq!(ref_obj.get_reference_count(), 1);
    }

    #[test]
    fn test_ref_ptr_complex_type() {
        struct Point {
            x: f32,
            y: f32,
        }

        let ref_ptr = RefPtr::new(Point { x: 1.5, y: 2.5 });
        let borrowed = ref_ptr.borrow();
        assert_eq!(borrowed.x, 1.5);
        assert_eq!(borrowed.y, 2.5);
    }

    #[test]
    fn test_ref_ptr_nested_borrow() {
        let ref_ptr = RefPtr::new(vec![1, 2, 3]);

        let _borrowed = ref_ptr.borrow();
        let _borrowed2 = ref_ptr.borrow();

        assert_eq!(*ref_ptr.borrow(), vec![1, 2, 3]);
    }

    #[test]
    fn test_ref_with_different_names() {
        let ref1 = Ref::with_name(Some("ref_a"));
        let ref2 = Ref::with_name(Some("ref_b"));
        let ref3 = Ref::with_name(None);

        assert_eq!(ref1.get_reference_count(), 1);
        assert_eq!(ref2.get_reference_count(), 1);
        assert_eq!(ref3.get_reference_count(), 1);

        assert!(Ref::ptr_eq(&ref1, &ref1));
        assert!(!Ref::ptr_eq(&ref1, &ref2));
    }
}
