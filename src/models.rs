//! # Amber Electric API Models
//!
//! This module contains all the data structures and types used to interact with the
//! [Amber Electric Public API](https://api.amber.com.au/v1).
//!
//! ## Core Configuration
//!
//! - [`State`] - Australian states for renewable energy data (NSW, VIC, QLD,
//!   SA)
//! - [`Resolution`] - Interval resolution options (5-minute, 30-minute)
//!
//! ## Sites and Channels
//!
//! - [`Site`] - Information about electricity sites linked to your account
//! - [`Channel`] - Power meter channels (General, Controlled Load, Feed In)
//! - [`ChannelType`] - Types of meter channels
//! - [`SiteStatus`] - Status of sites (Pending, Active, Closed)
//!
//! ## Pricing Data
//!
//! - [`Interval`] - Electricity pricing intervals (Actual, Forecast, Current)
//! - [`BaseInterval`] - Common fields for all interval types
//! - [`ActualInterval`] - Confirmed historical pricing data
//! - [`ForecastInterval`] - Predicted future pricing data
//! - [`CurrentInterval`] - Real-time pricing data
//! - [`PriceDescriptor`] - Price categories (extremely low, low, neutral, high,
//!   spike)
//! - [`SpikeStatus`] - Spike warning indicators
//! - [`Range`] - Price ranges when volatile
//! - [`AdvancedPrice`] - Advanced price prediction with confidence bands
//!
//! ## Usage Data
//!
//! - [`Usage`] - Historical electricity consumption and generation data
//! - [`UsageQuality`] - Data quality indicators (Estimated, Billable)
//!
//! ## Renewable Energy
//!
//! - [`Renewable`] - Renewable energy data (Actual, Forecast, Current)
//! - [`BaseRenewable`] - Common fields for renewable data
//! - [`ActualRenewable`] - Confirmed historical renewable values
//! - [`ForecastRenewable`] - Predicted future renewable values
//! - [`CurrentRenewable`] - Real-time renewable values
//! - [`RenewableDescriptor`] - Renewable energy levels (best, great, ok,
//!   not great, worst)
//!
//! ## Tariff Information
//!
//! - [`TariffInformation`] - Time-of-use and demand tariff details
//! - [`TariffPeriod`] - Time periods (off peak, shoulder, solar sponge, peak)
//! - [`TariffSeason`] - Seasonal variations (Summer, Winter, etc.)
//!
//! ## Date and Time Handling
//!
//! All datetime fields use the [`jiff`] crate for robust datetime handling:
//! - [`jiff::civil::Date`] for date-only fields (ISO 8601 dates)
//! - [`jiff::Timestamp`] for datetime fields (ISO 8601 timestamps)

#![expect(
    deprecated,
    reason = "Defining deprecated variant for backwards compatibility"
)]

use core::fmt;

use jiff::{Timestamp, civil::Date};
use serde::Deserialize;

/// Valid Australian states for renewable energy data
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum State {
    /// New South Wales
    Nsw,
    /// Victoria
    Vic,
    /// Queensland
    Qld,
    /// South Australia
    Sa,
}

impl fmt::Display for State {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            State::Nsw => write!(f, "nsw"),
            State::Vic => write!(f, "vic"),
            State::Qld => write!(f, "qld"),
            State::Sa => write!(f, "sa"),
        }
    }
}

/// Valid interval resolution options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Resolution {
    /// 5-minute intervals
    FiveMinute = 5,
    /// 30-minute intervals
    ThirtyMinute = 30,
}

impl fmt::Display for Resolution {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Resolution::FiveMinute => write!(f, "5"),
            Resolution::ThirtyMinute => write!(f, "30"),
        }
    }
}

impl From<Resolution> for u32 {
    #[inline]
    fn from(value: Resolution) -> Self {
        match value {
            Resolution::FiveMinute => 5,
            Resolution::ThirtyMinute => 30,
        }
    }
}

/// Meter channel type
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub enum ChannelType {
    /// General channel provides continuous power - all of your appliances and
    /// lights are attached to this channel
    General,
    /// Controlled load channels are only on for a limited time during the day
    /// (usually when the load on the network is low, or generation is high) -
    /// you may have your hot water system attached to this channel
    ControlledLoad,
    /// Feed in channel sends power back to the grid - you will have these types
    /// of channels if you have solar or batteries
    FeedIn,
}

