mod back;
mod ui;
use anyhow;

use back::config::{Config, load_config};
use std::path::Path;
use ui::interface::run;

fn main() -> anyhow::Result<()> {
    let config: Config = if !Path::new("config.toml").exists() {
        Config::default()
    } else {
        load_config("config.toml")?
    };

    run(config).expect("Error run app");
    Ok(())
}
