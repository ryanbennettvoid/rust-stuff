#[derive(Debug)]
pub struct TopupArgs {
    pub user_id: String,
    pub amount: u32,
    pub cause: String
}

#[derive(Debug)]
pub struct TopupResult {
    pub user_id: String,
    pub balance: u32,
}
