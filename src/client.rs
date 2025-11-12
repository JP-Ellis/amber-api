//! # Amber Electric API Client
//!
//! This module provides a client for interacting with the [Amber Electric Public API](https://api.amber.com.au/v1).

use crate::{error::Result, models};
use serde::de::DeserializeOwned;
use tracing::{debug, instrument};

/// The base URL for the Amber Electric API
const API_BASE_URL: &str = "https://api.amber.com.au/v1/";

/// Main client for the Amber Electric API
///
/// This client provides a high-level interface to all Amber Electric API
/// endpoints.
#[derive(Debug, bon::Builder)]
pub struct Amber {
    /// HTTP client for making requests
    agent: ureq::Agent,
    /// Optional API key for authenticated requests
    api_key: Option<String>,
    /// Base URL for the Amber API
    base_url: String,
}

impl Default for Amber {
    /// Create a new default Amber API client.
    ///
    /// This create a default client that is authenticated if an API key is set
    /// in the `AMBER_API_KEY` environment variable.
    #[inline]
    fn default() -> Self {
        debug!("Creating default Amber API client");
        Self {
            agent: ureq::agent(),
            api_key: std::env::var("AMBER_API_KEY")
                .ok()
                .filter(|s| !s.is_empty()),
            base_url: API_BASE_URL.to_owned(),
        }
    }
}

