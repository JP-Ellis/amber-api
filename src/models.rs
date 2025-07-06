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

impl fmt::Display for ChannelType {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChannelType::General => write!(f, "general"),
            ChannelType::ControlledLoad => write!(f, "controlled load"),
            ChannelType::FeedIn => write!(f, "feed-in"),
        }
    }
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

impl fmt::Display for Channel {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({}): {}",
            self.identifier, self.channel_type, self.tariff
        )
    }
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

impl fmt::Display for SiteStatus {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SiteStatus::Pending => write!(f, "pending"),
            SiteStatus::Active => write!(f, "active"),
            SiteStatus::Closed => write!(f, "closed"),
        }
    }
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

impl fmt::Display for Site {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Site {} (NMI: {}) - {} on {} network",
            self.id, self.nmi, self.status, self.network
        )
    }
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

impl fmt::Display for SpikeStatus {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SpikeStatus::None => write!(f, "none"),
            SpikeStatus::Potential => write!(f, "potential"),
            SpikeStatus::Spike => write!(f, "spike"),
        }
    }
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

impl fmt::Display for PriceDescriptor {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PriceDescriptor::Negative => write!(f, "negative"),
            PriceDescriptor::ExtremelyLow => write!(f, "extremely low"),
            PriceDescriptor::VeryLow => write!(f, "very low"),
            PriceDescriptor::Low => write!(f, "low"),
            PriceDescriptor::Neutral => write!(f, "neutral"),
            PriceDescriptor::High => write!(f, "high"),
            PriceDescriptor::Spike => write!(f, "spike"),
        }
    }
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

impl fmt::Display for RenewableDescriptor {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RenewableDescriptor::Best => write!(f, "best"),
            RenewableDescriptor::Great => write!(f, "great"),
            RenewableDescriptor::Ok => write!(f, "ok"),
            RenewableDescriptor::NotGreat => write!(f, "not great"),
            RenewableDescriptor::Worst => write!(f, "worst"),
        }
    }
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

impl fmt::Display for Range {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2}-{:.2}c/kWh", self.min, self.max)
    }
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

impl fmt::Display for AdvancedPrice {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "L:{:.2} H:{:.2} P:{:.2} c/kWh",
            self.low, self.predicted, self.high
        )
    }
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

impl fmt::Display for TariffInformation {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut parts = Vec::new();

        if let Some(ref period) = self.period {
            parts.push(format!("period:{period}"));
        }
        if let Some(ref season) = self.season {
            parts.push(format!("season:{season}"));
        }
        if let Some(block) = self.block {
            parts.push(format!("block:{block}"));
        }
        if let Some(demand_window) = self.demand_window {
            parts.push(format!("demand window:{demand_window}"));
        }

        if parts.is_empty() {
            write!(f, "No tariff information")
        } else {
            write!(f, "{}", parts.join(", "))
        }
    }
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

impl fmt::Display for TariffPeriod {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TariffPeriod::OffPeak => write!(f, "off peak"),
            TariffPeriod::Shoulder => write!(f, "shoulder"),
            TariffPeriod::SolarSponge => write!(f, "solar sponge"),
            TariffPeriod::Peak => write!(f, "peak"),
        }
    }
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

impl fmt::Display for TariffSeason {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TariffSeason::Default => write!(f, "default"),
            TariffSeason::Summer => write!(f, "summer"),
            TariffSeason::Autumn => write!(f, "autumn"),
            TariffSeason::Winter => write!(f, "winter"),
            TariffSeason::Spring => write!(f, "spring"),
            TariffSeason::NonSummer => write!(f, "non summer"),
            TariffSeason::Holiday => write!(f, "holiday"),
            TariffSeason::Weekend => write!(f, "weekend"),
            TariffSeason::WeekendHoliday => write!(f, "weekend holiday"),
            TariffSeason::Weekday => write!(f, "weekday"),
        }
    }
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

impl fmt::Display for BaseInterval {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {:.2}c/kWh (spot: {:.2}c/kWh) ({}) {}% renewable",
            self.date,
            self.channel_type,
            self.per_kwh,
            self.spot_per_kwh,
            self.descriptor,
            self.renewables
        )?;

        if self.spike_status != SpikeStatus::None {
            write!(f, " spike: {}", self.spike_status)?;
        }

        if let Some(ref tariff) = self.tariff_information {
            write!(f, " [{tariff}]")?;
        }

        Ok(())
    }
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

