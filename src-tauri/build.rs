use std::error::Error;

use vergen::{vergen, Config};

fn main() -> Result<(), Box<dyn Error>> {
    vergen(Config::default())?;
    tauri_build::build();
    Ok(())
}
