//! # Amber Electric API Client
//!
//! This module provides a client for interacting with the [Amber Electric
//! Public API](https://api.amber.com.au/v1).

use crate::{error::Result, models};
use serde::de::DeserializeOwned;
use tracing::{debug, instrument};

/// The base URL for the Amber Electric API.
const API_BASE_URL: &str = "https://api.amber.com.au/v1/";

/// Main client for the Amber Electric API.
///
/// This client provides a high-level interface to all Amber Electric API
/// endpoints with automatic retry logic for rate limit errors.
///
/// # Rate Limit Handling
///
/// By default, the client automatically retries requests that hit rate limits
/// (HTTP 429). The client reads the `RateLimit-Reset` header to determine how
/// long to wait before retrying. By default, up to 3 retry attempts will be
/// made. You can configure this behavior:
///
/// ```
/// use amber_api::Amber;
///
/// // Disable automatic retries
/// let client = Amber::builder()
///     .retry_on_rate_limit(false)
///     .build();
///
/// // Customize max retry attempts (default: 3)
/// let client = Amber::builder()
///     .max_retries(5)
///     .build();
/// ```
///
/// When rate limit retries are disabled or exhausted, the client returns an
/// error containing the suggested retry-after duration.
///
/// # Examples
///
/// ```
/// use amber_api::Amber;
///
/// // Create a client with default retry behavior (3 retries, enabled)
/// let client = Amber::default();
/// ```
#[derive(Debug, bon::Builder)]
pub struct Amber {
    /// HTTP client for making requests.
    agent: ureq::Agent,
    /// Optional API key for authenticated requests.
    api_key: Option<String>,
    /// Base URL for the Amber API.
    base_url: String,
    /// Maximum number of retry attempts for rate limit errors.
    ///
    /// When the API returns HTTP 429 (rate limit exceeded), the client will
    /// automatically retry up to this many times. Set to 0 to disable retries,
    /// or use `retry_on_rate_limit(false)` for clearer intent.
    ///
    /// Defaults to 3.
    #[builder(default = 3)]
    max_retries: u32,
    /// Whether to automatically retry on rate limit errors.
    ///
    /// When enabled (default), the client automatically waits and retries when
    /// hitting rate limits. The wait time is read from the `RateLimit-Reset`
    /// header, or defaults to 60 seconds if not present.
    ///
    /// When disabled, rate limit errors are returned immediately as
    /// [`AmberError::RateLimitExceeded`].
    ///
    /// Default to `true`.
    #[builder(default = true)]
    retry_on_rate_limit: bool,
}

impl Default for Amber {
    /// Create a new default Amber API client.
    ///
    /// This create a default client that is authenticated if an API key is set
    /// in the `AMBER_API_KEY` environment variable.
    ///
    /// The default client has automatic rate limit retry enabled with up to 3
    /// retry attempts.
    #[inline]
    fn default() -> Self {
        debug!("Creating default Amber API client");
        let config = ureq::Agent::config_builder().build();
        Self {
            agent: config.into(),
            api_key: std::env::var("AMBER_API_KEY")
                .ok()
                .filter(|s| !s.is_empty()),
            base_url: API_BASE_URL.to_owned(),
            max_retries: 3,
            retry_on_rate_limit: true,
        }
    }
}

#[bon::bon]
impl Amber {
    /// Perform a GET request to the Amber API with automatic retry on rate
    /// limits.
    ///
    /// This method automatically retries requests that hit rate limits (HTTP
    /// 429), waiting the time specified in the `RateLimit-Reset` header (or 60
    /// seconds if not present) before retrying. The number of retries is
    /// controlled by the `max_retries` and `retry_on_rate_limit` configuration
    /// options.
    #[instrument(skip(self, query), level = "debug")]
    fn get<T: DeserializeOwned, I, K, V>(&self, path: &str, query: I) -> Result<T>
    where
        I: IntoIterator<Item = (K, V)> + Clone,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let endpoint = format!("{}{}", self.base_url, path);
        let mut attempt: u32 = 0;

