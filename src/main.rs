use anyhow::Result;
use clap::Parser;
use pantheon::pantheon;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    no_integration: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = pantheon::Config { 
        desktop_integration: !cli.no_integration, 
        ..Default::default() 
    };
    let mut component = pantheon::new(config);
    
    component.initialize()?;
    component.launch()?;
    
    println!("SigmaOS-pantheon ready");
    Ok(())
}
