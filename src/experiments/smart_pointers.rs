use std::ops::Deref;

struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

pub fn run() {
    println!("Running smart pointer tests");

    let foo: Box<i32> = Box::new(10);
    let bar = Box::new(2.34);

    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))));


    let mut x = 5u8;
    let y = &mut x;

    *y = 10u8;

    let m = Box::new(String::from("Rust"));
    hello(&m);
}

