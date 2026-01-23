mod ui;
mod back;
use anyhow;

use ui::interface::run;
use std::path::Path;
use back::config::{load_config, Config};

fn main() -> anyhow::Result<()> {
    let config: Config = if !Path::new("config.toml").exists() {
        Config::default()
    } else {
        load_config("config.toml")?
    };

    run(config).expect("Error run app");
    Ok(())
}
