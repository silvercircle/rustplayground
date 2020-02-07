// experiments is a module with sub modules, therefore it should live in
// a mod.rs file inside the folder designating its name (experiments in that case)
//

pub mod test {              // this is experiments::test
    pub fn run() {
        println!("We are in experiments::test::run()");
        // super accesses the parent module, since we are at level 2
        // of the hierarchy (0 is the root), we need to chain calls to
        // super
        super::super::runme();
    }
}

// more sub-modules in the experiments name space are now declared,
// but their code is located in separate files. This is good practice for
// larger modules as it helps to keep individual files small in size.

pub mod default_args;       // -> module experiments::default_args
pub mod singleton;          // -> experiments::singleton
pub mod lazystatic;         // -> .... you get the idea
