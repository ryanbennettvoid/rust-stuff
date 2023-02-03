use tokio::sync::Mutex;
use tokio_postgres::{Client};
use uuid::{Uuid};

#[derive(Debug)]
pub struct Credits {
    pub id: String,
    pub user_id: String,
    pub balance: u32,
}

#[tonic::async_trait]
pub trait DAOInterface {
    async fn topup_credits_by_user_id(
        &self,
        user_id: String,
        amount: u32,
        cause: String,
    ) -> Result<Credits, String>;

    async fn consume_credits_by_user_id(
        &self,
        user_id: String,
        amount: u32,
        cause: String,
    ) -> Result<Credits, String>;

    async fn get_credits_by_user_id(
        &self,
        user_id: String
    ) -> Result<Credits, String>;
}

pub struct DAO {
    db_client: Mutex<Client>,
}

impl DAO {
    pub fn new(db_client: Mutex<Client>) -> Self {
        Self { db_client }
    }
}

#[tonic::async_trait]
impl DAOInterface for DAO {
    async fn topup_credits_by_user_id(
        &self,
        user_id: String,
        amount: u32,
        cause: String,
    ) -> Result<Credits, String> {
        let db_client = self.db_client.lock().await;

        let parsed_uuid = match Uuid::parse_str(user_id.as_str()) {
            Err(err) => return Err(format!("UUID parse err: {}", err.to_string())),
            Ok(result) => result,
        };
        let parsed_amount = amount as i32;

        let query_result = match db_client.query(
            "INSERT INTO credits (user_id, balance) VALUES ($1, $2) ON CONFLICT (user_id) DO UPDATE SET balance = credits.balance + EXCLUDED.balance RETURNING id, user_id, balance",
            &[&parsed_uuid, &parsed_amount]
        ).await {
            Err(err) => return Err(format!("db query err: {}", err.to_string())),
            Ok(result) => result
        };

        let credits = match query_result.iter().next() {
            None => {
                return Err(format!(
                    "failed to insert credits row: {} {}",
                    user_id, amount
                ))
            },
            Some(row) => {
                let id: Uuid = row.get(0);
                let user_id: Uuid = row.get(1);
                let balance: i32 = row.get(2);
                Credits {
                    id: id.to_string(),
                    user_id: user_id.to_string(),
                    balance: balance as u32,
                }
            }
        };

        // add history
        let delta = parsed_amount;
        let parsed_caused = cause.clone();
        match db_client.query(
            "INSERT INTO credits_history (user_id, delta, cause) VALUES ($1, $2, $3)",
            &[&parsed_uuid, &delta, &parsed_caused]
        ).await {
            Err(err) => return Err(format!("db query err: {}", err)),
            Ok(_) => Ok(credits)
        }
    }

    async fn consume_credits_by_user_id(
        &self,
        user_id: String,
        amount: u32,
        cause: String,
    ) -> Result<Credits, String> {

        let db_client = self.db_client.lock().await;

        let parsed_uuid = match Uuid::parse_str(user_id.as_str()) {
            Err(err) => return Err(format!("UUID parse err: {}", err.to_string())),
            Ok(result) => result,
        };
        let parsed_amount = amount as i32;

        let credits = match db_client.query(
            "UPDATE credits SET balance = credits.balance - $1 WHERE user_id = $2 AND credits.balance >= $3 RETURNING id, user_id, balance",
            &[&parsed_amount, &parsed_uuid, &parsed_amount]
        ).await {
            Err(err) => return Err(format!("failed to update credits row for user {} ({}): {}", user_id, amount, err)),
            Ok(rows) => {
                match rows.iter().next() {
                    None => return Err(format!("no row found")),
                    Some(row) => {
                        let id: Uuid = row.get(0);
                        let user_id: Uuid = row.get(1);
                        let balance: i32 = row.get(2);
                        Credits {
                            id: id.to_string(),
                            user_id: user_id.to_string(),
                            balance: balance as u32,
                        }
                    }
                }
            }
        };

        // add history
        let delta = -parsed_amount;
        let parsed_caused = cause.clone();
        match db_client.query(
            "INSERT INTO credits_history (user_id, delta, cause) VALUES ($1, $2, $3)",
            &[&parsed_uuid, &delta, &parsed_caused]
        ).await {
            Err(err) => return Err(format!("db query err: {}", err)),
            Ok(_) => Ok(credits)
        }
    }

    async fn get_credits_by_user_id(&self, user_id: String) -> Result<Credits, String> {

        let db_client = self.db_client.lock().await;

        let parsed_uuid = match Uuid::parse_str(user_id.as_str()) {
            Err(err) => return Err(format!("UUID parse err: {}", err.to_string())),
            Ok(result) => result,
        };

        let credits = match db_client.query(
            "SELECT id, user_id, balance FROM credits WHERE user_id = $1",
            &[&parsed_uuid]
        ).await {
            Err(err) => return Err(err.to_string()),
            Ok(query_result) => {
                match query_result.iter().next() {
                    None => return Err(format!("no credits record found")),
                    Some(row) => {
                        let id: Uuid = row.get(0);
                        let user_id: Uuid = row.get(1);
                        let balance: i32 = row.get(2);
                        Credits {
                            id: id.to_string(),
                            user_id: user_id.to_string(),
                            balance: balance as u32,
                        }
                    }
                }
            }
        };

        Ok(credits)
    }
}
