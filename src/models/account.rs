use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Account {
    /// Accepted ToS version
    pub accepted_terms_of_service_version: Option<i32>,

    /// True if this account is an affiliate
    pub affiliate: Option<bool>,

    pub autorefill_amount: Option<f32>,

    /// Timestamp of email confirmation
    pub confirmed_at: Option<i32>,

    /// Number of credits left
    pub credits: f32,

    /// Account email
    pub email: String,

    /// Discount percentage for the first month of a subscription
    pub first_month_discount_percentage: Option<i32>,

    /// Gravatar URL
    pub gravatar_url: String,

    /// Account ID
    pub id: String,

    /// Account discount percentage
    pub lifetime_discount_percentage: Option<i32>,

    /// Account subscription discount percentage
    pub lifetime_subscription_discount_percentage: Option<i32>,

    pub marketing_emails_enabled: Option<bool>,

    /// Special roles at dathost.net
    pub roles: Option<Vec<String>>,

    /// Approximately seconds the credits will last with current usage. Returns null if no servers are on
    pub seconds_left: Option<i32>,

    /// If true, try to pay subscription with credits first
    pub subscription_pay_with_credits: Option<bool>,

    /// Human readable approximate of the time the credits will last with current usage, for seconds see seconds_left
    pub time_left: Option<String>,

    /// An account is considered trial when no payments has been made
    pub trial: Option<bool>,
}
