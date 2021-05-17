use actix_web::{HttpRequest, HttpResponse};
use serde::Serialize;

#[derive(Serialize)]
struct Response {
    ip: String,
    forwarded_for: Vec<String>,
}

pub async fn ip(r: HttpRequest) -> HttpResponse {
    let headers = r.headers().get_all("X-Forwarded-For");
    let mut headers_vec:Vec<String> = Vec::new();

    for h in headers.into_iter() {
        match h.to_str() {
            Ok(v) => headers_vec.push(v.to_string()),
            Err(_) => return HttpResponse::BadRequest().finish(),
        }
    }

    let a = match r.peer_addr() {
        Some(a)  => a.ip().to_string(),
        None => String::from(""),
    };

    HttpResponse::Ok().json(Response {
        ip: a,
        forwarded_for: headers_vec,
    })
}