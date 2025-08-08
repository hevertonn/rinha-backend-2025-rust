use crate::{
    payments_dto::{PaymentDTO, PaymentProcessor, PaymentSummaryDTO},
    state::SharedState,
};
use axum::extract::{Json, State};
use redis::Commands;

pub async fn payments(State(shared_state): State<SharedState>, Json(payload): Json<PaymentDTO>) {
    let mut conn = shared_state.redis_connection_pool.get().unwrap();

    let _: () = conn
        .lpush(
            "payments:orders",
            payload.correlation_id + ":" + &payload.amount.to_string(),
        )
        .unwrap();
}

pub async fn payments_summary(State(shared_state): State<SharedState>) -> Json<PaymentSummaryDTO> {
    let mut conn = shared_state.redis_connection_pool.get().unwrap();

    let fields = vec!["total_requests", "total_amount"];

    let result: Vec<Vec<String>> = redis::pipe()
        .cmd("HMGET")
        .arg("payments:summary:default")
        .arg(&fields)
        .cmd("HMGET")
        .arg("payments:summary:fallback")
        .arg(&fields)
        .query(&mut conn)
        .unwrap();

    Json(PaymentSummaryDTO {
        payment_processor_default: PaymentProcessor {
            total_requests: result[0][0].parse().unwrap(),
            total_amount: result[0][1].parse().unwrap(),
        },
        payment_processor_fallback: PaymentProcessor {
            total_requests: result[1][0].parse().unwrap(),
            total_amount: result[1][1].parse().unwrap(),
        },
    })
}
