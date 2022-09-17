use log::{info, trace};
use serde::{Deserialize, Serialize};

use std::convert::Infallible;
use std::net::SocketAddr;
use warp::http::{HeaderMap, Method};

use crate::handle_swap::handle_swap;
use helloworld::SwapInstruction;
use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::Keypair;
use std::default::Default;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::{http::StatusCode, Filter, Rejection, Reply};
pub const MAX_JSON_BODY_SIZE: u64 = 1024 * 1024;

pub struct SwapContext {
    pub(crate) player: Keypair,
    pub(crate) program: Keypair,
    pub(crate) connection: RpcClient,
}

impl SwapContext {
    pub fn new() -> SwapContext {
        let args = std::env::args().collect::<Vec<_>>();
        if args.len() != 2 {
            eprintln!(
                "usage: {} <path to solana hello world example swap_program keypair>",
                args[0]
            );
            std::process::exit(-1);
        }
        let keypair_path = &args[1];

        let connection = crate::client::establish_connection().unwrap();
        println!(
            "Connected to remote solana node running version ({}).",
            connection.get_version().unwrap()
        );

        let balance_requirement = crate::client::get_balance_requirement(&connection).unwrap();
        println!(
            "({}) lamports are required for this transaction.",
            balance_requirement
        );

        let player = crate::utils::get_player().unwrap();
        let player_balance = crate::client::get_player_balance(&player, &connection).unwrap();
        println!("({}) lamports are owned by player.", player_balance);

        if player_balance < balance_requirement {
            let request = balance_requirement - player_balance;
            println!(
                "Player does not own sufficent lamports. Airdropping ({}) lamports.",
                request
            );
            crate::client::request_airdrop(&player, &connection, request).unwrap();
        }

        let program = crate::client::get_program(keypair_path, &connection).unwrap();

        crate::client::create_greeting_account(&player, &program, &connection).unwrap();
        SwapContext {
            player,
            program,
            connection,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AccessControl {
    pub access_control_allow_headers: String,
    pub access_control_allow_origin: String,
    pub access_control_allow_methods: String,
    pub content_type: String,
}

impl Default for AccessControl {
    fn default() -> Self {
        AccessControl {
            access_control_allow_headers:
                "Content-Type, User-Agent, Authorization, Access-Control-Allow-Origin".to_string(),
            access_control_allow_origin: "*".to_string(),
            access_control_allow_methods: "text/html".to_string(),
            content_type: "application/json".to_string(),
        }
    }
}

impl AccessControl {
    pub fn get_access_control_allow_headers(&self) -> Vec<String> {
        self.access_control_allow_headers
            .split(",")
            .into_iter()
            .map(|header| header.replace(" ", ""))
            .collect()
    }
}

#[derive(Default)]
pub struct WebServerBuilder {
    entry_point: String,
    access_control: AccessControl,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SimpleResponse {
    success: bool,
}
pub struct WorkerServer {
    access_control: AccessControl,
    entry_point: String,
    context: Arc<SwapContext>,
}

impl WorkerServer {
    pub fn builder() -> WebServerBuilder {
        WebServerBuilder::default()
    }
    pub async fn serve(&self) {
        let allow_headers: Vec<String> = self.access_control.get_access_control_allow_headers();
        info!("allow_headers: {:?}", allow_headers);
        let cors = warp::cors()
            .allow_any_origin()
            .allow_headers(allow_headers)
            .allow_methods(&[
                Method::GET,
                Method::POST,
                Method::PUT,
                Method::PATCH,
                Method::DELETE,
                Method::OPTIONS,
                Method::HEAD,
            ]);
        info!("cors: {:?}", cors);
        let router = self
            .create_ping()
            .with(&cors)
            //.create_get_status(self.scheduler_service.clone()).await.with(&cors)
            .or(self.create_route_swap(self.context.clone()).with(&cors))
            .recover(handle_rejection);

        info!("entry_point: {:?}", self.entry_point);
        let socket_addr: SocketAddr = self.entry_point.parse().unwrap();

        warp::serve(router).run(socket_addr).await;
    }

    /// Ping API
    fn create_ping(
        &self,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("ping")
            .and(warp::get())
            .and_then(move || async move {
                info!("Receive ping request");
                Self::simple_response(true).await
            })
    }
    pub(crate) async fn simple_response(success: bool) -> Result<impl Reply, Rejection> {
        let res = SimpleResponse { success };
        Ok(warp::reply::json(&res))
    }
    fn create_route_swap(
        &self,
        context: Arc<SwapContext>,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("swap")
            .and(WorkerServer::log_headers())
            .and(warp::post())
            .and(warp::body::content_length_limit(MAX_JSON_BODY_SIZE).and(warp::body::json()))
            .and_then(move |instruct: SwapInstruction| {
                let context = context.clone();
                info!("#### Received instruction {:?}  ####", &instruct);
                async move { handle_swap(context, instruct).await }
            })
    }

    fn log_headers() -> impl Filter<Extract = (), Error = Infallible> + Copy {
        warp::header::headers_cloned()
            .map(|headers: HeaderMap| {
                trace!("#### Received request header ####");
                for (k, v) in headers.iter() {
                    // Error from `to_str` should be handled properly
                    trace!(
                        "{}: {}",
                        k,
                        v.to_str().expect("Failed to print header value")
                    )
                }
            })
            .untuple_one()
    }
}

impl WebServerBuilder {
    pub fn build(self) -> WorkerServer {
        WorkerServer {
            access_control: self.access_control,
            entry_point: self.entry_point,
            context: Arc::new(SwapContext::new()),
        }
    }
    pub fn with_entry_point(mut self, entry_point: &str) -> Self {
        self.entry_point = String::from(entry_point);
        self
    }
    pub fn with_access_control(mut self, access_control: AccessControl) -> Self {
        self.access_control = access_control;
        self
    }
}

async fn handle_rejection(err: Rejection) -> std::result::Result<impl Reply, Infallible> {
    let (code, message) = if err.is_not_found() {
        (StatusCode::NOT_FOUND, "Not Found".to_string())
    } else if err.find::<warp::reject::PayloadTooLarge>().is_some() {
        (StatusCode::BAD_REQUEST, "Payload too large".to_string())
    } else if err.find::<warp::reject::MethodNotAllowed>().is_some() {
        (
            StatusCode::BAD_REQUEST,
            format!("Authorization error, {:?}", err),
        )
    } else {
        eprintln!("unhandled error: {:?}", err);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal Server Error".to_string(),
        )
    };

    Ok(warp::reply::with_status(message, code))
}
