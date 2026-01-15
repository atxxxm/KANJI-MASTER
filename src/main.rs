mod core;
mod interface;
mod localization;

use interface::run;

fn main() {
    run().expect("Error run app");
}
