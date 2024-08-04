use reqwest::Client;
use std::{error::Error, sync::Arc};

use crate::{
    gateway_auth::GatewayAuth, gateway_finance::GatewayFinance, gateway_sales::GatewaySales,
    gateway_storage::GatewayStorage, gateway_t::ApiGatewayService, read_config::GatewayService,
};

pub enum GatewayType {
    Auth(GatewayAuth),
    Finance(GatewayFinance),
    Sales(GatewaySales),
    Storage(GatewayStorage),
}

pub fn create_gateway(
    gateway_type: String,
    service: GatewayService,
    client: Arc<Client>,
) -> Result<GatewayType, Box<dyn Error>> {
    let client = Arc::clone(&client);

    match gateway_type.as_str() {
        "auth" => Ok(GatewayType::Auth(GatewayAuth::new(client, service))),
        "finance" => Ok(GatewayType::Finance(GatewayFinance::new(client, service))),
        "sales" => Ok(GatewayType::Sales(GatewaySales::new(client, service))),
        "storage" => Ok(GatewayType::Storage(GatewayStorage::new(client, service))),
        _ => Err(From::from("Could not find a proper gateway")),
    }
}
