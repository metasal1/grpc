use tonic::{transport::Server, Request, Response, Status};

use payments::solana_server::{Solana, SolanaServer};
use payments::{SolPaymentResponse, SolPaymentRequest};

pub mod payments {
    tonic::include_proto!("payments");
}

#[derive(Debug, Default)]
pub struct SolanaService {}

#[tonic::async_trait]
impl Solana for SolanaService {
    async fn send_payment(
        &self,
        request: Request<SolPaymentRequest>,
    ) -> Result<Response<SolPaymentResponse>, Status> {
        println!("Got request: {:?}", request);

        let req = request.into_inner();

        let reply = SolPaymentResponse {
            successful: true,
            message: format!("Sent {}BTC to {}.", req.amount, req.to_addr).into(),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let btc_service = SolanaService::default();

    Server::builder()
        .add_service(SolanaServer::new(sol_service))
        .serve(addr)
        .await?;

    Ok(())
}