/// Describes a power meter channel.
///
/// The General channel provides continuous power - it's the channel all of your
/// appliances and lights are attached to.
///
/// Controlled loads are only on for a limited time during the day (usually when
/// the load on the network is low, or generation is high) - you may have your
/// hot water system attached to this channel.
///
/// The feed in channel sends power back to the grid - you will have these types
/// of channels if you have solar or batteries.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Channel {
    /// Identifier of the channel
    pub identifier: String,
    /// Channel type
    #[serde(rename = "type")]
    pub channel_type: ChannelType,
    /// The tariff code of the channel
    pub tariff: String,
}

/// Site status.
///
/// Pending sites are still in the process of being transferred.
///
/// Note: Amber only includes sites that have correct address details. If you
/// expect to see a site, but don't, you may need to contact info@amber.com.au
/// to check that the address is correct.
///
/// Active sites are ones that Amber actively supplies electricity to.
///
/// Closed sites are old sites that Amber no longer supplies.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub enum SiteStatus {
    /// Site is still in the process of being transferred.
    ///
    /// Note: Amber only includes sites that have correct address details. If
    /// you expect to see a site, but don't, you may need to contact
    /// info@amber.com.au to check that the address is correct.
    Pending,
    /// Site is actively supplied with electricity by Amber
    Active,
    /// Old site that Amber no longer supplies
    Closed,
}

/// Site information
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Site {
    /// Unique Site Identifier
    pub id: String,
    /// National Metering Identifier (NMI) for the site
    pub nmi: String,
    /// List of channels that are readable from your meter
    pub channels: Vec<Channel>,
    /// The name of the site's network
    pub network: String,
    /// Site status
    pub status: SiteStatus,
    /// Date the site became active. This date will be in the future for pending
    /// sites. It may also be undefined, though if it is, contact
    /// info@amber.com.au as there may be an issue with your address.
    pub active_from: Option<Date>,
    /// Date the site closed. Undefined if the site is pending or active.
    pub closed_on: Option<Date>,
    /// Length of interval that you will be billed on. 5 or 30 minutes.
    pub interval_length: u32,
}

/// Spike status
///
/// Indicates whether this interval will potentially spike, or is currently in a
/// spike state
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub enum SpikeStatus {
    /// No spike expected or occurring
    None,
    /// Spike may potentially occur during this interval
    Potential,
    /// Spike is currently occurring during this interval
    Spike,
}

/// Describes the current price.
///
/// Gives you an indication of how cheap the price is in relation to the average
/// VMO and DMO. Note: Negative is no longer used. It has been replaced with
/// extremelyLow.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub enum PriceDescriptor {
    /// Negative pricing (deprecated - replaced with `ExtremelyLow`)
    #[deprecated(note = "Negative pricing is no longer used. Use `ExtremelyLow` instead.")]
    Negative,
    /// Extremely low pricing - significant cost savings opportunity
    ExtremelyLow,
    /// Very low pricing - good cost savings opportunity
    VeryLow,
    /// Low pricing - some cost savings available
    Low,
    /// Neutral pricing - average market conditions
    Neutral,
    /// High pricing - costs above average
    High,
    /// Spike pricing - very high costs, avoid high usage
    Spike,
}

/// Describes the state of renewables.
///
/// Gives you an indication of how green power is right now
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub enum RenewableDescriptor {
    /// Best renewable conditions - highest percentage of green energy
    Best,
    /// Great renewable conditions - high percentage of green energy
    Great,
    /// Ok renewable conditions - moderate percentage of green energy
    Ok,
    /// Not great renewable conditions - low percentage of green energy
    NotGreat,
    /// Worst renewable conditions - lowest percentage of green energy
    Worst,
}

/// When prices are particularly volatile, the API may return a range of NEM
/// spot prices (c/kWh) that are possible.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Range {
    /// Estimated minimum price (c/kWh)
    pub min: f64,
    /// Estimated maximum price (c/kWh)
    pub max: f64,
}

/// Advanced price prediction
///
/// Amber has created an advanced forecast system, that represents Amber's
/// confidence in the AEMO forecast. The range indicates where Amber thinks the
/// price will land for a given interval.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AdvancedPrice {
    /// The lower bound of Amber's prediction band. Price includes network and
    /// market fees. (c/kWh).
    pub low: f64,
    /// The predicted price. Use this if you need a single number to forecast
    /// against. Price includes network and market fees. (c/kWh).
    pub predicted: f64,
    /// The upper bound of Amber's prediction band. Price includes network and
    /// market fees. (c/kWh).
    pub high: f64,
}

