use crate::actions::topup::{TopupArgs, TopupResult};
use crate::controller::{Controller, ControllerInterface};
use crate::credits_manager_svc::credits_manager_server::{CreditsManager, CreditsManagerServer};
use crate::credits_manager_svc::{
    ConsumeRequest, ConsumeResponse, GetBalanceRequest, GetBalanceResponse, TopupRequest,
    TopupResponse,
};
use crate::dao::DAO;
use std::env;
use tokio::sync::Mutex;
use tokio_postgres::NoTls;
use tonic::{transport::Server, Request, Response, Status};
use crate::actions::consume::{ConsumeArgs};
use crate::actions::get_balance::GetBalanceArgs;

mod actions;
mod controller;
mod credits_manager_svc;
mod dao;

pub struct ServerRoutes {
    controller: Controller,
}

impl ServerRoutes {
    fn new(controller: Controller) -> Self {
        ServerRoutes { controller }
    }
}

#[tonic::async_trait]
impl CreditsManager for ServerRoutes {
    async fn topup(
        &self,
        request: Request<TopupRequest>,
    ) -> Result<Response<TopupResponse>, Status> {
        let req = request.get_ref();
        match self
            .controller
            .topup(TopupArgs {
                user_id: req.user_id.clone(),
                amount: req.amount,
                cause: req.cause.clone()
            })
            .await {
            Err(err) => return Err(Status::internal(err)),
            Ok(result) => {
                Ok(Response::new(TopupResponse {
                    user_id: result.user_id,
                    balance: result.balance
                }))
            }
        }
    }

    async fn consume(
        &self,
        request: Request<ConsumeRequest>,
    ) -> Result<Response<ConsumeResponse>, Status> {
        let req = request.get_ref();
        match self.controller.consume(ConsumeArgs {
            user_id: req.user_id.clone(),
            amount: req.amount,
            cause: req.cause.clone()
        }).await {
            Err(err) => return Err(Status::internal(err)),
            Ok(result) => {
                Ok(Response::new(ConsumeResponse {
                    user_id: result.user_id,
                    balance: result.balance
                }))
            }
        }
    }

    async fn get_balance(
        &self,
        request: Request<GetBalanceRequest>,
    ) -> Result<Response<GetBalanceResponse>, Status> {
        let req = request.get_ref();
        match self.controller.get_balance(GetBalanceArgs {
            user_id: req.user_id.clone()
        }).await {
            Err(err) => return Err(Status::internal(err)),
            Ok(result) => {
                Ok(Response::new(GetBalanceResponse {
                    user_id: result.user_id,
                    balance: result.balance
                }))
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = format!("0.0.0.0:{}", env::var("PORT").unwrap())
        .to_string()
        .parse()
        .unwrap();

    println!("server listening at {}", address);

    let (db_client, connection) = tokio_postgres::connect(
        "postgresql://postgres:admin@postgres/postgres?sslmode=disable",
        NoTls,
    )
    .await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            panic!("postgres connection err: {}", e);
        }
    });

    let wrapped_db_client = Mutex::new(db_client);
    let dao = DAO::new(wrapped_db_client);
    let controller = Controller::new(dao);
    let routes = ServerRoutes::new(controller);
    let server = CreditsManagerServer::new(routes);
    let service = tonic_web::config().enable(server);

    Server::builder()
        .accept_http1(true)
        .add_service(service)
        .serve(address)
        .await?;

    Ok(())
}
