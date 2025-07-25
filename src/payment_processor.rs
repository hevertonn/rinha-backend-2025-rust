use crate::payments_dto::PaymentDTO;
use reqwest::Client;

pub async fn post(
    payment_processor_route: &String,
    http_client: &Client,
    payment: &PaymentDTO,
) -> Result<(), reqwest::Error> {
    http_client
        .post(payment_processor_route)
        .json(payment)
        .send()
        .await?;

    Ok(())
}
