#[derive(Debug)]
pub struct ConsumeArgs {
    pub user_id: String,
    pub amount: u32,
    pub cause: String
}

#[derive(Debug)]
pub struct ConsumeResult {
    pub user_id: String,
    pub balance: u32,
}
