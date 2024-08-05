use reqwest::{Body, Client, StatusCode, Url};
use salvo::{Request, Response};
use std::sync::Arc;

use crate::read_config::GatewayService;

pub async fn redirect_to(
    client: Arc<Client>,
    service: GatewayService,
    req: &mut Request,
    res: &mut Response,
) {
    let request_path = req.param::<String>("**rest_path").unwrap();
    let request_url = format!(
        "{}:{}/{}",
        service.target_service, service.target_port, request_path,
    );
    let url = Url::parse(&request_url).unwrap();

    tracing::debug!("Sending request to {:#?}", url);

    let mut request_builder = client
        .request(req.method().clone(), url)
        .headers(req.headers().clone());

    if !req.body().is_none() {
        let request_payload = req.payload().await.unwrap();
        let request_body = Body::from(request_payload.clone());

        tracing::debug!(
            "Redirecting request body to new destination: {:#?}",
            request_body
        );

        request_builder = request_builder.body(request_body);
    }

    match request_builder.send().await {
        Ok(response) => {
            tracing::info!("Service request was successful");

            let status = response.status();
            let headers = response.headers().clone();
            let body = response.bytes().await.unwrap();

            res.status_code(status);
            res.headers_mut().extend(headers);
            res.write_body(body).unwrap();
        }
        Err(err) => {
            tracing::error!("Response error: {err}");

            res.status_code(StatusCode::BAD_GATEWAY);
        }
    }
}
