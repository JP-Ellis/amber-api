//! Example: Get current prices for a site.
//!
//! This example demonstrates how to retrieve the current price intervals for a
//! site.
//!
//! # Prerequisites
//!
//! Set your API key in the `AMBER_API_KEY` environment variable and ensure you
//! have at least one site.
//!
//! # Run this example
//!
//! ```console
//! cargo run --example current_prices
//! ```

#![allow(
    clippy::print_stdout,
    clippy::non_ascii_literal,
    reason = "Examples are meant to demonstrate usage and use console output"
)]

use amber_api::Amber;
use anyhow::{Result, bail};

#[tokio::main]
async fn main() -> Result<()> {
    let client = Amber::default();
    let sites = client.sites().await?;
    let site = sites
        .first()
        .ok_or_else(|| anyhow::anyhow!("No sites found"))?;

    let intervals = client.current_prices().site_id(&site.id).call().await?;
    if intervals.is_empty() {
        println!("⚠️ No current prices found");
        bail!("No current prices found");
    }

    println!("✅ Found {} current price intervals\n", intervals.len());
    for (i, interval) in intervals.iter().enumerate() {
        if let Some(base) = interval.as_base_interval() {
            println!("📈 Interval {}", i.saturating_add(1));
            println!(
                "   {:<15} {} to {}",
                "Period:", base.start_time, base.end_time
            );
            println!("   {:<15} {}", "Duration:", base.duration);
            println!("   {:<15} {}", "Channel:", base.channel_type);
            println!("   {:<15} ${:.2}", "Price:", base.per_kwh);
        }
    }
    Ok(())
}
