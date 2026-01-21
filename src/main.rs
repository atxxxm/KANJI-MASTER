mod core;
mod interface;
mod localization;
mod animator;

use interface::run;

fn main() {
    run().expect("Error run app");
}
