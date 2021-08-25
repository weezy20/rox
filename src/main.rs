#![allow(unused)]
use rox::Rox;
use color_eyre::eyre::Result;
fn main() -> Result<()>{
    color_eyre::install()?;
    if let Some(config_file_path) = std::env::args().nth(1) {
        Rox::interpret(&config_file_path);
    }
    else {
        eprintln!("Usage: `rox {{rox file}}`");
    }
    Ok(())
}