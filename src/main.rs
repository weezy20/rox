#![allow(unused)]
use rox::Rox;
use color_eyre::eyre::Result;
fn main() -> Result<()>{
    color_eyre::install()?;
    if let Some(config_file_path) = std::env::args().nth(1) {
        let rox = Rox::interpret(&config_file_path)?;
        println!("**Printing Name Value pairs**");
        for item in rox.switches {
            println!("{:?}", item);
        }
        println!("**Printing invalid**");
        for comment in rox.invalid {
            println!("{}", comment);
        }
    }
    else {
        eprintln!("Usage: `rox {{rox file}}`");
    }
    Ok(())
}
