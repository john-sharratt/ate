#[allow(unused_imports)]
use log::{info, error, debug};
use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use std::sync::Arc;

use ate::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Ping
{
    msg: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Pong
{
    msg: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct PingError
{
}

#[derive(Default)]
struct PingPongTable
{        
}

#[async_trait]
impl ServiceHandler<Ping, Pong, PingError>
for PingPongTable
{
    async fn process<'a>(&self, ping: Ping, _context: InvocationContext<'a>) -> Result<Pong, ServiceError<PingError>>
    {
        Ok(Pong { msg: ping.msg })
    }
}

#[tokio::main]
async fn main() -> Result<(), AteError>
{
    env_logger::init();

    debug!("creating test chain");

    // Create the chain with a public/private key to protect its integrity
    let conf = ConfAte::default();
    let builder = ChainBuilder::new(&conf).await.build();
    let chain = builder.open(&ChainKey::from("cmd")).await?;
    
    debug!("start the service on the chain");
    let session = AteSession::new(&conf);
    chain.add_service(session.clone(), Arc::new(PingPongTable::default()));
    
    debug!("sending ping");
    let pong: Result<Pong, InvokeError<PingError>> = chain.invoke(Ping {
        msg: "hi".to_string()
    }).await;

    debug!("received pong with msg [{}]", pong?.msg);
    Ok(())
}