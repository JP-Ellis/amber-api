//! # Example
//!
//! Obtain previous, current and future renewable energy data. This example
//! showcases all the parameters that can be used to retrieve a comprehensive
//! view of renewable energy generation.
//!
//! # Run this example
//!
//! ```console
//! cargo run --example renewables_complete
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
        .state(models::State::Vic)
        .next(6)
        .previous(3)
        .resolution(models::Resolution::ThirtyMinute)
        .call()?;

    if renewables.len() == 10 {
        println!("✅ Found 10 renewable energy entries");
    } else {
        println!("⚠️ Expected 10 entries, found {}", renewables.len());
        bail!("Unexpected number of entries");
    }

    for (index, entry) in renewables.iter().enumerate() {
        println!("📊 Entry {}", index.saturating_add(1));

        match entry {
            models::Renewable::ActualRenewable(actual) => {
                assert!(actual.base.duration == 30, "Duration should be 30 minutes");
                println!("   {:<15} Historical (Actual)", "Type:");
                println!("   {:<15} {:.1}%", "Renewables:", actual.base.renewables);
                println!("   {:<15} {} minutes", "Duration:", actual.base.duration);
                println!(
                    "   {:<15} {} to {}",
                    "Period:", actual.base.start_time, actual.base.end_time
                );
                println!("   {:<15} {}", "Descriptor:", actual.base.descriptor);
            }
            models::Renewable::CurrentRenewable(current) => {
                assert!(current.base.duration == 30, "Duration should be 30 minutes");
                println!("   {:<15} Current", "Type:");
                println!("   {:<15} {:.1}%", "Renewables:", current.base.renewables);
                println!("   {:<15} {} minutes", "Duration:", current.base.duration);
                println!("   {:<15} {}", "Date:", current.base.date);
                println!("   {:<15} {}", "Descriptor:", current.base.descriptor);
            }
            models::Renewable::ForecastRenewable(forecast) => {
                assert!(
                    forecast.base.duration == 30,
                    "Duration should be 30 minutes"
                );
                println!("   {:<15} Forecast", "Type:");
                println!("   {:<15} {:.1}%", "Renewables:", forecast.base.renewables);
                println!("   {:<15} {} minutes", "Duration:", forecast.base.duration);
                println!("   {:<15} {}", "Date:", forecast.base.date);
                println!("   {:<15} {}", "Descriptor:", forecast.base.descriptor);
            }
            _ => {
                bail!("Unexpected renewable energy data type");
            }
        }
    }

    Ok(())
}
