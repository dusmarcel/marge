use color_eyre::eyre::Result;

use marge::Marge;

#[tokio::main]
async fn main() -> Result<()> {
    let marge = Marge::new();

    let result = marge.run().await;

    result?;

    Ok(())
}
