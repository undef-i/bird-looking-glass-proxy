mod auth;
mod config;
mod route;
use actix_web::{web, App, HttpServer};
use clap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref GLOBAL_CONFIG: config::Config = {
        let matches = clap::App::new("bird-looking-glass-proxy")
            .bin_name("bird-looking-glass-proxy")
            .arg_from_usage("-c, --config=[FILE] 'Set the config file'")
            .arg_from_usage("-e, --example 'Export sample config file'")
            .get_matches();
        if matches.is_present("example") {
            println!("bind_ip = \"0.0.0.0\"\nbind_port = 8000\n\n# Used to restrict access to bird-looking-glass-proxy based on source IP address.\n# Empty list = any IP is allowed to run queries.\naccess_list = [\"127.0.0.1\"]\n\n# Used as source address when running traceroute\nipv4_source=\"198.51.100.42\"\nipv6_source=\"2001:db8:42::1\"\n\nbird_socket=\"/var/run/bird/bird.ctl\"\nbird6_socket=\"/var/run/bird/bird6.ctl\"");
            std::process::exit(0);
        }
        config::Config::new(matches.value_of("config").unwrap_or("config.toml"))
    };
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(auth::Auth)
            .route(
                "/traceroute6",
                web::get().to(route::traceroute::traceroute6),
            )
            .route("/traceroute", web::get().to(route::traceroute::traceroute4))
            .route("/bird6", web::get().to(route::bird::bird6))
            .route("/bird", web::get().to(route::bird::bird4))
    })
    .bind(format!(
        "{}:{}",
        GLOBAL_CONFIG.bind_ip, GLOBAL_CONFIG.bind_port
    ))?
    .run()
    .await
}
