use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserBalance {
    /// Whether the user's balance is sufficient for API calls.
    pub is_available: bool,
    pub balance_infos: Vec<BalacneInfo>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BalacneInfo {
    /// The currency of the balance.
    pub currency: Currency,
    /// The total available balance, including the granted balance and the topped-up balance.
    pub total_balance: String,
    /// The total not expired granted balance.
    pub granted_balance: String,
    /// The total topped-up balance.
    pub topped_up_balance: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub enum Currency {
    #[serde(rename = "CNY")]
    Cny,
    #[serde(rename = "USD")]
    Usd
}