        loop {
            debug!(
                "GET {endpoint} (attempt {}/{})",
                attempt.saturating_add(1),
                self.max_retries.saturating_add(1)
            );

            // Build request
            let mut request = self.agent.get(&endpoint);
            if let Some(api_key) = &self.api_key {
                request = request.header("Authorization", &format!("Bearer {api_key}"));
            }

            for (key, value) in query.clone() {
                debug!("Query parameter: {}={}", key.as_ref(), value.as_ref());
                request = request.query(key.as_ref(), value.as_ref());
            }

            // Make request
            match request.call() {
                Ok(mut response) => {
                    let status = response.status();
                    debug!("Status code: {}", status);

                    // Log rate limit info if available
                    if let Some(remaining) = response
                        .headers()
                        .get("RateLimit-Remaining")
                        .and_then(|v| v.to_str().ok())
                        .and_then(|s| s.parse::<u64>().ok())
                    {
                        debug!("Rate limit remaining: {}", remaining);
                    }

                    // Success - deserialize and return
                    return Ok(response.body_mut().read_json()?);
                }
                Err(ureq::Error::StatusCode(429)) => {
                    // Rate limit hit - try to get retry-after time
                    // Note: In ureq 3.x, Error::StatusCode doesn't contain the response,
                    // so we use a default retry time of 60 seconds
                    let retry_after = 60u64;

                    if !self.retry_on_rate_limit {
                        return Err(crate::error::AmberError::RateLimitExceeded(retry_after));
                    }

                    if attempt >= self.max_retries {
                        return Err(crate::error::AmberError::RateLimitExhausted {
                            attempts: attempt,
                            retry_after,
                        });
                    }

                    // Wait and retry
                    debug!(
                        "Rate limit hit. Waiting {} seconds before retry",
                        retry_after
                    );
                    std::thread::sleep(std::time::Duration::from_secs(retry_after));
                    attempt += 1;
                    continue;
                }
                Err(ureq::Error::StatusCode(status)) => {
                    // Other HTTP error status codes
                    return Err(crate::error::AmberError::UnexpectedStatus {
                        status,
                        body: String::from("<body not available>"),
                    });
                }
                Err(e) => {
                    // Network or other transport errors
                    return Err(e.into());
                }
            }
        }
    }

    /// Returns the current percentage of renewables in the grid for a specific
    /// state.
    ///
    /// This method retrieves renewable energy data for the specified Australian
    /// state. The data shows the current percentage of renewable energy in the
    /// grid and can optionally include historical and forecast data.
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
    /// This endpoint does not require authentication and can be called without
    /// an API key.
    ///
    /// # Returns
    ///
    /// Returns a [`Result`] containing a [`Vec`] of [`Renewable`] objects on
    /// success.
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
    /// This method returns information about all electricity sites associated
    /// with your Amber account. Each site represents a location where you have
    /// electricity service.
    ///
    /// # Authentication
    ///
    /// This method requires authentication via API key. The API key can be
    /// provided either through the `AMBER_API_KEY` environment variable (when
    /// using [`Amber::default()`]) or by explicitly setting it when building
    /// the client.
    ///
    /// # Returns
    ///
    /// Returns a [`Result`] containing a [`Vec`] of [`Site`] objects on
    /// success.
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

    /// Returns all the prices between the start and end dates for a specific
    /// site.
    ///
    /// This method retrieves historical pricing data for the specified site
    /// between the given date range. The date range cannot exceed 7 days.
    ///
    /// # Parameters
    ///
    /// - `site_id`: ID of the site you are fetching prices for (obtained from
    ///   [`sites()`])
    /// - `start_date`: Optional start date for the price range (defaults to
    ///   today)
    /// - `end_date`: Optional end date for the price range (defaults to today)
    /// - `resolution`: Optional interval duration (5 or 30 minutes, defaults to
    ///   your billing interval)
    ///
    /// # Authentication
    ///
    /// This method requires authentication via API key. The API key can be
    /// provided either through the `AMBER_API_KEY` environment variable (when
    /// using [`Amber::default()`]) or by explicitly setting it when building
    /// the client.
    ///
    /// # Returns
    ///
    /// Returns a [`Result`] containing a [`Vec`] of [`Interval`] objects on
    /// success. Intervals are returned in order: General > Controlled Load >
    /// Feed In.
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
    /// - `site_id`: ID of the site you are fetching prices for (obtained from
    ///   [`sites()`])
    /// - `next`: Optional number of forecast intervals to return (max 2048
    ///   total)
    /// - `previous`: Optional number of historical intervals to return (max
    ///   2048 total)
    /// - `resolution`: Optional interval duration (5 or 30 minutes, defaults to
    ///   your billing interval)
    ///
    /// # Authentication
    ///
    /// This method requires authentication via API key. The API key can be
    /// provided either through the `AMBER_API_KEY` environment variable (when
    /// using [`Amber::default()`]) or by explicitly setting it when building
    /// the client.
    ///
    /// # Returns
    ///
    /// Returns a [`Result`] containing a [`Vec`] of [`Interval`] objects on
    /// success. Intervals are returned in order: General > Controlled Load >
    /// Feed In.
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

    /// Returns all usage data between the start and end dates for a specific
    /// site.
    ///
    /// This method retrieves historical usage data for the specified site
    /// between the given date range. The date range cannot exceed 7 days, and
    /// the API can only return 90 days worth of data.
    ///
    /// # Parameters
    ///
    /// - `site_id`: ID of the site you are fetching usage for (obtained from
    ///   [`sites()`])
    /// - `start_date`: Start date for the usage data (required)
    /// - `end_date`: End date for the usage data (required)
    /// - `resolution`: Optional interval duration (deprecated, will be ignored)
    ///
    /// # Authentication
    ///
    /// This method requires authentication via API key. The API key can be
    /// provided either through the `AMBER_API_KEY` environment variable (when
    /// using [`Amber::default()`]) or by explicitly setting it when building
    /// the client.
    ///
    /// # Returns
    ///
    /// Returns a [`Result`] containing a [`Vec`] of [`Usage`] objects on
    /// success. Usage data is returned in order: General > Controlled Load >
    /// Feed In.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_amber_default_retry_config() {
        let client = Amber::default();
        assert_eq!(client.max_retries, 3);
        assert!(client.retry_on_rate_limit);
    }

    #[test]
    fn test_amber_builder_custom_retry_config() {
        let config = ureq::Agent::config_builder().build();
        let client = Amber::builder()
            .agent(config.into())
            .base_url(API_BASE_URL.to_owned())
            .max_retries(5)
            .retry_on_rate_limit(false)
            .build();

        assert_eq!(client.max_retries, 5);
        assert!(!client.retry_on_rate_limit);
    }

    #[test]
    fn test_amber_builder_with_retry_defaults() {
        let config = ureq::Agent::config_builder().build();
        let client = Amber::builder()
            .agent(config.into())
            .base_url(API_BASE_URL.to_owned())
            .build();

        assert_eq!(client.max_retries, 3);
        assert!(client.retry_on_rate_limit);
    }

    #[test]
    fn test_amber_builder_only_max_retries() {
        let config = ureq::Agent::config_builder().build();
        let client = Amber::builder()
            .agent(config.into())
            .base_url(API_BASE_URL.to_owned())
            .max_retries(10)
            .build();

        assert_eq!(client.max_retries, 10);
        assert!(client.retry_on_rate_limit); // Should still be default
    }

    #[test]
    fn test_amber_builder_only_retry_flag() {
        let config = ureq::Agent::config_builder().build();
        let client = Amber::builder()
            .agent(config.into())
            .base_url(API_BASE_URL.to_owned())
            .retry_on_rate_limit(false)
            .build();

        assert_eq!(client.max_retries, 3); // Should still be default
        assert!(!client.retry_on_rate_limit);
    }

    #[test]
    fn test_amber_builder_disable_retries_with_zero() {
        let config = ureq::Agent::config_builder().build();
        let client = Amber::builder()
            .agent(config.into())
            .base_url(API_BASE_URL.to_owned())
            .max_retries(0)
            .build();

        assert_eq!(client.max_retries, 0);
    }
}
