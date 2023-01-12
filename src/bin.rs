use anyhow::Result;
use clap::Parser;

#[derive(Debug, clap::Parser)]
#[clap(version = env!("CARGO_PKG_VERSION"))]
#[clap(about = "Convert B58 address to animal name")]
struct Cli {
    /// The address to convert
    address: String,
}
fn main() -> Result<()> {
    let cli = Cli::parse();
    let animal_name = cli.address.parse::<angry_purple_tiger::AnimalName>()?;
    println!("{animal_name}");
    Ok(())
}
