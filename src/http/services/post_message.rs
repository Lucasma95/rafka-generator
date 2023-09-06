use actix_web::{ post, web::{Json, Data}, HttpResponse, Responder};
use crate::use_cases::log_request::RequestLogger;
use serde_json::json;

use crate::http::contracts::post_message::Request;

#[post("/echo")]
pub async fn echo(request: Json<Request>, logger: Data<dyn RequestLogger>) -> impl Responder {
    let result = logger.log(&request);

    match result {
        Ok(_) => {
            return HttpResponse::Ok().json(json!(request))
        }
        Err(_) => {
            return HttpResponse::InternalServerError().json(json!(request));

        }
    }
}
