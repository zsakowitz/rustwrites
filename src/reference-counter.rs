//! A struct that counts the numbers of its references.
#![allow(dead_code, unused_imports, unused_variables)]

use std::{
    ops::{Deref, DerefMut},
    rc::Rc,
};

fn main() {
    let rc: MyRc<i32> = MyRc::new(5);
    let d = rc.to_le();
}

struct RcBox<T> {
    value: T,
    refs: i32,
}

impl<T> RcBox<T> {
    fn new(value: T) -> Self {
        Self { value, refs: 1 }
    }
}

struct MyRc<T> {
    rc_box: RcBox<T>,
}

impl<T> MyRc<T> {
    fn new(value: T) -> Self {
        Self {
            rc_box: RcBox::new(value),
        }
    }
}

impl<T> Drop for MyRc<T> {
    fn drop(&mut self) {
        self.rc_box.refs -= 1;
    }
}

impl<T> Deref for MyRc<T> {
    type Target = T;

    fn deref(&self) -> &T {
        return &self.rc_box.value;
    }
}

impl<T> DerefMut for MyRc<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self.rc_box.value;
    }
}
