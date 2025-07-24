use reqwest::Client;
use std::env;

#[derive(Clone)]
pub struct PaymentProcessorUrl {
    pub default: String,
    pub fallback: String,
}

#[derive(Clone)]
pub struct AppState {
    pub payment_processor_url: PaymentProcessorUrl,
    pub http_client: Client,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            payment_processor_url: PaymentProcessorUrl {
                default: env::var("PAYMENT_PROCESSOR_DEFAULT_URL").unwrap(),
                fallback: env::var("PAYMENT_PROCESSOR_FALLBACK_URL").unwrap(),
            },
            http_client: Client::new(),
        }
    }
}