#[bon::bon]
impl Amber {
    /// Perform a GET request to the Amber API.
    #[instrument(skip(self, query), level = "debug")]
    #[expect(
        clippy::cognitive_complexity,
        reason = "False positive due to macro expansions"
    )]
    fn get<T: DeserializeOwned, I, K, V>(&self, path: &str, query: I) -> Result<T>
    where
        I: IntoIterator<Item = (K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let endpoint = format!("{}{}", self.base_url, path);
        debug!("GET {endpoint}");

        let mut request = self.agent.get(&endpoint);
        if let Some(api_key) = &self.api_key {
            request = request.header("Authorization", &format!("Bearer {api_key}"));
        }

        for (key, value) in query {
            debug!("Query parameter: {}={}", key.as_ref(), value.as_ref());
            request = request.query(key.as_ref(), value.as_ref());
        }

        let mut response = request.call()?;
        debug!("Status code: {}", response.status());
        Ok(response.body_mut().read_json()?)
    }

    /// Returns the current percentage of renewables in the grid for a specific state.
    ///
    /// This method retrieves renewable energy data for the specified Australian state.
    /// The data shows the current percentage of renewable energy in the grid and can
    /// optionally include historical and forecast data.
    ///
    /// # Parameters
    ///
    /// - `state`: The Australian state (NSW, VIC, QLD, SA)
    /// - `next`: Optional number of forecast intervals to return
    /// - `previous`: Optional number of historical intervals to return
    /// - `resolution`: Optional interval duration (5 or 30 minutes, default 30)
    ///
    /// # Authentication
    ///
    /// This endpoint does not require authentication and can be called without an API key.
    ///
    /// # Returns
    ///
    /// Returns a [`Result`] containing a [`Vec`] of [`Renewable`] objects on success.
    ///
    /// # Errors
    ///
    /// This method will return an error if:
    ///
    /// - There's a network error communicating with the API
    /// - The API returns an internal server error (HTTP 500)
    ///
    /// # Example
    ///
    /// ```
    /// use amber_api::Amber;
    /// use amber_api::models::{State, Resolution};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Amber::default();
    ///
    /// // Get current renewables data for Victoria
    /// let renewables = client.current_renewables()
    ///     .state(State::Vic)
    ///     .call()?;
    ///
    /// for renewable in renewables {
    ///     println!("{}", renewable);
    /// }
    ///
    /// // Get current data with 8 forecast intervals
    /// let renewables_with_forecast = client.current_renewables()
    ///     .state(State::Nsw)
    ///     .next(8)
    ///     .resolution(Resolution::FiveMinute)
    ///     .call()?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`Renewable`]: crate::models::Renewable
    #[inline]
    #[builder]
    pub fn current_renewables(
        &self,
        state: models::State,
        next: Option<u32>,
        previous: Option<u32>,
        resolution: Option<models::Resolution>,
    ) -> Result<Vec<models::Renewable>> {
        self.get(
            &format!("state/{state}/renewables/current"),
            [
                ("next", next.map(|n| n.to_string())),
                ("previous", previous.map(|p| p.to_string())),
                ("resolution", resolution.map(|r| r.to_string())),
            ]
            .into_iter()
            .filter_map(|(k, v)| v.map(|val| (k, val))),
        )
    }

    /// Return all sites linked to your account.
    ///
    /// This method returns information about all electricity sites associated with your
    /// Amber account. Each site represents a location where you have electricity service.
    ///
    /// # Authentication
    ///
    /// This method requires authentication via API key. The API key can be provided
    /// either through the `AMBER_API_KEY` environment variable (when using [`Amber::default()`])
    /// or by explicitly setting it when building the client.
    ///
    /// # Returns
    ///
    /// Returns a [`Result`] containing a [`Vec`] of [`Site`] objects on success.
    ///
    /// # Errors
    ///
    /// This method will return an error if:
    ///
    /// - The API key is missing or invalid (HTTP 401)
    /// - There's a network error communicating with the API
    /// - The API returns an internal server error (HTTP 500)
    ///
    /// # Example
    ///
    /// ```
    /// use amber_api::Amber;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Amber::default();
    /// let sites = client.sites()?;
    ///
    /// for site in sites {
    ///     println!("Site {}: {} ({})", site.id, site.network, site.status);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`Site`]: crate::models::Site
    #[inline]
    pub fn sites(&self) -> Result<Vec<crate::models::Site>> {
        self.get("sites", core::iter::empty::<(&str, &str)>())
    }

    /// Returns all the prices between the start and end dates for a specific site.
    ///
    /// This method retrieves historical pricing data for the specified site between
    /// the given date range. The date range cannot exceed 7 days.
    ///
    /// # Parameters
    ///
    /// - `site_id`: ID of the site you are fetching prices for (obtained from [`sites()`])
    /// - `start_date`: Optional start date for the price range (defaults to today)
    /// - `end_date`: Optional end date for the price range (defaults to today)
    /// - `resolution`: Optional interval duration (5 or 30 minutes, defaults to your billing interval)
    ///
    /// # Authentication
    ///
    /// This method requires authentication via API key. The API key can be provided
    /// either through the `AMBER_API_KEY` environment variable (when using [`Amber::default()`])
    /// or by explicitly setting it when building the client.
    ///
    /// # Returns
    ///
    /// Returns a [`Result`] containing a [`Vec`] of [`Interval`] objects on success.
    /// Intervals are returned in order: General > Controlled Load > Feed In.
    ///
    /// # Errors
    ///
    /// This method will return an error if:
    ///
    /// - The API key is missing or invalid (HTTP 401)
    /// - The site ID is invalid (HTTP 400)
    /// - The site is not found (HTTP 404)
    /// - The date range exceeds 7 days (HTTP 422)
    /// - There's a network error communicating with the API
    /// - The API returns an internal server error (HTTP 500)
    ///
    /// # Example
    ///
    /// ```
    /// use std::str::FromStr;
    ///
    /// use amber_api::Amber;
    /// use amber_api::models::Resolution;
    /// use jiff::civil::Date;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Amber::default();
    /// let sites = client.sites()?;
    /// let site_id = &sites[0].id;
    ///
    /// // Get prices for today
    /// let prices = client.prices()
    ///     .site_id(site_id)
    ///     .call()?;
    ///
    /// // Get prices for a specific date range
    /// let start_date = Date::from_str("2021-05-01").expect("Invalid start date");
    /// let end_date = Date::from_str("2021-05-03").expect("Invalid end date");
    /// let prices = client.prices()
    ///     .site_id(site_id)
    ///     .start_date(start_date)
    ///     .end_date(end_date)
    ///     .resolution(Resolution::FiveMinute)
    ///     .call()?;
    ///
    /// for interval in prices {
    ///     match interval {
    ///         amber_api::models::Interval::ActualInterval(actual) => {
    ///             println!("Actual price: {:.2}c/kWh", actual.base.per_kwh);
    ///         }
    ///         _ => {} // Handle other interval types as needed
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`sites()`]: Self::sites
    /// [`Interval`]: crate::models::Interval
    #[inline]
    #[builder]
    pub fn prices(
        &self,
        site_id: &str,
        start_date: Option<jiff::civil::Date>,
        end_date: Option<jiff::civil::Date>,
        resolution: Option<models::Resolution>,
    ) -> Result<Vec<models::Interval>> {
        self.get(
            &format!("sites/{site_id}/prices"),
            [
                ("startDate", start_date.map(|d| d.to_string())),
                ("endDate", end_date.map(|d| d.to_string())),
                ("resolution", resolution.map(|r| r.to_string())),
            ]
            .into_iter()
            .filter_map(|(k, v)| v.map(|val| (k, val))),
        )
    }

    /// Returns the current price for a specific site.
    ///
    /// This method retrieves the current pricing data for the specified site,
    /// optionally including historical and forecast data.
    ///
    /// # Parameters
    ///
    /// - `site_id`: ID of the site you are fetching prices for (obtained from [`sites()`])
    /// - `next`: Optional number of forecast intervals to return (max 2048 total)
    /// - `previous`: Optional number of historical intervals to return (max 2048 total)
    /// - `resolution`: Optional interval duration (5 or 30 minutes, defaults to your billing interval)
    ///
    /// # Authentication
    ///
    /// This method requires authentication via API key. The API key can be provided
    /// either through the `AMBER_API_KEY` environment variable (when using [`Amber::default()`])
    /// or by explicitly setting it when building the client.
    ///
    /// # Returns
    ///
    /// Returns a [`Result`] containing a [`Vec`] of [`Interval`] objects on success.
    /// Intervals are returned in order: General > Controlled Load > Feed In.
    ///
    /// # Errors
    ///
    /// This method will return an error if:
    ///
    /// - The API key is missing or invalid (HTTP 401)
    /// - The site ID is invalid (HTTP 400)
    /// - The site is not found (HTTP 404)
    /// - The total number of intervals exceeds 2048 (HTTP 422)
    /// - There's a network error communicating with the API
    /// - The API returns an internal server error (HTTP 500)
    ///
    /// # Example
    ///
    /// ```
    /// use amber_api::Amber;
    /// use amber_api::models::Resolution;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Amber::default();
    /// let sites = client.sites()?;
    /// let site_id = &sites[0].id;
    ///
    /// // Get current prices only
    /// let current_prices = client.current_prices()
    ///     .site_id(site_id)
    ///     .call()?;
    ///
    /// // Get current prices with forecast
    /// let prices_with_forecast = client.current_prices()
    ///     .site_id(site_id)
    ///     .next(8)
    ///     .resolution(Resolution::ThirtyMinute)
    ///     .call()?;
    ///
    /// // Get current prices with history and forecast
    /// let full_prices = client.current_prices()
    ///     .site_id(site_id)
    ///     .previous(8)
    ///     .next(8)
    ///     .call()?;
    ///
    /// for interval in current_prices {
    ///     match interval {
    ///         amber_api::models::Interval::CurrentInterval(current) => {
    ///             println!("Current price: {:.2}c/kWh (estimate: {})",
    ///                      current.base.per_kwh, current.estimate);
    ///         }
    ///         _ => {} // Handle other interval types as needed
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`sites()`]: Self::sites
    /// [`Interval`]: crate::models::Interval
    #[inline]
    #[builder]
    pub fn current_prices(
        &self,
        site_id: &str,
        next: Option<u32>,
        previous: Option<u32>,
        resolution: Option<models::Resolution>,
    ) -> Result<Vec<models::Interval>> {
        self.get(
            &format!("sites/{site_id}/prices/current"),
            [
                ("next", next.map(|n| n.to_string())),
                ("previous", previous.map(|p| p.to_string())),
                ("resolution", resolution.map(|r| r.to_string())),
            ]
            .into_iter()
            .filter_map(|(k, v)| v.map(|val| (k, val))),
        )
    }

    /// Returns all usage data between the start and end dates for a specific site.
    ///
    /// This method retrieves historical usage data for the specified site between
    /// the given date range. The date range cannot exceed 7 days, and the API can
    /// only return 90 days worth of data.
    ///
    /// # Parameters
    ///
    /// - `site_id`: ID of the site you are fetching usage for (obtained from [`sites()`])
    /// - `start_date`: Start date for the usage data (required)
    /// - `end_date`: End date for the usage data (required)
    /// - `resolution`: Optional interval duration (deprecated, will be ignored)
    ///
    /// # Authentication
    ///
    /// This method requires authentication via API key. The API key can be provided
    /// either through the `AMBER_API_KEY` environment variable (when using [`Amber::default()`])
    /// or by explicitly setting it when building the client.
    ///
    /// # Returns
    ///
    /// Returns a [`Result`] containing a [`Vec`] of [`Usage`] objects on success.
    /// Usage data is returned in order: General > Controlled Load > Feed In.
    ///
    /// # Errors
    ///
    /// This method will return an error if:
    ///
    /// - The API key is missing or invalid (HTTP 401)
    /// - The site ID is invalid (HTTP 400)
    /// - The site is not found (HTTP 404)
    /// - The date range exceeds 7 days (HTTP 422)
    /// - There's a network error communicating with the API
    /// - The API returns an internal server error (HTTP 500)
    ///
    /// # Example
    ///
    /// ```
    /// use std::str::FromStr;
    ///
    /// use amber_api::Amber;
    /// use jiff::civil::Date;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Amber::default();
    /// let sites = client.sites()?;
    /// let site_id = &sites[0].id;
    ///
    /// // Get usage data for a specific date range
    /// let start_date = Date::from_str("2021-05-01").expect("Invalid start date");
    /// let end_date = Date::from_str("2021-05-03").expect("Invalid end date");
    /// let usage_data = client.usage()
    ///     .site_id(site_id)
    ///     .start_date(start_date)
    ///     .end_date(end_date)
    ///     .call()?;
    ///
    /// for usage in usage_data {
    ///     println!("Channel {}: {:.2} kWh, Cost: ${:.2}",
    ///              usage.channel_identifier, usage.kwh, usage.cost);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`sites()`]: Self::sites
    /// [`Usage`]: crate::models::Usage
    #[inline]
    #[builder]
    pub fn usage(
        &self,
        site_id: &str,
        start_date: jiff::civil::Date,
        end_date: jiff::civil::Date,
    ) -> Result<Vec<models::Usage>> {
        let start_date_str = start_date.to_string();
        let end_date_str = end_date.to_string();
        let query_params = [
            ("startDate", start_date_str.as_str()),
            ("endDate", end_date_str.as_str()),
        ];

        self.get(&format!("sites/{site_id}/usage"), query_params)
    }
}
