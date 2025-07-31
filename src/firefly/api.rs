use std::collections::HashMap;

use chrono::prelude::*;

#[derive(Debug, serde::Deserialize)]
pub struct FireflyResponse<T> {
    pub data: T,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct DataEntry<ATTR> {
    #[serde(rename = "type")]
    pub ty: String,
    pub id: String,
    pub attributes: ATTR,
    pub links: serde_json::Value,
    #[serde(flatten)]
    pub other: HashMap<String, serde_json::Value>,
}

pub type Bill = DataEntry<BillAttributes>;

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

pub type Budget = DataEntry<BudgetAttributes>;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct BudgetAttributes {
    pub name: String,
    pub active: bool,
    pub spent: Vec<Spent>,
    pub auto_budget_amount: String,
    #[serde(flatten)]
    pub other: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Currency<ID> {
    pub currency_name: String,
    pub currency_decimal_places: u64,
    pub currency_symbol: String,
    pub currency_id: ID,
    pub currency_code: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Spent {
    pub sum: String,
    #[serde(flatten)]
    pub currency: Currency<i64>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Earned {
    pub sum: String,
    #[serde(flatten)]
    pub currency: Currency<String>,
}

pub type ListCategory = DataEntry<ListCategoryAttributes>;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct ListCategoryAttributes {
    pub name: String,
    pub earned: Vec<serde_json::Value>,
    pub spent: Vec<serde_json::Value>,
    #[serde(flatten)]
    pub other: HashMap<String, serde_json::Value>,
}

pub type DetailsCategory = DataEntry<DetailsCategoryAttributes>;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct DetailsCategoryAttributes {
    pub name: String,
    pub earned: Vec<Earned>,
    pub spent: Vec<Spent>,
    #[serde(flatten)]
    pub other: HashMap<String, serde_json::Value>,
}
