mod ui;
mod back;

use ui::interface::run;

fn main() {
    run().expect("Error run app");
}
