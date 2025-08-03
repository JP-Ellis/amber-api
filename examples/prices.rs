//! Example: Get historical prices for a site
//!
//! This example demonstrates how to retrieve historical price intervals for a site with all optional arguments.
//!
//! # Prerequisites
//!
//! Set your API key in the `AMBER_API_KEY` environment variable and ensure you have at least one site.
//!
//! # Run this example
//!
//! ```console
//! cargo run --example prices
//! ```

#![allow(
    clippy::print_stdout,
    clippy::non_ascii_literal,
    reason = "Examples are meant to demonstrate usage and use console output"
)]

use amber_api::{Amber, models};
use anyhow::{Result, anyhow, bail};
use jiff::civil::Date;

fn main() -> Result<()> {
    let client = Amber::default();
    let sites = client.sites()?;
    let site = sites
        .first()
        .ok_or_else(|| anyhow::anyhow!("No sites found"))?;
    let site_id = &site.id;

    let intervals = client
        .prices()
        .site_id(site_id)
        .start_date(Date::new(2025, 8, 1).map_err(|e| anyhow!("Failed to create start date: {e}"))?)
        .end_date(Date::new(2025, 8, 1).map_err(|e| anyhow!("Failed to create end date: {e}"))?)
        .resolution(models::Resolution::ThirtyMinute)
        .call()?;

    if intervals.is_empty() {
        println!("⚠️ No price intervals found");
        bail!("No price intervals found");
    }

    println!("✅ Found {} price intervals", intervals.len());
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