impl fmt::Display for ActualInterval {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Actual: {}", self.base)
    }
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

impl fmt::Display for ForecastInterval {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Forecast: {}", self.base)?;
        if let Some(ref range) = self.range {
            write!(f, " Range: {range}")?;
        }
        if let Some(ref adv_price) = self.advanced_price {
            write!(f, " Advanced: {adv_price}")?;
        }
        Ok(())
    }
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

impl fmt::Display for CurrentInterval {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Current: {}", self.base)?;
        if self.estimate {
            write!(f, " (estimate)")?;
        }
        if let Some(ref range) = self.range {
            write!(f, " Range: {range}")?;
        }
        if let Some(ref adv_price) = self.advanced_price {
            write!(f, " Advanced: {adv_price}")?;
        }
        Ok(())
    }
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

impl fmt::Display for Interval {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Interval::ActualInterval(actual) => write!(f, "{actual}"),
            Interval::ForecastInterval(forecast) => write!(f, "{forecast}"),
            Interval::CurrentInterval(current) => write!(f, "{current}"),
        }
    }
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

impl fmt::Display for Usage {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Usage {} {:.2}kWh ${:.2} ({})",
            self.channel_identifier, self.kwh, self.cost, self.quality
        )
    }
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

impl fmt::Display for UsageQuality {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UsageQuality::Estimated => write!(f, "estimated"),
            UsageQuality::Billable => write!(f, "billable"),
        }
    }
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

impl fmt::Display for BaseRenewable {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}% renewable ({})",
            self.date, self.renewables, self.descriptor
        )
    }
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

impl fmt::Display for ActualRenewable {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Actual: {}", self.base)
    }
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

impl fmt::Display for ForecastRenewable {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Forecast: {}", self.base)
    }
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

impl fmt::Display for CurrentRenewable {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Current: {}", self.base)
    }
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

