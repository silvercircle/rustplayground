/*
 * some patterns for using default and optional arguments in rust
 * There are no C++-style default args nor are there named arguments or
 * function overloading.
 *
 * argument structures or Option<T> can be used to simulate optional
 * parameters in Rust though.
 */

pub struct Aoptargs {
  pub i: i32,
  pub f: f64,
  pub foo: String,
}

impl Default for Aoptargs {
  fn default() -> Self {
    Self {
      i: 10,
      f: 1.0,
      foo: "hallo".to_string(),
    }
  }
}

/// method 1: Use a struct with an implementation of the Default trait
///
fn optarg(args: &Aoptargs) {
  println!("{}", args.foo);
}

/// method 2: Use Option<T> und unwrap_or() with the default
/// value
fn optarg1(_i: i32, _f: Option<i32>, _s: Option<&String>) {
  println!("{}", _s.unwrap_or(&"no string given".to_string()));
}

pub fn run() {
  // method 1: param struct
  // only specify parameters that differ from default values
  optarg(&Aoptargs {
    i: 20,
    ..Aoptargs::default()
  });

  // method 2: use Option<T>
  optarg1(1, Some(10), Some(&"string was given".to_string()));
  optarg1(1, Some(10), None);
}
