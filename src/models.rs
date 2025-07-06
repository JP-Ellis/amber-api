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
//! - [`SiteStatus`] - Status of sites (Pending, Active, Closed)
//!
//! ## Pricing Data
//!
//! - [`Interval`] - Electricity pricing intervals (Actual, Forecast, Current)
//! - [`BaseInterval`] - Common fields for all interval types
//! - [`PriceDescriptor`] - Price categories (extremely low, low, neutral, high,
//!   spike)
//! - [`SpikeStatus`] - Spike warning indicators
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
    /// Interval type
    #[serde(rename = "type")]
    pub interval_type: String,
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
    /// Renewable data type
    #[serde(rename = "type")]
    pub renewable_type: String,
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
