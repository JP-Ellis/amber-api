//! Integration tests.
//!
//! This module contains integration tests for the Amber API client.

#![cfg(test)]
#![expect(
    clippy::shadow_reuse,
    reason = "Intentional shadowing with rstest async fixture"
)]

use amber_api::{Amber, models};
use anyhow::{Result, anyhow};
use pretty_assertions::assert_eq;
use rstest::{fixture, rstest};

/// Create a test fixture for the Amber client
///
/// This fixture creates a default Amber client that will use the
/// `AMBER_API_KEY` environment variable if available, or will be
/// unauthenticated for tests that don't require it.
#[fixture]
fn amber_client() -> Amber {
    Amber::default()
}

/// Test the `current_renewables()` method to ensure it works with default
/// arguments.
#[rstest]
#[tokio::test]
async fn current_renewables_default(amber_client: Amber) -> Result<()> {
    let renewables = amber_client
        .current_renewables()
        .state(models::State::Vic)
        .call()
        .await?;

    assert_eq!(renewables.len(), 1);

    let entry = renewables
        .first()
        .ok_or_else(|| anyhow!("Expect at least one entry"))?;
    assert!(entry.is_current_renewable());
    let base = entry.as_base_renewable();
    assert!(base.start_time < base.end_time);
    assert_eq!(base.duration, 30);

    Ok(())
}

/// Test the `current_renewables()` method ensuring it works with all optional
/// arguments.
#[rstest]
#[tokio::test]
async fn current_renewables_optional(amber_client: Amber) -> Result<()> {
    let renewables = amber_client
        .current_renewables()
        .state(models::State::Vic)
        .previous(6)
        .next(3)
        .resolution(models::Resolution::FiveMinute)
        .call()
        .await?;

    assert_eq!(renewables.len(), 10);

    let actual_count = renewables
        .iter()
        .filter(|e| e.is_actual_renewable())
        .count();
    let current_count = renewables
        .iter()
        .filter(|e| e.is_current_renewable())
        .count();
    let forecast_count = renewables
        .iter()
        .filter(|e| e.is_forecast_renewable())
        .count();
    assert_eq!(actual_count, 6);
    assert_eq!(current_count, 1);
    assert_eq!(forecast_count, 3);

    for entry in renewables {
        let base = entry.as_base_renewable();
        assert!(base.start_time < base.end_time);
        assert_eq!(base.duration, 5);
    }

    Ok(())
}

/// Test the `sites()` method to ensure it returns expected data structure
#[rstest]
#[tokio::test]
async fn sites_retrieval(amber_client: Amber) -> Result<()> {
    let sites = amber_client.sites().await?;

    assert!(!sites.is_empty(), "Expected non-empty sites list");
    let site = sites
        .first()
        .ok_or_else(|| anyhow!("Expected at least one site"))?;

    assert!(!site.id.is_empty(), "Site ID should not be empty");
    assert!(!site.nmi.is_empty(), "Site NMI should not be empty");
    assert!(!site.network.is_empty(), "Site network should not be empty");
    assert!(
        site.interval_length == 5 || site.interval_length == 30,
        "Site interval length should be 5 or 30"
    );

    // Test the Display implementation
    let display_string = format!("{site}");
    assert!(display_string.contains(&site.id));
    assert!(display_string.contains(&site.nmi));
    assert!(display_string.contains(&site.network));

    Ok(())
}

/// Return a site ID.
///
/// This is required for additional tests.
#[fixture]
async fn site_id(amber_client: Amber) -> String {
    amber_client
        .sites()
        .await
        .expect("Failed to obtain sites")
        .into_iter()
        .next()
        .map(|site| site.id)
        .expect("Expected at least one site")
}

/// Test the `prices()` method to ensure it works with default arguments.
#[rstest]
#[tokio::test]
async fn prices_default(amber_client: Amber, #[future] site_id: String) -> Result<()> {
    let site_id = site_id.await;
    let intervals = amber_client.prices().site_id(&site_id).call().await?;

    assert!(!intervals.is_empty(), "Expected non-empty prices list");
    let interval = intervals
        .first()
        .ok_or_else(|| anyhow!("Expected at least one price"))?;

    assert!(interval.is_actual_interval());
    let base_interval = interval.as_base_interval().expect("Expected base interval");
    assert_eq!(base_interval.duration, 5);

    Ok(())
}

/// Test the `prices()` method to ensure it works with all optional arguments.
#[rstest]
#[tokio::test]
async fn prices_optional(amber_client: Amber, #[future] site_id: String) -> Result<()> {
    let site_id = site_id.await;
    let intervals = amber_client
        .prices()
        .site_id(&site_id)
        .start_date(
            jiff::civil::Date::new(2025, 8, 1)
                .map_err(|e| anyhow!("Failed to create start date: {e}"))?,
        )
        .end_date(
            jiff::civil::Date::new(2025, 8, 1)
                .map_err(|e| anyhow!("Failed to create end date: {e}"))?,
        )
        .resolution(models::Resolution::ThirtyMinute)
        .call()
        .await?;

    assert!(!intervals.is_empty(), "Expected non-empty prices list");
    assert_eq!(intervals.len(), 96, "Expected 96 intervals");
    assert_eq!(
        intervals
            .iter()
            .filter(|i| i
                .as_base_interval()
                .is_some_and(|b| b.channel_type == models::ChannelType::General))
            .count(),
        48
    );
    assert_eq!(
        intervals
            .iter()
            .filter(|i| i
                .as_base_interval()
                .is_some_and(|b| b.channel_type == models::ChannelType::FeedIn))
            .count(),
        48
    );

    Ok(())
}

