use reqwest::Client;
use std::env;

#[derive(Clone)]
pub struct PaymentProcessorsRoutes {
    pub default: String,
    pub fallback: String,
}

#[derive(Clone)]
pub struct AppState {
    pub payment_processors_routes: PaymentProcessorsRoutes,
    pub http_client: Client,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            payment_processors_routes: PaymentProcessorsRoutes {
                default: env::var("PAYMENT_PROCESSOR_DEFAULT_URL").unwrap() + "/payments",
                fallback: env::var("PAYMENT_PROCESSOR_FALLBACK_URL").unwrap() + "/payments",
            },
            http_client: Client::new(),
        }
    }
}
