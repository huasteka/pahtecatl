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
    let source = req.param::<String>("**rest_path").unwrap();
    let request_url = format!(
        "{}:{}/{}",
        service.target_service, service.target_port, source
    );

    let mut request_builder = client
        .request(req.method().clone(), Url::parse(&request_url).unwrap())
        .headers(req.headers().clone());

    if !req.body().is_none() {
        let request_payload = req.payload().await.unwrap();
        let request_body = Body::from(request_payload.clone());

        request_builder = request_builder.body(request_body);
    }

    match request_builder.send().await {
        Ok(response) => {
            let status = response.status();
            let headers = response.headers().clone();
            let body = response.bytes().await.unwrap();

            res.status_code(status);
            res.headers_mut().extend(headers);
            res.write_body(body).unwrap();
        }
        Err(_response) => {
            res.status_code(StatusCode::BAD_GATEWAY);
        }
    }
}
