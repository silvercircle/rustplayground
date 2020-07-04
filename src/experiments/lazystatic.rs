/*
 * use lazy_static for a global shared values with read/write access control
 * via a synchronization mechanisms.
 *
 * demonstrate simple use of Mutexes and R/W locks to access global shared
 * values.
 */

use std::sync::{Mutex, RwLock};

#[derive(Debug)]
pub struct Test {
    pub name:       String,
    pub ids:        Vec<i32>,
    pub use_count:  i64
}

impl Test {
    pub fn greet(&self) {
        println!("Hi, I'm a Test structure, my name is {} and my use count is {}.", self.name, self.use_count);
    }
}

impl Default for Test {
    fn default() -> Self {
        Test {
            name: "foobar".to_string(),
            use_count: 0,
            ids: vec![1000]
        }
    }
}

lazy_static!(
    // shared global struct Test, protected by Mutex (one lock at any given time)
    pub static ref GLOBAL: Mutex<Test> = Mutex::new( Test { name: "simplemutex".to_string(), .. Test::default() } );

    // shared global struct Test, protected by R/W lock (only one write lock,
    // but multiple read locks possible)
    pub static ref RWGLOBAL: RwLock<Test> = RwLock::new( Test { name: "readwritelock".to_string(), .. Test::default() } );
);


pub fn run() {
    // Acquire an instance of GLOBAL and lock it. Only one can be held at any given time
    // other threads have to wait until this instance is dropped or goes out of scope
    // ordinary Mutexes do not distinguish between read and write access. Even read-only
    // locks are exclusive.
    let mut _instance = GLOBAL.lock().unwrap();
    _instance.greet();
    _instance.use_count = _instance.use_count + 1;

    // _instance is dropped when it goes out of scope, that is at the end
    // of this function. the protected value is then unlocked and another
    // lock can be acquired.
    drop(_instance);            // or manually drop it here.


    // read write locks allow multiple read instances at the same time
    let mut _rwinstance = RWGLOBAL.read().unwrap();
    let mut _rwinstance1 = RWGLOBAL.read().unwrap();
    _rwinstance.greet();
    _rwinstance1.greet();


    // _rwinstance1.use_count = _rwinstance1.use_count + 1
    // The above line would lead to a compilation error, even though the instance
    // variable is declared mut, it cannot modify the inner value. The .read() lock
    // only allows read access.
    drop(_rwinstance1);
    drop(_rwinstance);

    // obtaining a write lock permits us write access to the inner value.
    // only one write lock can be held at any given time.
    let mut _winstance = RWGLOBAL.write().unwrap();
    _winstance.use_count = _winstance.use_count + 1;
    debug_assert_eq!(_winstance.use_count, 1);
}
