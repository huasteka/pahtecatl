use reqwest::Client;
use salvo::{async_trait, Depot, FlowCtrl, Handler, Request, Response};
use std::sync::Arc;

use crate::gateway_t::ApiGatewayService;
use crate::read_config::GatewayService;
use crate::redirect_to::redirect_to;

pub struct GatewayAuth {
    client: Arc<Client>,
    service: GatewayService,
}

impl ApiGatewayService for GatewayAuth {
    fn new(client: Arc<Client>, service: GatewayService) -> Self {
        Self { client, service }
    }
}

#[async_trait]
impl Handler for GatewayAuth {
    async fn handle(
        &self,
        req: &mut Request,
        _depot: &mut Depot,
        res: &mut Response,
        _ctrl: &mut FlowCtrl,
    ) {
        redirect_to(self.client.clone(), self.service.clone(), req, res).await
    }
}
