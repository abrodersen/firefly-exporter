
use crate::model::*;

use serde::{Serialize, Deserialize};
use serde_json::{from_value, Value};

use chrono::prelude::*;

pub struct Client {
    base: String,
    token: String,
    client: reqwest::Client,
}

#[derive(Deserialize, Debug)]
#[serde(tag="type", content="attributes")]
enum EntityType {
    #[serde(rename = "transactions")]
    Transaction(TransactionGroup)
}

#[derive(Deserialize, Debug)]
struct Entity {
    id: String,

    #[serde(flatten)]
    data: EntityType,
}

#[derive(Deserialize, Debug)]
struct Response {
    data: Vec<Entity>,
}

impl Client {

    pub fn new<A, B>(base: A, token: B) -> Client 
        where A: Into<String>, B: Into<String>
    {
        Client {
            base: base.into(),
            token: token.into(),
            client: reqwest::Client::new(),
        }
    }

    pub async fn get_daily_transactions(&self, day: NaiveDate) -> Result<Vec<TransactionGroup>, Box<dyn std::error::Error>> {
        let uri = format!("{}/api/v1/transactions", self.base);

        let end = day + chrono::Duration::days(1);
        let json = self.client
            .get(&uri)
            .bearer_auth(&self.token)
            .query(&[("start", day), ("end", end)])
            .send()
            .await?
            .json::<Value>()
            .await?;
        
        // println!("{:#?}", json);

        let structured: Response = from_value(json)?;
        
        let data = structured.data.into_iter().filter_map(|e| {
            let id = e.id;
            match e.data {
                EntityType::Transaction(mut grp) => {
                    grp.id = id;
                    Some(grp)
                },
            }
        }).collect::<Vec<_>>();

        Ok(data)
    }
}