/// Test the `current_prices()` method to ensure it works with default
/// arguments.
#[rstest]
#[tokio::test]
async fn current_prices_default(amber_client: Amber, #[future] site_id: String) -> Result<()> {
    let site_id = site_id.await;
    let intervals = amber_client
        .current_prices()
        .site_id(&site_id)
        .call()
        .await?;

    assert!(
        !intervals.is_empty(),
        "Expected non-empty current prices list"
    );
    let interval = intervals
        .first()
        .ok_or_else(|| anyhow!("Expected at least one current price"))?;

    assert!(interval.is_current_interval());
    let base_interval = interval.as_base_interval().expect("Expected base interval");
    assert!(
        base_interval.duration == 5 || base_interval.duration == 30,
        "Expected duration to be 5 or 30 minutes"
    );

    Ok(())
}

/// Test the `current_prices()` method to ensure it works with all optional
/// arguments.
#[rstest]
#[tokio::test]
async fn current_prices_optional(amber_client: Amber, #[future] site_id: String) -> Result<()> {
    let site_id = site_id.await;
    let intervals = amber_client
        .current_prices()
        .site_id(&site_id)
        .previous(6)
        .next(3)
        .resolution(models::Resolution::ThirtyMinute)
        .call()
        .await?;

    assert!(
        !intervals.is_empty(),
        "Expected non-empty current prices list"
    );

    let actual_count = intervals.iter().filter(|i| i.is_actual_interval()).count();
    let current_count = intervals.iter().filter(|i| i.is_current_interval()).count();
    let forecast_count = intervals
        .iter()
        .filter(|i| i.is_forecast_interval())
        .count();

    // We expect double the number listed above as Amber reports on the feed-in
    // and general channels separately.
    assert_eq!(actual_count, 12, "Expected at most 12 actual intervals");
    assert_eq!(current_count, 2, "Expected exactly 2 current interval");
    assert_eq!(forecast_count, 6, "Expected at most 6 forecast intervals");

    // Verify all intervals have the correct duration
    for interval in &intervals {
        if let Some(base) = interval.as_base_interval() {
            assert_eq!(base.duration, 30, "Expected 30-minute resolution");
        }
    }

    Ok(())
}

/// Test the `usage()` method to ensure it works with required arguments.
#[rstest]
#[tokio::test]
async fn usage_default(amber_client: Amber, #[future] site_id: String) -> Result<()> {
    let site_id = site_id.await;
    let start_date = jiff::civil::Date::new(2025, 11, 1)
        .map_err(|e| anyhow!("Failed to create start date: {e}"))?;
    let end_date = jiff::civil::Date::new(2025, 11, 1)
        .map_err(|e| anyhow!("Failed to create end date: {e}"))?;

    let usage_data = amber_client
        .usage()
        .site_id(&site_id)
        .start_date(start_date)
        .end_date(end_date)
        .call()
        .await?;

    assert!(!usage_data.is_empty(), "Expected non-empty usage data list");
    let usage = usage_data
        .first()
        .ok_or_else(|| anyhow!("Expected at least one usage entry"))?;

    assert!(
        !usage.channel_identifier.is_empty(),
        "Channel identifier should not be empty"
    );
    assert!(usage.kwh >= 0.0_f64, "kWh should be non-negative");
    assert_eq!(
        usage.base.date, start_date,
        "Date should match requested date"
    );

    Ok(())
}

/// Test the `usage()` method with a multi-day date range.
#[rstest]
#[tokio::test]
async fn usage_multi_day(amber_client: Amber, #[future] site_id: String) -> Result<()> {
    let site_id = site_id.await;
    let start_date = jiff::civil::Date::new(2025, 11, 1)
        .map_err(|e| anyhow!("Failed to create start date: {e}"))?;
    let end_date = jiff::civil::Date::new(2025, 11, 3)
        .map_err(|e| anyhow!("Failed to create end date: {e}"))?;

    let usage_data = amber_client
        .usage()
        .site_id(&site_id)
        .start_date(start_date)
        .end_date(end_date)
        .call()
        .await?;

    assert!(!usage_data.is_empty(), "Expected non-empty usage data list");

    // Verify we have usage data spanning the requested date range
    let earliest_date = usage_data
        .iter()
        .map(|u| u.base.date)
        .min()
        .ok_or_else(|| anyhow!("Expected at least one usage entry"))?;
    let latest_date = usage_data
        .iter()
        .map(|u| u.base.date)
        .max()
        .ok_or_else(|| anyhow!("Expected at least one usage entry"))?;

    assert!(
        earliest_date >= start_date,
        "Earliest usage date should be >= start date"
    );
    assert!(
        latest_date <= end_date,
        "Latest usage date should be <= end date"
    );

    // Verify data integrity
    for usage in &usage_data {
        assert!(
            !usage.channel_identifier.is_empty(),
            "Channel identifier should not be empty"
        );
        assert!(usage.kwh >= 0.0_f64, "kWh should be non-negative");
        assert!(usage.cost.is_finite(), "Cost should be a finite number");
        assert!(
            usage.base.start_time < usage.base.end_time,
            "Start time should be before end time"
        );
    }

    Ok(())
}
