use ::pantheon::{pantheon, Config};

fn main() -> anyhow::Result<()> {
    let config = Config::default();
    let mut component = pantheon::new(config);
    component.initialize().map_err(|e| anyhow::anyhow!("{}", e))?;
    println!("SigmaOS-pantheon operational");
    Ok(())
}
