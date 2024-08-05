use salvo::proxy::Proxy;
use salvo::{async_trait, Depot, FlowCtrl, Handler, Request, Response};
use tracing::instrument::WithSubscriber;

use crate::read_config::GatewayService;

pub struct GatewayAuth {
    service: GatewayService,
}

impl GatewayAuth {
    pub fn new(service: GatewayService) -> Self {
        Self { service }
    }
}

#[async_trait]
impl Handler for GatewayAuth {
    async fn handle(
        &self,
        req: &mut Request,
        depot: &mut Depot,
        res: &mut Response,
        ctrl: &mut FlowCtrl,
    ) {
        let request_url = format!(
            "{}:{}",
            self.service.target_service, self.service.target_port,
        );

        Proxy::use_reqwest_client(request_url)
            .handle(req, depot, res, ctrl)
            .with_current_subscriber()
            .await;
    }
}
