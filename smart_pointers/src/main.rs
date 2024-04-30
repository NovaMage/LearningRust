#![forbid(unsafe_code)]

use std::fmt::{Debug, Display, Formatter};
use std::sync::{Arc, Mutex};
use std::{thread, time};

use once_cell::sync::OnceCell;

use crate::boxes::MyBox;

#[derive(Debug)]
struct UserCreateCommand<'a> {
    username: &'a str,
    password: &'a str,
    priority: u16,
}

#[derive(Debug)]
struct User<'a> {
    username: &'a str,
    password: &'a str,
    deleted: bool,
}

impl Display for User<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

mod boxes;

static BOXED: OnceCell<MyBox<usize>> = OnceCell::new();

fn create_user(input: UserCreateCommand) -> User {
    take_partial(input.password);
    let value = User {
        username: input.username,
        password: input.password,
        deleted: false,
    };
    value
}

fn take_partial(str: &str) {
    println!("Printing the string! {}", str)
}

fn main() {
    let x = Arc::new(Mutex::new(1));
    // thread::scope(|s| {
    let reference = Arc::clone(&x);
    thread::spawn(move || {
        *reference.lock().unwrap() += 27;
        println!("Alphonse");
        return 137;
    });
    let reference = Arc::clone(&x);
    thread::spawn(move || {
        *reference.lock().unwrap() += 8;
        println!("Gaston");
        return 139;
    });
    println!("Hello from the main thread!");
    thread::sleep(time::Duration::new(0, 1000));
    println!("The value of x is :{}", x.lock().unwrap());

    let values: [u8; 5] = [2, 4, 6, 8, 10];
    some_fun_with_slice(values);
    BOXED.set(MyBox::new(175)).unwrap();

    let command = UserCreateCommand {
        username: "Some user",
        password: "Some password",
        priority: 15,
    };
    println!("Priority is {}", command.priority);
    let a = create_user(command);
    println!("User values are {} - {} - {}", a.username, a.password, a.deleted);
    let the_value = some_fun();
    boxes::print_box(the_value);
    send_test(the_value);
    sync_test(the_value);

    println!("The value of x is :{}", x.lock().unwrap());

    println!("GCD: {}", gcd(573, 217));
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

fn some_fun_with_slice<T: Display>(arr: [T; 5]) {
    for x in arr {
        println!("The element is {}", x);
    }
}

fn some_fun() -> &'static MyBox<usize> {
    BOXED.get().unwrap()
}

fn send_test<T: Send>(_: T) {
    println!("Value is safe to send!")
}

fn sync_test<T: Sync>(_: T) {
    println!("Value is safe to sync!")
}
