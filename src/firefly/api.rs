use std::collections::HashMap;

use chrono::prelude::*;

#[derive(Debug, serde::Deserialize)]
pub struct FireflyResponse<T> {
    pub data: T,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Bill {
    #[serde(rename = "type")]
    pub ty: String,
    pub id: String,
    pub attributes: BillAttributes,
    pub links: serde_json::Value,
    #[serde(flatten)]
    pub other: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct BillAttributes {
    pub active: bool,
    pub name: String,
    pub date: chrono::DateTime<Utc>,
    pub next_expected_match: chrono::DateTime<Utc>,
    pub pay_dates: Vec<chrono::DateTime<Utc>>,
    pub amount_min: String,
    pub amount_max: String,
    pub currency_symbol: String,
    #[serde(flatten)]
    pub other: HashMap<String, serde_json::Value>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Budget {
    #[serde(rename = "type")]
    pub ty: String,
    pub id: String,
    pub attributes: BudgetAttributes,
    #[serde(flatten)]
    pub other: HashMap<String, serde_json::Value>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct BudgetAttributes {
    pub name: String,
    pub active: bool,
    pub spent: Vec<BudgetSpent>,
    pub auto_budget_amount: String,
    #[serde(flatten)]
    pub other: HashMap<String, serde_json::Value>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Currency {
    pub currency_name: String,
    pub currency_decimal_places: u64,
    pub currency_symbol: String,
    pub currency_id: u64,
    pub currency_code: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct BudgetSpent {
    pub sum: String,
    #[serde(flatten)]
    pub currency: Currency,
}
