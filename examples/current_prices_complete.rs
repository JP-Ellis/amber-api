//! Example: Get previous, current, and next price intervals for a site
//!
//! This example demonstrates how to retrieve previous, current, and future price intervals for a site using the `previous` and `next` parameters.
//!
//! # Prerequisites
//!
//! Set your API key in the `AMBER_API_KEY` environment variable and ensure you have at least one site.
//!
//! # Run this example
//!
//! ```console
//! cargo run --example current_prices_complete
//! ```

#![allow(
    clippy::print_stdout,
    clippy::non_ascii_literal,
    reason = "Examples are meant to demonstrate usage and use console output"
)]

use amber_api::Amber;
use anyhow::{Result, bail};

fn main() -> Result<()> {
    let client = Amber::default();
    let sites = client.sites()?;
    let site = sites
        .first()
        .ok_or_else(|| anyhow::anyhow!("No sites found"))?;

    // Request previous 2, current, and next 3 intervals
    let intervals = client
        .current_prices()
        .site_id(&site.id)
        .previous(2)
        .next(3)
        .call()?;

    if intervals.len() == 12 {
        println!(
            "✅ Found 12 price intervals (2 previous, 1 current, 3 next; all 2x for feed-in and general)\n"
        );
    } else {
        println!(
            "⚠️ Expected 12 intervals (2 previous, 1 current, 3 next; all 2x for feed-in and general), found {}",
            intervals.len()
        );
        bail!("Unexpected number of intervals");
    }

    for (i, interval) in intervals.iter().enumerate() {
        if let Some(base) = interval.as_base_interval() {
            let interval_type = if interval.is_actual_interval() {
                "Historical (Actual)"
            } else if interval.is_current_interval() {
                "Current"
            } else if interval.is_forecast_interval() {
                "Forecast"
            } else {
                "Unknown"
            };
            println!("📈 Interval {}", i.saturating_add(1));
            println!("   {:<15} {}", "Type:", interval_type);
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
