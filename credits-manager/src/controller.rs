use crate::actions::consume::{ConsumeArgs, ConsumeResult};
use crate::actions::get_balance::{GetBalanceArgs, GetBalanceResult};
use crate::dao::{DAOInterface, DAO};
use crate::{TopupArgs, TopupResult};

#[tonic::async_trait]
pub trait ControllerInterface {
    async fn topup(&self, req: TopupArgs) -> Result<TopupResult, String>;
    async fn consume(&self, req: ConsumeArgs) -> Result<ConsumeResult, String>;
    async fn get_balance(&self, req: GetBalanceArgs) -> Result<GetBalanceResult, String>;
}

pub struct Controller {
    dao: DAO,
}

impl Controller {
    pub fn new(dao: DAO) -> Self {
        Controller { dao }
    }
}

#[tonic::async_trait]
impl ControllerInterface for Controller {
    async fn topup(&self, req: TopupArgs) -> Result<TopupResult, String> {
        match self
            .dao
            .topup_credits_by_user_id(req.user_id, req.amount, req.cause)
            .await
        {
            Err(err) => return Err(err),
            Ok(credits) => Ok(TopupResult {
                user_id: credits.user_id,
                balance: credits.balance,
            }),
        }
    }

    async fn consume(&self, req: ConsumeArgs) -> Result<ConsumeResult, String> {
        match self
            .dao
            .consume_credits_by_user_id(req.user_id, req.amount, req.cause)
            .await
        {
            Err(err) => return Err(err),
            Ok(credits) => Ok(ConsumeResult {
                user_id: credits.user_id,
                balance: credits.balance,
            }),
        }
    }

    async fn get_balance(&self, req: GetBalanceArgs) -> Result<GetBalanceResult, String> {
        match self
            .dao
            .get_credits_by_user_id(req.user_id)
            .await
        {
            Err(err) => return Err(err),
            Ok(credits) => Ok(GetBalanceResult {
                user_id: credits.user_id,
                balance: credits.balance,
            }),
        }
    }
}
