use std::ops::Deref;

struct Test {
  x: i32,
  name: String,
}

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

// drop is similar to destructors in C++. The code runs when
// the object goes out of scope
impl<T> Drop for MyBox<T> {
  fn drop(&mut self) {
    println!("Dropping a Mybox object");
  }
}

impl Drop for Test {
  fn drop(&mut self) {
    println!("Dropping a Test object with the name {}", self.name);
  }
}

fn greet(name: &str) {
  println!("Hello, {}!", name);
}

fn greet_test(o: &Test) {
  println!("Test object's name is: {} and the value is {}", o.name, o.x);
}

pub fn run() {
  println!("Running smart pointer tests");

  // let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))));

  let mut x = 5u8;
  let y = &mut x;

  *y = 10u8;

  // demonstrate deref coercion
  let m = Box::new(String::from("Foobar"));

  // the following uses a compiler feature called deref coercion to simplify code
  // m is a Box<String> and greet() accepts a reference to a string slice, thus
  // the correct form would have to be hello(&(*m)[..]) - first dereference m to
  // get the String, take the full string as a slice [..] and then pass a reference
  // to the slice to our function.
  //
  // deref coercion simplifies this, the compiler can automatically deduce that we
  // want to pass a reference to the inner value of the Box<String>
  // deref coercion requires that the objects implements the Deref trait, which the
  // Box smart pointer type does by default.
  greet(&m);
  drop(m);

  // the same deref coercion with another inner value type. Works analog to a String
  // greet_test() expects a reference to a Test structure. We can pass a reference
  // to the Box<Test> instead.
  let f = Box::new(Test {
    x: 10,
    name: "foo".to_string(),
  });
  greet_test(&f);
  drop(f);

  let m = MyBox::new(String::from("Foo"));
  drop(m);

  let mut m = Test {
    x: 10,
    name: "Foo".to_string(),
  };

  let _m: &mut Test = &mut m;
  _m.name = "Bar".to_string();

  // dropping the reference does not call the Drop implementation
  drop(_m);

  // here, the owner goes out of scope, so Drop is called.
  drop(m);
}
