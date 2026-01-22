mod core;
mod interface;
mod localization;
mod animator;
mod settings;

use interface::run;

fn main() {
    run().expect("Error run app");
}
