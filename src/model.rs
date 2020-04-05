
use chrono::prelude::*;
use decimal::d128;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionGroup {
    #[serde(skip)]
    pub id: String,
    pub group_title: Option<String>,
    pub transactions: Vec<Transaction>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TransactionType {
    #[serde(rename = "withdrawal")]
    Withdrawal,
    #[serde(rename = "deposit")]
    Deposit,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub date: DateTime<Utc>,
    #[serde(rename = "type")]
    pub kind: TransactionType,
    pub amount: d128,
    pub category_id: Option<u64>,
    pub description: Option<String>,
    pub external_id: Option<String>,
    pub source_id: Option<u64>,
    pub destination_id: Option<u64>,
}