use anyhow::{Context, Result};
use nmrs::NetworkManager;

#[tokio::main]
async fn main() -> Result<()> {
    let nm = NetworkManager::new().await?;

    let networks = nm
        .list_networks(None)
        .await
        .context("failed to list networks")?;
    for net in &networks {
        println!("{} - Signal: {}%", net.ssid, net.strength.unwrap_or(0));
    }

    let ssid = nm
        .current_ssid()
        .await
        .context("failed to get current SSID")?;
    println!("Connected to: {}", ssid);

    Ok(())
}
