mod gateway_auth;
mod create_gateway;
mod gateway_finance;
mod gateway_sales;
mod gateway_storage;
mod gateway_t;
mod listen_shutdown;
mod read_config;
mod redirect_to;

use listen_shutdown::listen_shutdown;
use reqwest::Client;
use salvo::cors::{AllowOrigin, Cors};
use salvo::http::Method;
use salvo::logging::Logger;
use salvo::prelude::*;
use std::sync::Arc;

use create_gateway::create_gateway;
use create_gateway::GatewayType::{Auth, Finance, Sales, Storage};
use read_config::GatewayConfig;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let cors = Cors::new()
        .allow_origin(AllowOrigin::any())
        .allow_methods(vec![
            Method::OPTIONS,
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
        ])
        .allow_headers(vec!["authorization", "content-type"])
        .into_handler();

    let mut router = Router::new();
    let gateway_config = GatewayConfig::new().unwrap();

    let client = Arc::new(Client::new());
    for (service_type, service) in gateway_config.proxies {
        let client = client.clone();
        let client_path = format!("{}/<**rest_path>", service_type);
        let gateway_type = create_gateway(service_type.clone(), service, client).unwrap();

        let route = match gateway_type {
            Auth(proxy) => Router::with_path(client_path).goal(proxy),
            Finance(proxy) => Router::with_path(client_path).goal(proxy),
            Sales(proxy) => Router::with_path(client_path).goal(proxy),
            Storage(proxy) => Router::with_path(client_path).goal(proxy),
        };

        router = router.push(route);
    }

    let service = Service::new(router).hoop(cors).hoop(Logger::new());
    let listener = TcpListener::new("0.0.0.0:9705").bind().await;

    let server = Server::new(listener);
    let server_handler = server.handle();
    tokio::spawn(listen_shutdown(server_handler));
    server.serve(service).await;
}
