//! Example: Get usage data for a site.
//!
//! This example demonstrates how to retrieve usage data for a site for a given
//! date range.
//!
//! # Prerequisites
//!
//! Set your API key in the `AMBER_API_KEY` environment variable and ensure you
//! have at least one site.
//!
//! # Run this example
//!
//! ```console
//! cargo run --example usage
//! ```

#![allow(
    clippy::print_stdout,
    clippy::non_ascii_literal,
    reason = "Examples are meant to demonstrate usage and use console output"
)]

use amber_api::Amber;
use anyhow::{Result, bail};
use jiff::ToSpan as _;

#[tokio::main]
async fn main() -> Result<()> {
    let client = Amber::default();
    let sites = client.sites().await?;
    let site = sites
        .first()
        .ok_or_else(|| anyhow::anyhow!("No sites found"))?;

    let start_date = jiff::Zoned::now()
        .round(jiff::Unit::Day)?
        .date()
        .saturating_sub(7_i8.days());

    let usage_data = client
        .usage()
        .site_id(&site.id)
        .start_date(start_date)
        .end_date(start_date)
        .call()
        .await?;

    if usage_data.is_empty() {
        println!("⚠️ No usage data found");
        bail!("No usage data found");
    }

    println!("✅ Found {} usage entries\n", usage_data.len());
    for (i, usage) in usage_data.iter().enumerate() {
        println!("📊 Usage {}", i.saturating_add(1));
        println!(
            "   {:<15} {} to {}",
            "Period:", usage.base.start_time, usage.base.end_time
        );
        println!("   {:<15} {}", "Date:", usage.base.date);
        println!("   {:<15} {:.3} kWh", "kWh:", usage.kwh);
        println!("   {:<15} ${:.2}", "Cost:", usage.cost);
    }
    Ok(())
}
