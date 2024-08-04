use reqwest::Client;
use std::sync::Arc;

use crate::read_config::GatewayService;

pub trait ApiGatewayService {
  fn new(client: Arc<Client>, service: GatewayService) -> Self;
}
