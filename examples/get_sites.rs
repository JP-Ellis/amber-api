//! Example: Get all sites linked to your account.
//!
//! This example demonstrates how to retrieve all electricity sites
//! associated with your Amber account.
//!
//! # Prerequisites
//!
//! Set your API key in the `AMBER_API_KEY` environment variable:
//! ```console
//! export AMBER_API_KEY="your-api-key-here"
//! ```
//!
//! # Run this example
//!
//! ```console
//! cargo run --example get_sites
//! ```

#![allow(
    clippy::print_stdout,
    clippy::non_ascii_literal,
    reason = "Examples are meant to demonstrate usage and use console output"
)]

use anyhow::{Result, bail};

#[tokio::main]
async fn main() -> Result<()> {
    let client = amber_api::Amber::default();
    let sites = client.sites().await?;

    match sites.len() {
        0 => {
            println!("⚠️ No sites found");
            bail!("No sites found")
        }
        1 => println!("✅ Found 1 site"),
        _ => println!("✅ Found {} sites", sites.len()),
    }

    for (index, site) in sites.iter().enumerate() {
        println!("\n📍 Site {} ({})", index.saturating_add(1), site.id);
        println!("   {:<20} {}", "MNI:", site.nmi);
        println!("   {:<20} {}", "Network:", site.network);
        println!("   {:<20} {}", "Status:", site.status);
        println!("   {:<20} {}", "Channels:", site.channels.len());
        println!(
            "   {:<20} {} minutes",
            "Interval Length:", site.interval_length
        );

        if let Some(active_from) = &site.active_from {
            println!("   {:<20} {}", "Active From:", active_from);
        }

        if let Some(closed_on) = &site.closed_on {
            println!("   {:<20} {}", "Closed On:", closed_on);
        } else {
            println!("   {:<20} Active", "Status:");
        }
    }

    Ok(())
}
