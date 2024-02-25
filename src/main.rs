use color_eyre::eyre::Result;

use marge::Marge;

#[tokio::main]
async fn main() -> Result<()> {
    let marge = Marge::new();

    if let Ok(mut marge) = marge {
        let result = marge.run().await;
        result?
    }

    Ok(())
}
