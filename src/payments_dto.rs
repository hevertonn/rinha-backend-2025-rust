use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct PaymentDTO {
    #[serde(rename = "correlationId")]
    pub correlation_id: String,
    pub amount: f32,
}

#[derive(Serialize)]
pub struct PaymentProcessor {
    #[serde(rename = "totalRequests")]
    pub total_requests: i32,
    #[serde(rename = "totalAmount")]
    pub total_amount: f32,
}

#[derive(Serialize)]
pub struct PaymentSummaryDTO {
    #[serde(rename = "default")]
    pub payment_processor_default: PaymentProcessor,
    #[serde(rename = "fallback")]
    pub payment_processor_fallback: PaymentProcessor,
}