impl fmt::Display for Renewable {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Renewable::ActualRenewable(actual) => write!(f, "{actual}"),
            Renewable::ForecastRenewable(forecast) => write!(f, "{forecast}"),
            Renewable::CurrentRenewable(current) => write!(f, "{current}"),
        }
    }
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

    // Display trait tests using insta snapshots
    #[test]
    fn display_state() {
        insta::assert_snapshot!(State::Nsw.to_string(), @"nsw");
        insta::assert_snapshot!(State::Vic.to_string(), @"vic");
        insta::assert_snapshot!(State::Qld.to_string(), @"qld");
        insta::assert_snapshot!(State::Sa.to_string(), @"sa");
    }

    #[test]
    fn display_resolution() {
        insta::assert_snapshot!(Resolution::FiveMinute.to_string(), @"5");
        insta::assert_snapshot!(Resolution::ThirtyMinute.to_string(), @"30");
    }

    #[test]
    fn display_channel_type() {
        insta::assert_snapshot!(ChannelType::General.to_string(), @"general");
        insta::assert_snapshot!(ChannelType::ControlledLoad.to_string(), @"controlled load");
        insta::assert_snapshot!(ChannelType::FeedIn.to_string(), @"feed-in");
    }

    #[test]
    fn display_channel() {
        let channel = Channel {
            identifier: "E1".to_owned(),
            channel_type: ChannelType::General,
            tariff: "A100".to_owned(),
        };
        insta::assert_snapshot!(channel.to_string(), @"E1 (general): A100");
    }

    #[test]
    fn display_site_status() {
        insta::assert_snapshot!(SiteStatus::Pending.to_string(), @"pending");
        insta::assert_snapshot!(SiteStatus::Active.to_string(), @"active");
        insta::assert_snapshot!(SiteStatus::Closed.to_string(), @"closed");
    }

    #[test]
    fn display_site() {
        use jiff::civil::Date;
        let site = Site {
            id: "01F5A5CRKMZ5BCX9P1S4V990AM".to_owned(),
            nmi: "3052282872".to_owned(),
            channels: vec![],
            network: "Jemena".to_owned(),
            status: SiteStatus::Active,
            active_from: Some(Date::constant(2022, 1, 1)),
            closed_on: None,
            interval_length: 30,
        };
        insta::assert_snapshot!(site.to_string(), @"Site 01F5A5CRKMZ5BCX9P1S4V990AM (NMI: 3052282872) - active on Jemena network");
    }

    #[test]
    fn display_spike_status() {
        insta::assert_snapshot!(SpikeStatus::None.to_string(), @"none");
        insta::assert_snapshot!(SpikeStatus::Potential.to_string(), @"potential");
        insta::assert_snapshot!(SpikeStatus::Spike.to_string(), @"spike");
    }

    #[test]
    fn display_price_descriptor() {
        insta::assert_snapshot!(PriceDescriptor::Negative.to_string(), @"negative");
        insta::assert_snapshot!(PriceDescriptor::ExtremelyLow.to_string(), @"extremely low");
        insta::assert_snapshot!(PriceDescriptor::VeryLow.to_string(), @"very low");
        insta::assert_snapshot!(PriceDescriptor::Low.to_string(), @"low");
        insta::assert_snapshot!(PriceDescriptor::Neutral.to_string(), @"neutral");
        insta::assert_snapshot!(PriceDescriptor::High.to_string(), @"high");
        insta::assert_snapshot!(PriceDescriptor::Spike.to_string(), @"spike");
    }

    #[test]
    fn display_renewable_descriptor() {
        insta::assert_snapshot!(RenewableDescriptor::Best.to_string(), @"best");
        insta::assert_snapshot!(RenewableDescriptor::Great.to_string(), @"great");
        insta::assert_snapshot!(RenewableDescriptor::Ok.to_string(), @"ok");
        insta::assert_snapshot!(RenewableDescriptor::NotGreat.to_string(), @"not great");
        insta::assert_snapshot!(RenewableDescriptor::Worst.to_string(), @"worst");
    }

    #[test]
    fn display_range() {
        let range = Range {
            min: 12.34,
            max: 56.78,
        };
        insta::assert_snapshot!(range.to_string(), @"12.34-56.78c/kWh");
    }

    #[test]
    fn display_advanced_price() {
        let advanced_price = AdvancedPrice {
            low: 1.23,
            predicted: 4.56,
            high: 7.89,
        };
        insta::assert_snapshot!(advanced_price.to_string(), @"L:1.23 H:4.56 P:7.89 c/kWh");
    }

    #[test]
    fn display_tariff_period() {
        insta::assert_snapshot!(TariffPeriod::OffPeak.to_string(), @"off peak");
        insta::assert_snapshot!(TariffPeriod::Shoulder.to_string(), @"shoulder");
        insta::assert_snapshot!(TariffPeriod::SolarSponge.to_string(), @"solar sponge");
        insta::assert_snapshot!(TariffPeriod::Peak.to_string(), @"peak");
    }

    #[test]
    fn display_tariff_season() {
        insta::assert_snapshot!(TariffSeason::Default.to_string(), @"default");
        insta::assert_snapshot!(TariffSeason::Summer.to_string(), @"summer");
        insta::assert_snapshot!(TariffSeason::Autumn.to_string(), @"autumn");
        insta::assert_snapshot!(TariffSeason::Winter.to_string(), @"winter");
        insta::assert_snapshot!(TariffSeason::Spring.to_string(), @"spring");
        insta::assert_snapshot!(TariffSeason::NonSummer.to_string(), @"non summer");
        insta::assert_snapshot!(TariffSeason::Holiday.to_string(), @"holiday");
        insta::assert_snapshot!(TariffSeason::Weekend.to_string(), @"weekend");
        insta::assert_snapshot!(TariffSeason::WeekendHoliday.to_string(), @"weekend holiday");
        insta::assert_snapshot!(TariffSeason::Weekday.to_string(), @"weekday");
    }

    #[test]
    fn display_tariff_information() {
        // Test with no information
        let empty_tariff = TariffInformation {
            period: None,
            season: None,
            block: None,
            demand_window: None,
        };
        insta::assert_snapshot!(empty_tariff.to_string(), @"No tariff information");

        // Test with all information
        let full_tariff = TariffInformation {
            period: Some(TariffPeriod::Peak),
            season: Some(TariffSeason::Summer),
            block: Some(2),
            demand_window: Some(true),
        };
        insta::assert_snapshot!(full_tariff.to_string(), @"period:peak, season:summer, block:2, demand window:true");

        // Test with partial information
        let partial_tariff = TariffInformation {
            period: Some(TariffPeriod::OffPeak),
            season: None,
            block: Some(1),
            demand_window: Some(false),
        };
        insta::assert_snapshot!(partial_tariff.to_string(), @"period:off peak, block:1, demand window:false");
    }

    #[test]
    fn display_base_interval() {
        use jiff::{Timestamp, civil::Date};
        // Use parse instead of constant for complex timestamps
        let nem_time = "2021-05-06T12:30:00+10:00"
            .parse::<Timestamp>()
            .expect("valid timestamp");
        let start_time = "2021-05-05T02:00:01Z"
            .parse::<Timestamp>()
            .expect("valid timestamp");
        let end_time = "2021-05-05T02:30:00Z"
            .parse::<Timestamp>()
            .expect("valid timestamp");

        // Test basic case with no spike status and no tariff information
        let base_interval_basic = BaseInterval {
            duration: 5,
            spot_per_kwh: 6.12,
            per_kwh: 24.33,
            date: Date::constant(2021, 5, 5),
            nem_time,
            start_time,
            end_time,
            renewables: 45.5,
            channel_type: ChannelType::General,
            tariff_information: None,
            spike_status: SpikeStatus::None,
            descriptor: PriceDescriptor::Low,
        };
        insta::assert_snapshot!(base_interval_basic.to_string(), @"2021-05-05 general 24.33c/kWh (spot: 6.12c/kWh) (low) 45.5% renewable");

        // Test with spike status potential
        let base_interval_potential_spike = BaseInterval {
            duration: 5,
            spot_per_kwh: 6.12,
            per_kwh: 24.33,
            date: Date::constant(2021, 5, 5),
            nem_time,
            start_time,
            end_time,
            renewables: 45.5,
            channel_type: ChannelType::General,
            tariff_information: None,
            spike_status: SpikeStatus::Potential,
            descriptor: PriceDescriptor::High,
        };
        insta::assert_snapshot!(base_interval_potential_spike.to_string(), @"2021-05-05 general 24.33c/kWh (spot: 6.12c/kWh) (high) 45.5% renewable spike: potential");

        // Test with spike status spike
        let base_interval_spike = BaseInterval {
            duration: 5,
            spot_per_kwh: 100.50,
            per_kwh: 120.75,
            date: Date::constant(2021, 5, 5),
            nem_time,
            start_time,
            end_time,
            renewables: 25.0,
            channel_type: ChannelType::General,
            tariff_information: None,
            spike_status: SpikeStatus::Spike,
            descriptor: PriceDescriptor::Spike,
        };
        insta::assert_snapshot!(base_interval_spike.to_string(), @"2021-05-05 general 120.75c/kWh (spot: 100.50c/kWh) (spike) 25% renewable spike: spike");

        // Test with tariff information only
        let tariff_info = TariffInformation {
            period: Some(TariffPeriod::Peak),
            season: Some(TariffSeason::Summer),
            block: Some(2),
            demand_window: Some(true),
        };
        let base_interval_tariff = BaseInterval {
            duration: 30,
            spot_per_kwh: 15.20,
            per_kwh: 35.40,
            date: Date::constant(2021, 7, 15),
            nem_time,
            start_time,
            end_time,
            renewables: 30.2,
            channel_type: ChannelType::ControlledLoad,
            tariff_information: Some(tariff_info),
            spike_status: SpikeStatus::None,
            descriptor: PriceDescriptor::Neutral,
        };
        insta::assert_snapshot!(base_interval_tariff.to_string(), @"2021-07-15 controlled load 35.40c/kWh (spot: 15.20c/kWh) (neutral) 30.2% renewable [period:peak, season:summer, block:2, demand window:true]");

        // Test with both spike status and tariff information
        let tariff_info_combined = TariffInformation {
            period: Some(TariffPeriod::OffPeak),
            season: None,
            block: None,
            demand_window: Some(false),
        };
        let base_interval_combined = BaseInterval {
            duration: 5,
            spot_per_kwh: 8.75,
            per_kwh: 28.90,
            date: Date::constant(2021, 12, 25),
            nem_time,
            start_time,
            end_time,
            renewables: 60.8,
            channel_type: ChannelType::FeedIn,
            tariff_information: Some(tariff_info_combined),
            spike_status: SpikeStatus::Potential,
            descriptor: PriceDescriptor::VeryLow,
        };
        insta::assert_snapshot!(base_interval_combined.to_string(), @"2021-12-25 feed-in 28.90c/kWh (spot: 8.75c/kWh) (very low) 60.8% renewable spike: potential [period:off peak, demand window:false]");
    }

    #[test]
    fn display_actual_interval() {
        use jiff::{Timestamp, civil::Date};
        let nem_time = "2021-05-06T12:30:00+10:00"
            .parse::<Timestamp>()
            .expect("valid timestamp");
        let start_time = "2021-05-05T02:00:01Z"
            .parse::<Timestamp>()
            .expect("valid timestamp");
        let end_time = "2021-05-05T02:30:00Z"
            .parse::<Timestamp>()
            .expect("valid timestamp");

        let actual_interval = ActualInterval {
            base: BaseInterval {
                duration: 5,
                spot_per_kwh: 6.12,
                per_kwh: 24.33,
                date: Date::constant(2021, 5, 5),
                nem_time,
                start_time,
                end_time,
                renewables: 45.5,
                channel_type: ChannelType::General,
                tariff_information: None,
                spike_status: SpikeStatus::None,
                descriptor: PriceDescriptor::Low,
            },
        };
        insta::assert_snapshot!(actual_interval.to_string(), @"Actual: 2021-05-05 general 24.33c/kWh (spot: 6.12c/kWh) (low) 45.5% renewable");
    }

    #[test]
    fn display_forecast_interval() {
        use jiff::{Timestamp, civil::Date};
        let nem_time = "2021-05-06T12:30:00+10:00"
            .parse::<Timestamp>()
            .expect("valid timestamp");
        let start_time = "2021-05-05T02:00:01Z"
            .parse::<Timestamp>()
            .expect("valid timestamp");
        let end_time = "2021-05-05T02:30:00Z"
            .parse::<Timestamp>()
            .expect("valid timestamp");

        let forecast_interval = ForecastInterval {
            base: BaseInterval {
                duration: 5,
                spot_per_kwh: 6.12,
                per_kwh: 24.33,
                date: Date::constant(2021, 5, 5),
                nem_time,
                start_time,
                end_time,
                renewables: 45.5,
                channel_type: ChannelType::General,
                tariff_information: None,
                spike_status: SpikeStatus::Potential,
                descriptor: PriceDescriptor::High,
            },
            range: Some(Range {
                min: 10.0,
                max: 30.0,
            }),
            advanced_price: Some(AdvancedPrice {
                low: 15.0,
                predicted: 20.0,
                high: 25.0,
            }),
        };
        insta::assert_snapshot!(forecast_interval.to_string(), @"Forecast: 2021-05-05 general 24.33c/kWh (spot: 6.12c/kWh) (high) 45.5% renewable spike: potential Range: 10.00-30.00c/kWh Advanced: L:15.00 H:20.00 P:25.00 c/kWh");
    }

    #[test]
    fn display_current_interval() {
        use jiff::{Timestamp, civil::Date};
        let nem_time = "2021-05-06T12:30:00+10:00"
            .parse::<Timestamp>()
            .expect("valid timestamp");
        let start_time = "2021-05-05T02:00:01Z"
            .parse::<Timestamp>()
            .expect("valid timestamp");
        let end_time = "2021-05-05T02:30:00Z"
            .parse::<Timestamp>()
            .expect("valid timestamp");

        let current_interval = CurrentInterval {
            base: BaseInterval {
                duration: 5,
                spot_per_kwh: 6.12,
                per_kwh: 24.33,
                date: Date::constant(2021, 5, 5),
                nem_time,
                start_time,
                end_time,
                renewables: 45.5,
                channel_type: ChannelType::FeedIn,
                tariff_information: None,
                spike_status: SpikeStatus::Spike,
                descriptor: PriceDescriptor::Spike,
            },
            range: Some(Range {
                min: 50.0,
                max: 100.0,
            }),
            estimate: true,
            advanced_price: Some(AdvancedPrice {
                low: 60.0,
                predicted: 75.0,
                high: 90.0,
            }),
        };
        insta::assert_snapshot!(current_interval.to_string(), @"Current: 2021-05-05 feed-in 24.33c/kWh (spot: 6.12c/kWh) (spike) 45.5% renewable spike: spike (estimate) Range: 50.00-100.00c/kWh Advanced: L:60.00 H:75.00 P:90.00 c/kWh");
    }

    #[test]
    fn display_interval_enum() {
        use jiff::{Timestamp, civil::Date};
        let nem_time = "2021-05-06T12:30:00+10:00"
            .parse::<Timestamp>()
            .expect("valid timestamp");
        let start_time = "2021-05-05T02:00:01Z"
            .parse::<Timestamp>()
            .expect("valid timestamp");
        let end_time = "2021-05-05T02:30:00Z"
            .parse::<Timestamp>()
            .expect("valid timestamp");

        let base = BaseInterval {
            duration: 5,
            spot_per_kwh: 6.12,
            per_kwh: 24.33,
            date: Date::constant(2021, 5, 5),
            nem_time,
            start_time,
            end_time,
            renewables: 45.5,
            channel_type: ChannelType::General,
            tariff_information: None,
            spike_status: SpikeStatus::None,
            descriptor: PriceDescriptor::Neutral,
        };

        let actual_interval = Interval::ActualInterval(ActualInterval { base: base.clone() });
        let forecast_interval = Interval::ForecastInterval(ForecastInterval {
            base: base.clone(),
            range: None,
            advanced_price: None,
        });
        let current_interval = Interval::CurrentInterval(CurrentInterval {
            base,
            range: None,
            estimate: false,
            advanced_price: None,
        });

        insta::assert_snapshot!(actual_interval.to_string(), @"Actual: 2021-05-05 general 24.33c/kWh (spot: 6.12c/kWh) (neutral) 45.5% renewable");
        insta::assert_snapshot!(forecast_interval.to_string(), @"Forecast: 2021-05-05 general 24.33c/kWh (spot: 6.12c/kWh) (neutral) 45.5% renewable");
        insta::assert_snapshot!(current_interval.to_string(), @"Current: 2021-05-05 general 24.33c/kWh (spot: 6.12c/kWh) (neutral) 45.5% renewable");
    }

    #[test]
    fn display_usage_quality() {
        insta::assert_snapshot!(UsageQuality::Estimated.to_string(), @"estimated");
        insta::assert_snapshot!(UsageQuality::Billable.to_string(), @"billable");
    }

    #[test]
    fn display_usage() {
        use jiff::{Timestamp, civil::Date};
        let nem_time = "2021-05-06T12:30:00+10:00"
            .parse::<Timestamp>()
            .expect("valid timestamp");
        let start_time = "2021-05-05T02:00:01Z"
            .parse::<Timestamp>()
            .expect("valid timestamp");
        let end_time = "2021-05-05T02:30:00Z"
            .parse::<Timestamp>()
            .expect("valid timestamp");

        let usage = Usage {
            base: BaseInterval {
                duration: 5,
                spot_per_kwh: 6.12,
                per_kwh: 24.33,
                date: Date::constant(2021, 5, 5),
                nem_time,
                start_time,
                end_time,
                renewables: 45.5,
                channel_type: ChannelType::General,
                tariff_information: None,
                spike_status: SpikeStatus::None,
                descriptor: PriceDescriptor::Low,
            },
            channel_identifier: "E1".to_owned(),
            kwh: 1.25,
            quality: UsageQuality::Billable,
            cost: 30.41,
        };
        insta::assert_snapshot!(usage.to_string(), @"Usage E1 1.25kWh $30.41 (billable)");
    }

    #[test]
    fn display_base_renewable() {
        use jiff::{Timestamp, civil::Date};
        let nem_time = "2021-05-06T12:30:00+10:00"
            .parse::<Timestamp>()
            .expect("valid timestamp");
        let start_time = "2021-05-05T02:00:01Z"
            .parse::<Timestamp>()
            .expect("valid timestamp");
        let end_time = "2021-05-05T02:30:00Z"
            .parse::<Timestamp>()
            .expect("valid timestamp");

        let base_renewable = BaseRenewable {
            duration: 5,
            date: Date::constant(2021, 5, 5),
            nem_time,
            start_time,
            end_time,
            renewables: 78.5,
            descriptor: RenewableDescriptor::Great,
        };
        insta::assert_snapshot!(base_renewable.to_string(), @"2021-05-05 78.5% renewable (great)");
    }

    #[test]
    fn display_actual_renewable() {
        use jiff::{Timestamp, civil::Date};
        let nem_time = "2021-05-06T12:30:00+10:00"
            .parse::<Timestamp>()
            .expect("valid timestamp");
        let start_time = "2021-05-05T02:00:01Z"
            .parse::<Timestamp>()
            .expect("valid timestamp");
        let end_time = "2021-05-05T02:30:00Z"
            .parse::<Timestamp>()
            .expect("valid timestamp");

        let actual_renewable = ActualRenewable {
            base: BaseRenewable {
                duration: 5,
                date: Date::constant(2021, 5, 5),
                nem_time,
                start_time,
                end_time,
                renewables: 78.5,
                descriptor: RenewableDescriptor::Great,
            },
        };
        insta::assert_snapshot!(actual_renewable.to_string(), @"Actual: 2021-05-05 78.5% renewable (great)");
    }

    #[test]
    fn display_forecast_renewable() {
        use jiff::{Timestamp, civil::Date};
        let nem_time = "2021-05-06T12:30:00+10:00"
            .parse::<Timestamp>()
            .expect("valid timestamp");
        let start_time = "2021-05-05T02:00:01Z"
            .parse::<Timestamp>()
            .expect("valid timestamp");
        let end_time = "2021-05-05T02:30:00Z"
            .parse::<Timestamp>()
            .expect("valid timestamp");

        let forecast_renewable = ForecastRenewable {
            base: BaseRenewable {
                duration: 5,
                date: Date::constant(2021, 5, 5),
                nem_time,
                start_time,
                end_time,
                renewables: 78.5,
                descriptor: RenewableDescriptor::Great,
            },
        };
        insta::assert_snapshot!(forecast_renewable.to_string(), @"Forecast: 2021-05-05 78.5% renewable (great)");
    }

    #[test]
    fn display_current_renewable() {
        use jiff::{Timestamp, civil::Date};
        let nem_time = "2021-05-06T12:30:00+10:00"
            .parse::<Timestamp>()
            .expect("valid timestamp");
        let start_time = "2021-05-05T02:00:01Z"
            .parse::<Timestamp>()
            .expect("valid timestamp");
        let end_time = "2021-05-05T02:30:00Z"
            .parse::<Timestamp>()
            .expect("valid timestamp");

        let current_renewable = CurrentRenewable {
            base: BaseRenewable {
                duration: 5,
                date: Date::constant(2021, 5, 5),
                nem_time,
                start_time,
                end_time,
                renewables: 78.5,
                descriptor: RenewableDescriptor::Great,
            },
        };
        insta::assert_snapshot!(current_renewable.to_string(), @"Current: 2021-05-05 78.5% renewable (great)");
    }

    #[test]
    fn display_renewable_enum() {
        use jiff::{Timestamp, civil::Date};
        let nem_time = "2021-05-06T12:30:00+10:00"
            .parse::<Timestamp>()
            .expect("valid timestamp");
        let start_time = "2021-05-05T02:00:01Z"
            .parse::<Timestamp>()
            .expect("valid timestamp");
        let end_time = "2021-05-05T02:30:00Z"
            .parse::<Timestamp>()
            .expect("valid timestamp");

        let base = BaseRenewable {
            duration: 5,
            date: Date::constant(2021, 5, 5),
            nem_time,
            start_time,
            end_time,
            renewables: 78.5,
            descriptor: RenewableDescriptor::Great,
        };

        let actual_renewable = Renewable::ActualRenewable(ActualRenewable { base: base.clone() });
        let forecast_renewable =
            Renewable::ForecastRenewable(ForecastRenewable { base: base.clone() });
        let current_renewable = Renewable::CurrentRenewable(CurrentRenewable { base });

        insta::assert_snapshot!(actual_renewable.to_string(), @"Actual: 2021-05-05 78.5% renewable (great)");
        insta::assert_snapshot!(forecast_renewable.to_string(), @"Forecast: 2021-05-05 78.5% renewable (great)");
        insta::assert_snapshot!(current_renewable.to_string(), @"Current: 2021-05-05 78.5% renewable (great)");
    }
}