/// Information about how your tariff affects an interval
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct TariffInformation {
    /// The Time of Use period that is currently active.
    ///
    /// Only available if the site in on a time of use tariff
    pub period: Option<TariffPeriod>,
    /// The Time of Use season that is currently active.
    ///
    /// Only available if the site in on a time of use tariff
    pub season: Option<TariffSeason>,
    /// The block that is currently active.
    ///
    /// Only available in the site in on a block tariff
    pub block: Option<u32>,
    /// Is this interval currently in the demand window?
    ///
    /// Only available if the site in on a demand tariff
    pub demand_window: Option<bool>,
}

/// Time of Use period
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub enum TariffPeriod {
    /// Off-peak period with lowest electricity rates
    OffPeak,
    /// Shoulder period with moderate electricity rates
    Shoulder,
    /// Solar sponge period designed to encourage consumption when solar
    /// generation is high
    SolarSponge,
    /// Peak period with highest electricity rates
    Peak,
}

/// Time of Use season
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub enum TariffSeason {
    /// Default tariff season
    Default,
    /// Summer tariff season with typically higher rates due to increased demand
    Summer,
    /// Autumn tariff season with moderate rates
    Autumn,
    /// Winter tariff season with higher rates due to heating demand
    Winter,
    /// Spring tariff season with moderate rates
    Spring,
    /// Non-summer tariff season (autumn, winter, spring combined)
    NonSummer,
    /// Holiday tariff period with special rates
    Holiday,
    /// Weekend tariff period with typically lower rates
    Weekend,
    /// Combined weekend and holiday tariff period
    WeekendHoliday,
    /// Weekday tariff period with standard rates
    Weekday,
}

/// Base interval structure containing common fields
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct BaseInterval {
    /// Length of the interval in minutes.
    pub duration: u32,
    /// NEM spot price (c/kWh).
    ///
    /// This is the price generators get paid to generate electricity, and what
    /// drives the variable component of your perKwh price - includes GST
    pub spot_per_kwh: f64,
    /// Number of cents you will pay per kilowatt-hour (c/kWh) - includes GST
    pub per_kwh: f64,
    /// Date the interval belongs to (in NEM time).
    ///
    /// This may be different to the date component of nemTime, as the last
    /// interval of the day ends at 12:00 the following day.
    pub date: Date,
    /// The interval's NEM time.
    ///
    /// This represents the time at the end of the interval UTC+10.
    pub nem_time: Timestamp,
    /// Start time of the interval in UTC.
    pub start_time: Timestamp,
    /// End time of the interval in UTC.
    pub end_time: Timestamp,
    /// Percentage of renewables in the grid
    pub renewables: f64,
    /// Channel type
    pub channel_type: ChannelType,
    /// Tariff information
    pub tariff_information: Option<TariffInformation>,
    /// Spike status
    pub spike_status: SpikeStatus,
    /// Price descriptor
    pub descriptor: PriceDescriptor,
}

/// Actual interval with confirmed pricing
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ActualInterval {
    /// Base interval data with confirmed pricing
    #[serde(flatten)]
    pub base: BaseInterval,
}

/// Forecast interval with predicted pricing
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ForecastInterval {
    /// Base interval data with predicted pricing
    #[serde(flatten)]
    pub base: BaseInterval,
    /// Price range when volatile
    pub range: Option<Range>,
    /// Advanced price prediction
    pub advanced_price: Option<AdvancedPrice>,
}

/// Current interval with real-time pricing
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct CurrentInterval {
    /// Base interval data with real-time pricing
    #[serde(flatten)]
    pub base: BaseInterval,
    /// Price range when volatile
    pub range: Option<Range>,
    /// Shows true the current price is an estimate. Shows false is the price
    /// has been locked in.
    pub estimate: bool,
    /// Advanced price prediction
    pub advanced_price: Option<AdvancedPrice>,
}

/// Interval enum that can be any of the interval types
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum Interval {
    /// Actual interval with confirmed historical pricing data
    ActualInterval(ActualInterval),
    /// Forecast interval with predicted future pricing data
    ForecastInterval(ForecastInterval),
    /// Current interval with real-time pricing data
    CurrentInterval(CurrentInterval),
}

