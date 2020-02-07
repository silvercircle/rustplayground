extern crate gtk;
extern crate gio;
#[macro_use]
extern crate lazy_static;

use gtk::prelude::*;
use gio::prelude::*;

use gtk::{Application, ApplicationWindow, Button};

use std::marker::Copy;
use std::rc::Rc;
use std::cell::{Cell, RefCell};
use std::default::Default;

pub mod my_utils;
use my_utils::*;

mod experiments;
use experiments::{default_args, lazystatic};
use experiments::singleton as context;

struct Foo {
    x: i32, y: i32
}

#[derive(Debug, Copy, Clone)]
struct Point<T> {
    x: T,
    y: T,
}

struct Rect<T> {
    a: Point<T>,
    b: Point<T>,
    valid: Cell<bool>,
    foo: RefCell<Point<T>>
}

impl<T> Rect<T>  {
    pub fn greet(&self) {
        println!("Hi, I am in Rect, greeting");
    }

}

impl Rect<i32> {
    pub fn shiftpoint(&mut self) {
        self.a.x = self.a.x + 10;
    }
}

pub trait Area<T> {
    fn area(&self) -> i32;
}

impl Area<i32> for Rect<i32> {
    fn area(&self) -> i32 {
        let width: i32 = self.b.x - self.a.x;
        let height: i32 = self.b.y - self.a.y;
        self.greet();
        width * height
    }
}

fn main() {
    let mut ctx = context::get_instance();

    let application = Application::new(
        Some("com.github.gtk-rs.examples.basic"),
        Default::default(),
    ).expect("failed to initialize GTK application");

    application.connect_activate(|app| {
        let window = ApplicationWindow::new(app);

        window.set_title("First GTK+ Program");
        window.set_default_size(350, 70);

        let button = Button::new_with_label("Click me!");
        button.connect_clicked(|_| {
            println!("Clicked!");
        });
        window.add(&button);
        window.resize(500, 200);
        window.show_all();
    });
    testtest(&mut ctx);
    application.run(&[]);
    ctx.cleanup();
}

fn testtest(ctx: &mut context::Context) {
    ctx._use_count = ctx._use_count + 1;
    println!("Hello, world!");

    let mut a1: Foo  = Foo { x:10, y:10 };
    let mut a2 = Foo { x:20, y:20};             // equivalent to a1

    println!("Foo is {x}, {y}", x = a1.x, y = a1.y);
    let p1: &mut Foo = &mut a1;
    p1.x = p1.x + 1;
    p1.y = p1.y + 1;
    println!("Now, Foo is {x}, {y}", x = p1.x, y = p1.y);
    p1.x = p1.x + 1;
    p1.y = p1.y + 1;
    println!("Now, Foo is {x}, {y}", x = p1.x, y = p1.y);
    println!("Finally, Foo is {x}, {y}", x = a1.x, y = a1.y);

    increment(&mut a2);

    println!("Finally, a2 is {}, {}", a2.x, a2.y);

    let mut _point1: Point<i32> = Point {x: 10, y:10};
    let mut _point2: Point<i32> = Point {x: 20, y:20};
    let mut _point3: Point<i32> = Point {x: 30, y:40};

    let mut _rect: Rect<i32> = Rect { a: _point1, b: _point2,
        valid: Cell::new(false), foo: RefCell::new( Point {x:30, y:30} )};

    let bar: Rc<Rect<i32>> = Rc::new(Rect {a: _point1.clone(),
        b: _point2.clone(), valid: Cell::new(false),
        foo: RefCell::new(Point{x: 30, y: 40}) });

    do_rect(&_rect);
    _rect.shiftpoint();
    println!("{}", _rect.area());
    println!("{}", bar.area());
    bar.valid.set(true);

    let f = bar.foo.borrow();
    drop(f);

    let mut t = bar.foo.borrow_mut();
    t.x = 20;
    t.y = 50;

    my_test();

    let mut y = Vec::new();

    for i in 1..10 {
        y.push(i.to_string());
    }

    let mut _foo = vec![1, 2, 3, 4];
    do_foo(&mut _foo, &mut 10);
    _foo.push(10);

    experiments::test::run();
    
    let mut s = getstring();
    s.push_str("appended");
    default_args::run();
    lazystatic::run();

    let mut _instance = lazystatic::GLOBAL.lock().unwrap();
    _instance.greet();
    drop(_instance);
}

fn do_rect(_r: &Rect<i32>) {

}
fn do_foo(_v: &mut Vec<i32>, _i: &mut i32) {
    _v.push(20);
    println!("The value of i is: {}", *_i);
    *_i = 500;
    println!("The value of i is: {}", *_i);
}

fn increment(foo: &mut Foo) -> &Foo {
    foo.x = foo.x + 1;
    foo.y = foo.y + 1;
    foo
}

fn getstring() -> String {
    let s = String::from("Hello ihr Affen");
    s
}

pub fn runme() {
    println!("runme() in main module");
}
