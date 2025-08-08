use r2d2::Pool;
use redis::Client;
use reqwest::Client as ReqwestClient;
use std::{env, sync::Arc};

#[derive(Clone)]
pub struct PaymentProcessorsRoutes {
    pub default: String,
    pub fallback: String,
}

#[derive(Clone)]
pub struct AppState {
    pub payment_processors_routes: PaymentProcessorsRoutes,
    pub http_client: ReqwestClient,
    pub redis_connection_pool: Pool<Client>,
}

impl AppState {
    pub fn new() -> Self {
        let client = Client::open(env::var("REDIS_URL").unwrap()).unwrap();

        let pool = Pool::builder().build(client).unwrap();

        Self {
            payment_processors_routes: PaymentProcessorsRoutes {
                default: env::var("PAYMENT_PROCESSOR_DEFAULT_URL").unwrap() + "/payments",
                fallback: env::var("PAYMENT_PROCESSOR_FALLBACK_URL").unwrap() + "/payments",
            },
            http_client: ReqwestClient::new(),
            redis_connection_pool: pool,
        }
    }
}

pub type SharedState = Arc<AppState>;