/// Usage data for a specific interval
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Usage {
    /// Base interval data for usage reporting
    #[serde(flatten)]
    pub base: BaseInterval,
    /// Meter channel identifier
    pub channel_identifier: String,
    /// Number of kWh you consumed or generated.
    ///
    /// Generated numbers will be negative
    pub kwh: f64,
    /// Data quality indicator
    pub quality: UsageQuality,
    /// The total cost of your consumption or generation for this period -
    /// includes GST
    pub cost: f64,
}

/// Usage data quality
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub enum UsageQuality {
    /// Estimated by the metering company
    Estimated,
    /// Actual billable data
    Billable,
}

/// Base renewable data structure
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct BaseRenewable {
    /// Length of the interval in minutes.
    pub duration: u32,
    /// Date the interval belongs to (in NEM time).
    ///
    /// This may be different to the date component of nemTime, as the last
    /// interval of the day ends at 12:00 the following day.
    pub date: Date,
    /// The interval's NEM time.
    ///
    /// This represents the time at the end of the interval UTC+10.
    pub nem_time: Timestamp,
    /// Start time of the interval in UTC.
    pub start_time: Timestamp,
    /// End time of the interval in UTC.
    pub end_time: Timestamp,
    /// Percentage of renewables in the grid
    pub renewables: f64,
    /// Renewable descriptor
    pub descriptor: RenewableDescriptor,
}

/// Actual renewable data
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ActualRenewable {
    /// Base renewable data with confirmed historical values
    #[serde(flatten)]
    pub base: BaseRenewable,
}

/// Forecast renewable data
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ForecastRenewable {
    /// Base renewable data with predicted future values
    #[serde(flatten)]
    pub base: BaseRenewable,
}

/// Current renewable data
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct CurrentRenewable {
    /// Base renewable data with current real-time values
    #[serde(flatten)]
    pub base: BaseRenewable,
}

