use std::fmt::{Display, Formatter};
use std::ops::Deref;
#[derive(Debug)]
pub struct MyBox<T> {
    value: T
}

impl<T> MyBox<T> {
    pub fn new(value: T) -> MyBox<T> {
        MyBox {
            value,
        }
    }
}

impl<T: Display> Display for MyBox<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

pub fn print_box(x: &MyBox<usize>) {
    println!("Debug print: {:?}", x);
    println!("The context of the box are: {}", x)
}