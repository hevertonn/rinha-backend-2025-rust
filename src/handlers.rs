use crate::payments_dto::PaymentDTO;
use axum::Json;

pub async fn payments(Json(payload): Json<PaymentDTO>) {}

pub async fn payments_summary() {}
