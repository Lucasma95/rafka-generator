pub mod http;
pub mod use_cases;
use actix_web::{get, web::Data, App, HttpResponse, HttpServer, Responder};
use dotenvy::dotenv;
use std::{env, sync::Arc};
use use_cases::log_request::RequestLogger;


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    dotenv().ok();
    let port = define_port();

    let request_logger_impl = use_cases::log_request::RequestLoggerImpl::new();
    let log_arc = Arc::new(request_logger_impl) as Arc<dyn RequestLogger>;

    HttpServer::new(move || {
        App::new()
            .service(hello)
            .service(http::services::post_message::echo)
            .app_data(Data::from(log_arc.clone()))
            //.route("/echo", web::post().to(http::services::post_message::echo))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}

fn define_port() -> u16 {
    env::var("PORT")
        .ok()
        .and_then(|port| port.parse().ok())
        .unwrap_or(8080)
}

/*fn define_port_long() -> u16 {
    let port = env::var("PORT");
    match port {
        Ok(port) => {
            let parsed_port: Result<u16, _> = port.parse();
            match parsed_port {
                Ok(value) => value,
                Err(_) => 8080,
            }
        }
        Err(_) => 8080,
    }
}*/
