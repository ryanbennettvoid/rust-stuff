#[derive(Debug)]
pub struct GetBalanceArgs {
    pub user_id: String,
}

#[derive(Debug)]
pub struct GetBalanceResult {
    pub user_id: String,
    pub balance: u32,
}
