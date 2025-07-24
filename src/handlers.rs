use crate::{payment_processor, payments_dto::PaymentDTO, state::AppState};
use axum::extract::{Json, State};

pub async fn payments(State(app_state): State<AppState>, Json(payload): Json<PaymentDTO>) {
    if let Err(e) = payment_processor::post(
        &app_state.payment_processor_url.default,
        &app_state.http_client,
        &payload,
    )
    .await
    {
        println!("Payment processor default error: {}", e);

        if let Err(e) = payment_processor::post(
            &app_state.payment_processor_url.fallback,
            &app_state.http_client,
            &payload,
        )
        .await
        {
            println!("Payment processor fallback error: {}", e);
        }
    }
}

pub async fn payments_summary() {}