/// Renewable enum that can be any of the renewable types
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum Renewable {
    /// Actual renewable data with confirmed historical values
    ActualRenewable(ActualRenewable),
    /// Forecast renewable data with predicted future values
    ForecastRenewable(ForecastRenewable),
    /// Current renewable data with real-time values
    CurrentRenewable(CurrentRenewable),
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use pretty_assertions::assert_eq;

    #[test]
    fn actual_renewable_deserialisation_strict() -> Result<()> {
        let json = r#"{
            "type": "ActualRenewable",
            "duration": 5,
            "date": "2021-05-05",
            "nemTime": "2021-05-06T12:30:00+10:00",
            "startTime": "2021-05-05T02:00:01Z",
            "endTime": "2021-05-05T02:30:00Z",
            "renewables": 45,
            "descriptor": "best"
        }"#;

        let actual: ActualRenewable = serde_json::from_str(json)?;
        assert_eq!(actual.base.duration, 5);
        assert_eq!(actual.base.date.to_string(), "2021-05-05");
        assert!(44.0_f64 < actual.base.renewables && actual.base.renewables < 46.0_f64);
        assert_eq!(actual.base.descriptor, RenewableDescriptor::Best);

        Ok(())
    }

    #[test]
    fn actual_renewable_deserialisation() -> Result<()> {
        let json = r#"{
            "type": "ActualRenewable",
            "duration": 5,
            "date": "2021-05-05",
            "nemTime": "2021-05-06T12:30:00+10:00",
            "startTime": "2021-05-05T02:00:01Z",
            "endTime": "2021-05-05T02:30:00Z",
            "renewables": 45,
            "descriptor": "best"
        }"#;

        let renewable: Renewable = serde_json::from_str(json)?;
        if let Renewable::ActualRenewable(actual) = renewable {
            assert_eq!(actual.base.duration, 5);
            assert_eq!(actual.base.date.to_string(), "2021-05-05");
            assert!(44.0_f64 < actual.base.renewables && actual.base.renewables < 46.0_f64);
            assert_eq!(actual.base.descriptor, RenewableDescriptor::Best);
        } else {
            panic!("Expected ActualRenewable variant");
        }

        Ok(())
    }

    #[test]
    fn current_renewable_deserialisation_strict() -> Result<()> {
        let json = r#"{
            "type": "CurrentRenewable",
            "duration": 5,
            "date": "2021-05-05",
            "nemTime": "2021-05-06T12:30:00+10:00",
            "startTime": "2021-05-05T02:00:01Z",
            "endTime": "2021-05-05T02:30:00Z",
            "renewables": 45,
            "descriptor": "best"
        }"#;

        let current: CurrentRenewable = serde_json::from_str(json)?;
        assert_eq!(current.base.duration, 5);
        assert_eq!(current.base.date.to_string(), "2021-05-05");
        assert!(44.0_f64 < current.base.renewables && current.base.renewables < 46.0_f64);
        assert_eq!(current.base.descriptor, RenewableDescriptor::Best);

        Ok(())
    }

    #[test]
    fn current_renewable_deserialisation() -> Result<()> {
        let json = r#"{
            "type": "CurrentRenewable",
            "duration": 5,
            "date": "2021-05-05",
            "nemTime": "2021-05-06T12:30:00+10:00",
            "startTime": "2021-05-05T02:00:01Z",
            "endTime": "2021-05-05T02:30:00Z",
            "renewables": 45,
            "descriptor": "best"
        }"#;

        let renewable: Renewable = serde_json::from_str(json)?;
        if let Renewable::CurrentRenewable(current) = renewable {
            assert_eq!(current.base.duration, 5);
            assert_eq!(current.base.date.to_string(), "2021-05-05");
            assert!(44.0_f64 < current.base.renewables && current.base.renewables < 46.0_f64);
            assert_eq!(current.base.descriptor, RenewableDescriptor::Best);
        } else {
            panic!("Expected CurrentRenewable variant");
        }

        Ok(())
    }

    #[test]
    fn forecast_renewable_deserialisation_strict() -> Result<()> {
        let json = r#"{
            "type": "ForecastRenewable",
            "duration": 5,
            "date": "2021-05-05",
            "nemTime": "2021-05-06T12:30:00+10:00",
            "startTime": "2021-05-05T02:00:01Z",
            "endTime": "2021-05-05T02:30:00Z",
            "renewables": 45,
            "descriptor": "best"
        }"#;

        let forecast: ForecastRenewable = serde_json::from_str(json)?;
        assert_eq!(forecast.base.duration, 5);
        assert_eq!(forecast.base.date.to_string(), "2021-05-05");
        assert!(44.0_f64 < forecast.base.renewables && forecast.base.renewables < 46.0_f64);
        assert_eq!(forecast.base.descriptor, RenewableDescriptor::Best);

        Ok(())
    }

    #[test]
    fn forecast_renewable_deserialisation() -> Result<()> {
        let json = r#"{
            "type": "ForecastRenewable",
            "duration": 5,
            "date": "2021-05-05",
            "nemTime": "2021-05-06T12:30:00+10:00",
            "startTime": "2021-05-05T02:00:01Z",
            "endTime": "2021-05-05T02:30:00Z",
            "renewables": 45,
            "descriptor": "best"
        }"#;

        let renewable: Renewable = serde_json::from_str(json)?;
        if let Renewable::ForecastRenewable(forecast) = renewable {
            assert_eq!(forecast.base.duration, 5);
            assert_eq!(forecast.base.date.to_string(), "2021-05-05");
            assert!(44.0_f64 < forecast.base.renewables && forecast.base.renewables < 46.0_f64);
            assert_eq!(forecast.base.descriptor, RenewableDescriptor::Best);
        } else {
            panic!("Expected ForecastRenewable variant");
        }

        Ok(())
    }

    // Test Site deserialization
    #[test]
    fn site_deserialisation() -> Result<()> {
        let json = r#"[
            {
                "id": "01F5A5CRKMZ5BCX9P1S4V990AM",
                "nmi": "3052282872",
                "channels": [
                    {
                        "identifier": "E1",
                        "type": "general",
                        "tariff": "A100"
                    }
                ],
                "network": "Jemena",
                "status": "closed",
                "activeFrom": "2022-01-01",
                "closedOn": "2022-05-01",
                "intervalLength": 30
            }
        ]"#;

        let sites: Vec<Site> = serde_json::from_str(json)?;
        assert_eq!(sites.len(), 1);

        let site = sites.first().expect("Expected at least one site");
        assert_eq!(site.id, "01F5A5CRKMZ5BCX9P1S4V990AM");
        assert_eq!(site.nmi, "3052282872");
        assert_eq!(site.channels.len(), 1);

        let channel = site
            .channels
            .first()
            .expect("Expected at least one channel");
        assert_eq!(channel.identifier, "E1");
        assert_eq!(channel.channel_type, ChannelType::General);
        assert_eq!(channel.tariff, "A100");

        assert_eq!(site.network, "Jemena");
        assert_eq!(site.status, SiteStatus::Closed);
        assert_eq!(
            site.active_from
                .expect("Expected active_from date")
                .to_string(),
            "2022-01-01"
        );
        assert_eq!(
            site.closed_on.expect("Expected closed_on date").to_string(),
            "2022-05-01"
        );
        assert_eq!(site.interval_length, 30);

        Ok(())
    }

    // Test Interval deserialization (prices endpoint)
    #[test]
    #[expect(
        clippy::too_many_lines,
        reason = "Comprehensive test for all interval types"
    )]
    fn prices_interval_deserialisation() -> Result<()> {
        let json = r#"[
            {
                "type": "ActualInterval",
                "duration": 5,
                "spotPerKwh": 6.12,
                "perKwh": 24.33,
                "date": "2021-05-05",
                "nemTime": "2021-05-06T12:30:00+10:00",
                "startTime": "2021-05-05T02:00:01Z",
                "endTime": "2021-05-05T02:30:00Z",
                "renewables": 45,
                "channelType": "general",
                "tariffInformation": null,
                "spikeStatus": "none",
                "descriptor": "negative"
            },
            {
                "type": "CurrentInterval",
                "duration": 5,
                "spotPerKwh": 6.12,
                "perKwh": 24.33,
                "date": "2021-05-05",
                "nemTime": "2021-05-06T12:30:00+10:00",
                "startTime": "2021-05-05T02:00:01Z",
                "endTime": "2021-05-05T02:30:00Z",
                "renewables": 45,
                "channelType": "general",
                "tariffInformation": null,
                "spikeStatus": "none",
                "descriptor": "negative",
                "range": {
                    "min": 0,
                    "max": 0
                },
                "estimate": true,
                "advancedPrice": {
                    "low": 1,
                    "predicted": 3,
                    "high": 10
                }
            },
            {
                "type": "ForecastInterval",
                "duration": 5,
                "spotPerKwh": 6.12,
                "perKwh": 24.33,
                "date": "2021-05-05",
                "nemTime": "2021-05-06T12:30:00+10:00",
                "startTime": "2021-05-05T02:00:01Z",
                "endTime": "2021-05-05T02:30:00Z",
                "renewables": 45,
                "channelType": "general",
                "tariffInformation": null,
                "spikeStatus": "none",
                "descriptor": "negative",
                "range": {
                    "min": 0,
                    "max": 0
                },
                "advancedPrice": {
                    "low": 1,
                    "predicted": 3,
                    "high": 10
                }
            }
        ]"#;

        let intervals: Vec<Interval> = serde_json::from_str(json)?;
        assert_eq!(intervals.len(), 3);

        // Test ActualInterval
        if let Some(Interval::ActualInterval(actual)) = intervals.first() {
            assert_eq!(actual.base.duration, 5);
            assert!((actual.base.spot_per_kwh - 6.12_f64).abs() < f64::EPSILON);
            assert!((actual.base.per_kwh - 24.33_f64).abs() < f64::EPSILON);
            assert_eq!(actual.base.date.to_string(), "2021-05-05");
            assert!((actual.base.renewables - 45.0_f64).abs() < f64::EPSILON);
            assert_eq!(actual.base.channel_type, ChannelType::General);
            assert_eq!(actual.base.spike_status, SpikeStatus::None);
            assert_eq!(actual.base.descriptor, PriceDescriptor::Negative);
        } else {
            panic!("Expected ActualInterval at index 0");
        }

        // Test CurrentInterval
        if let Some(Interval::CurrentInterval(current)) = intervals.get(1) {
            assert_eq!(current.base.duration, 5);
            assert!((current.base.spot_per_kwh - 6.12_f64).abs() < f64::EPSILON);
            assert!((current.base.per_kwh - 24.33_f64).abs() < f64::EPSILON);
            assert_eq!(current.estimate, true);
            assert!(current.range.is_some());
            assert!(current.advanced_price.is_some());

            if let Some(ref range) = current.range {
                assert!((range.min - 0.0_f64).abs() < f64::EPSILON);
                assert!((range.max - 0.0_f64).abs() < f64::EPSILON);
            }

            if let Some(ref adv_price) = current.advanced_price {
                assert!((adv_price.low - 1.0_f64).abs() < f64::EPSILON);
                assert!((adv_price.predicted - 3.0_f64).abs() < f64::EPSILON);
                assert!((adv_price.high - 10.0_f64).abs() < f64::EPSILON);
            }
        } else {
            panic!("Expected CurrentInterval at index 1");
        }

        // Test ForecastInterval
        if let Some(Interval::ForecastInterval(forecast)) = intervals.get(2) {
            assert_eq!(forecast.base.duration, 5);
            assert!((forecast.base.spot_per_kwh - 6.12_f64).abs() < f64::EPSILON);
            assert!((forecast.base.per_kwh - 24.33_f64).abs() < f64::EPSILON);
            assert!(forecast.range.is_some());
            assert!(forecast.advanced_price.is_some());
        } else {
            panic!("Expected ForecastInterval at index 2");
        }

        Ok(())
    }

    // Test Current Prices endpoint (same as prices but for current endpoint)
    #[test]
    fn current_prices_interval_deserialisation() -> Result<()> {
        let json = r#"[
            {
                "type": "ActualInterval",
                "duration": 5,
                "spotPerKwh": 6.12,
                "perKwh": 24.33,
                "date": "2021-05-05",
                "nemTime": "2021-05-06T12:30:00+10:00",
                "startTime": "2021-05-05T02:00:01Z",
                "endTime": "2021-05-05T02:30:00Z",
                "renewables": 45,
                "channelType": "general",
                "tariffInformation": null,
                "spikeStatus": "none",
                "descriptor": "negative"
            },
            {
                "type": "CurrentInterval",
                "duration": 5,
                "spotPerKwh": 6.12,
                "perKwh": 24.33,
                "date": "2021-05-05",
                "nemTime": "2021-05-06T12:30:00+10:00",
                "startTime": "2021-05-05T02:00:01Z",
                "endTime": "2021-05-05T02:30:00Z",
                "renewables": 45,
                "channelType": "general",
                "tariffInformation": null,
                "spikeStatus": "none",
                "descriptor": "negative",
                "range": {
                    "min": 0,
                    "max": 0
                },
                "estimate": true,
                "advancedPrice": {
                    "low": 1,
                    "predicted": 3,
                    "high": 10
                }
            },
            {
                "type": "ForecastInterval",
                "duration": 5,
                "spotPerKwh": 6.12,
                "perKwh": 24.33,
                "date": "2021-05-05",
                "nemTime": "2021-05-06T12:30:00+10:00",
                "startTime": "2021-05-05T02:00:01Z",
                "endTime": "2021-05-05T02:30:00Z",
                "renewables": 45,
                "channelType": "general",
                "tariffInformation": null,
                "spikeStatus": "none",
                "descriptor": "negative",
                "range": {
                    "min": 0,
                    "max": 0
                },
                "advancedPrice": {
                    "low": 1,
                    "predicted": 3,
                    "high": 10
                }
            }
        ]"#;

        let intervals: Vec<Interval> = serde_json::from_str(json)?;
        assert_eq!(intervals.len(), 3);

        // Verify we can deserialize all three types in the current prices endpoint
        let first_interval = intervals.first().expect("Expected at least one interval");
        let second_interval = intervals.get(1).expect("Expected at least two intervals");
        let third_interval = intervals.get(2).expect("Expected at least three intervals");

        assert!(matches!(first_interval, Interval::ActualInterval(_)));
        assert!(matches!(second_interval, Interval::CurrentInterval(_)));
        assert!(matches!(third_interval, Interval::ForecastInterval(_)));

        Ok(())
    }

    // Test Usage deserialization
    #[test]
    fn usage_deserialisation() -> Result<()> {
        let json = r#"[
            {
                "type": "Usage",
                "duration": 5,
                "spotPerKwh": 6.12,
                "perKwh": 24.33,
                "date": "2021-05-05",
                "nemTime": "2021-05-06T12:30:00+10:00",
                "startTime": "2021-05-05T02:00:01Z",
                "endTime": "2021-05-05T02:30:00Z",
                "renewables": 45,
                "channelType": "general",
                "tariffInformation": null,
                "spikeStatus": "none",
                "descriptor": "negative",
                "channelIdentifier": "E1",
                "kwh": 0,
                "quality": "estimated",
                "cost": 0
            }
        ]"#;

        let usage_data: Vec<Usage> = serde_json::from_str(json)?;
        assert_eq!(usage_data.len(), 1);

        let usage = usage_data
            .first()
            .expect("Expected at least one usage entry");
        assert_eq!(usage.base.duration, 5);
        assert!((usage.base.spot_per_kwh - 6.12_f64).abs() < f64::EPSILON);
        assert!((usage.base.per_kwh - 24.33_f64).abs() < f64::EPSILON);
        assert_eq!(usage.base.date.to_string(), "2021-05-05");
        assert!((usage.base.renewables - 45.0_f64).abs() < f64::EPSILON);
        assert_eq!(usage.base.channel_type, ChannelType::General);
        assert_eq!(usage.base.spike_status, SpikeStatus::None);
        assert_eq!(usage.base.descriptor, PriceDescriptor::Negative);
        assert_eq!(usage.channel_identifier, "E1");
        assert!((usage.kwh - 0.0_f64).abs() < f64::EPSILON);
        assert_eq!(usage.quality, UsageQuality::Estimated);
        assert!((usage.cost - 0.0_f64).abs() < f64::EPSILON);

        Ok(())
    }

    // Test individual components with edge cases
    #[test]
    fn channel_types_deserialisation() -> Result<()> {
        // Test all channel types
        let general_json = r#"{"identifier": "E1", "type": "general", "tariff": "A100"}"#;
        let controlled_json = r#"{"identifier": "E2", "type": "controlledLoad", "tariff": "A200"}"#;
        let feedin_json = r#"{"identifier": "E3", "type": "feedIn", "tariff": "A300"}"#;

        let general: Channel = serde_json::from_str(general_json)?;
        let controlled: Channel = serde_json::from_str(controlled_json)?;
        let feedin: Channel = serde_json::from_str(feedin_json)?;

        assert_eq!(general.channel_type, ChannelType::General);
        assert_eq!(controlled.channel_type, ChannelType::ControlledLoad);
        assert_eq!(feedin.channel_type, ChannelType::FeedIn);

        Ok(())
    }

    #[test]
    fn site_status_deserialisation() -> Result<()> {
        #[derive(Deserialize)]
        struct TestSiteStatus {
            status: SiteStatus,
        }

        // Test all site statuses
        let pending_json = r#"{"status": "pending"}"#;
        let active_json = r#"{"status": "active"}"#;
        let closed_json = r#"{"status": "closed"}"#;

        let pending: TestSiteStatus = serde_json::from_str(pending_json)?;
        let active: TestSiteStatus = serde_json::from_str(active_json)?;
        let closed: TestSiteStatus = serde_json::from_str(closed_json)?;

        assert_eq!(pending.status, SiteStatus::Pending);
        assert_eq!(active.status, SiteStatus::Active);
        assert_eq!(closed.status, SiteStatus::Closed);

        Ok(())
    }

    #[test]
    fn range_and_advanced_price_deserialisation() -> Result<()> {
        let range_json = r#"{"min": 0, "max": 100}"#;
        let advanced_price_json = r#"{"low": 1, "predicted": 3, "high": 10}"#;

        let range: Range = serde_json::from_str(range_json)?;
        let advanced_price: AdvancedPrice = serde_json::from_str(advanced_price_json)?;

        assert!((range.min - 0.0_f64).abs() < f64::EPSILON);
        assert!((range.max - 100.0_f64).abs() < f64::EPSILON);
        assert!((advanced_price.low - 1.0_f64).abs() < f64::EPSILON);
        assert!((advanced_price.predicted - 3.0_f64).abs() < f64::EPSILON);
        assert!((advanced_price.high - 10.0_f64).abs() < f64::EPSILON);

        Ok(())
    }

    #[test]
    fn usage_quality_deserialisation() -> Result<()> {
        #[derive(Deserialize)]
        struct TestUsageQuality {
            quality: UsageQuality,
        }

        let estimated_json = r#"{"quality": "estimated"}"#;
        let billable_json = r#"{"quality": "billable"}"#;

        let estimated: TestUsageQuality = serde_json::from_str(estimated_json)?;
        let billable: TestUsageQuality = serde_json::from_str(billable_json)?;

        assert_eq!(estimated.quality, UsageQuality::Estimated);
        assert_eq!(billable.quality, UsageQuality::Billable);

        Ok(())
    }
}
