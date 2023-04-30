use birdc::*;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde_derive::Deserialize;
use crate::GLOBAL_CONFIG;
use futures::executor::block_on;

#[derive(Deserialize, Debug)]
pub struct Param {
    q: String,
}
#[tokio::main]
pub async fn bird(_param: web::Query<Param>, _req: HttpRequest, mode: i8) -> Result<String> {
    let client = Client::for_unix_socket(match mode{
        6 => GLOBAL_CONFIG.bird6_socket.as_str(),
        _ => GLOBAL_CONFIG.bird_socket.as_str(),
    });
    let mut connection = block_on(client.connect())?;
    let messages = block_on(connection.send_request(&_param.q))?;
    return Ok(messages.iter().map(|message | match message {
        Message::Welcome(s) |
        Message::ReadingConfiguration(s) |
        Message::Reconfigured(s) |
        Message::ReconfigurationInProgress(s) |
        Message::ReconfigurationQueued(s) |
        Message::ReconfigurationIgnoredShutdown(s) |
        Message::ShutdownOrdered(s) |
        Message::AlreadyDisable(s) |
        Message::Disabled(s) |
        Message::AlreadyEnabled(s) |
        Message::Enabled(s) |
        Message::Restarted(s) |
        Message::StatusReport(s) |
        Message::RouteCount(s) |
        Message::Reloading(s) |
        Message::AccessRestricted(s) |
        Message::ReconfigurationUnQueued(s) |
        Message::ReconfigurationConfirmed(s) |
        Message::NothingToDo(s) |
        Message::ConfigurationOk(s) |
        Message::UndoRequested(s) |
        Message::UndoScheduled(s) |
        Message::EvaluatingExpression(s) |
        Message::GracefulRestartStatus(s) |
        Message::BirdVersion(s) |
        Message::InterfaceList(s) |
        Message::ProtocolList(s) |
        Message::InterfaceAddress(s) |
        Message::InterfaceFlags(s) |
        Message::InterfaceSummary(s) |
        Message::ProtocolDetails(s) |
        Message::RouteList(s) |
        Message::RouteDetails(s) |
        Message::StaticRouteList(s) |
        Message::SymbolList(s) |
        Message::Uptime(s) |
        Message::RouteExtendedAttributeList(s) |
        Message::OspfNeighbors(s) |
        Message::Ospf(s) |
        Message::OspfInterface(s) |
        Message::OspfState(s) |
        Message::OspfLsadb(s) |
        Message::Memory(s) |
        Message::RoaList(s) |
        Message::BfdSessions(s) |
        Message::RipInterfaces(s) |
        Message::RipNeighbors(s) |
        Message::BabelInterfaces(s) |
        Message::BabelNeighbors(s) |
        Message::BabelEntries(s) |
        Message::ProtocolListHeader(s) |
        Message::InterfaceSummaryHeader(s) |
        Message::TableHeader(_, s) |
        Message::ReplyTooLong(s) |
        Message::RouteNotFound(s) |
        Message::ConfigurationFileError(s) |
        Message::NoProtocolsMatch(s) |
        Message::StoppedDueToReconfiguration(s) |
        Message::ProtocolDown(s) |
        Message::ReloadFailed(s) |
        Message::AccessDenied(s) |
        Message::RuntimeError(_, s) |
        Message::CommandTooLong(s) |
        Message::ParseError(s) |
        Message::InvalidSymbol(s) |
        Message::ClientError(_, s) |
        Message::Unknown(_, s) => s.to_string(),
        _ => "".to_string(),
    }).collect::<Vec<String>>().join("\n"));

}
pub async fn bird4(_param: web::Query<Param>, _req: HttpRequest) -> impl Responder {
    match bird(_param, _req, 4) {
        Ok(response) => HttpResponse::Ok().body(response),
        Err(_) => HttpResponse::Ok().body("Error".to_string()),
    }
}

pub async fn bird6(_param: web::Query<Param>, _req: HttpRequest) -> impl Responder {
    match bird(_param, _req, 6) {
        Ok(response) => HttpResponse::Ok().body(response),
        Err(_) => HttpResponse::Ok().body("Error".to_string()),
    }
}