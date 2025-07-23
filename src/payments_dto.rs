use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct PaymentDTO {
    #[serde(rename = "correlationId")]
    correlation_id: String,
    amount: f32,
}

#[derive(Serialize)]
pub struct PaymentProcessor {
    #[serde(rename = "totalRequests")]
    total_requests: String,
    #[serde(rename = "totalAmount")]
    total_amount: f32,
}

#[derive(Serialize)]
pub struct PaymentSummaryDTO {
    #[serde(rename = "default")]
    payment_processor_default: PaymentProcessor,
    #[serde(rename = "fallback")]
    payment_processor_fallback: PaymentProcessor,
}
