
pub struct Aoptargs {
    pub i: i32,
    pub f: f64,
    pub foo: String
}

impl Default for Aoptargs {
    fn default() -> Self {
        Self {
            i: 10,
            f: 1.0,
            foo: "hallo".to_string()
        }
    }
}
pub fn optarg(args: &Aoptargs) {
    println!("{}", args.foo);
}

pub fn optarg1(i: i32, f: Option<i32>, s: Option<&String>) {
    println!("{}", s.unwrap_or(&"no string given".to_string()));
}

