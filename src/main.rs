mod back;
mod ui;
use std::fs;
use anyhow;
use back::config::{Config, load_config, get_app_config_dir};
use ui::interface::run;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "data/"]
struct Asset;


fn main() -> anyhow::Result<()> {
    initialize_app_data()?;

    let config: Config = load_config()?;

    run(config).expect("Error run app");
    Ok(())
}

fn initialize_app_data() -> anyhow::Result<()> {
    let app_config_dir = get_app_config_dir();

    for filename in Asset::iter() {
        let target_path = app_config_dir.join(filename.as_ref());

        if !target_path.exists() {
            if let Some(parent) = target_path.parent() {
                fs::create_dir_all(parent)?;
            }

            if let Some(embedded_file) = Asset::get(filename.as_ref()) {
                fs::write(&target_path, embedded_file.data)?;
            }
        }
    }

    Ok(())
}
