use crate::server_builder::SwapContext;
use helloworld::SwapInstruction;
use log::{error, info};
use std::convert::Infallible;
use std::sync::Arc;
use warp::http::StatusCode;
use warp::Reply;

pub async fn handle_swap(
    context: Arc<SwapContext>,
    instruct: SwapInstruction,
) -> std::result::Result<impl Reply, Infallible> {
    let player = &context.player;
    let program = &context.program;
    let connection = &context.connection;
    let res = crate::client::swap_token(&player, &program, &connection, instruct.amount);
    match res {
        Ok(_) => {
            info!(
                "({}) greetings have been sent.",
                crate::client::count_greetings(&player, &program, &connection).unwrap()
            );
        }
        Err(err) => {
            error!("swap_token Error: {:?}", err);
        }
    }

    let message = "Request swap received";
    Ok(warp::reply::with_status(message, StatusCode::OK))
}
