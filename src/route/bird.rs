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
    return Ok(messages.iter().map(|message |format!("{:?}",message)).collect::<String>());

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