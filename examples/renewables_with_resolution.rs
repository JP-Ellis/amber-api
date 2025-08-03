//! # Example
//!
//! Obtain the current renewables data with a finer grained resolution of just 5
//! minutes.
//!
//! # Run this example
//!
//! ```console
//! cargo run --example renewables_with_resolution
//! ```

#![allow(
    clippy::print_stdout,
    clippy::non_ascii_literal,
    reason = "Examples are meant to demonstrate usage and use console output"
)]

use amber_api::models;
use anyhow::{Result, bail};

fn main() -> Result<()> {
    let client = amber_api::Amber::default();
    let renewables = client
        .current_renewables()
        .state(models::State::Sa)
        .resolution(models::Resolution::FiveMinute)
        .call()?;

    match renewables.len() {
        0 => {
            println!("⚠️ No renewable energy data available");
            bail!("No renewable energy data available")
        }
        1 => println!("✅ Found 1 renewable energy entry"),
        _ => {
            println!("✅ Found {} renewable energy entries", renewables.len());
            bail!("Found multiple entries, expected only one")
        }
    }

    if let Some(models::Renewable::CurrentRenewable(current)) = renewables.first() {
        assert!(current.base.duration == 5, "Duration should be 5 minutes");
        println!("\n📊 Current Data");
        println!("   {:<15} {:.1}%", "Renewables:", current.base.renewables);
        println!("   {:<15} {} minutes", "Duration:", current.base.duration);
        println!(
            "   {:<15} {} to {}",
            "Period:", current.base.start_time, current.base.end_time
        );
        println!("   {:<15} {}", "Descriptor:", current.base.descriptor);
    } else {
        bail!("Unexpected renewable energy data type");
    }

    Ok(())
}
