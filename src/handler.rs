use actix_web::{HttpResponse, Responder, Error, HttpRequest};
use futures::future::{ready, Ready};
use serde::Serialize;
use serde_json;


#[derive(Serialize)]
struct Response {
    ip: String,
    forwarded_for: Vec<String>,
}

impl Responder for Response {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}


pub async fn ip(r: HttpRequest) -> impl Responder {
    let headers = r.headers().get_all("X-Forwarded-For");
    let mut headers_vec:Vec<String> = Vec::new();

    for h in headers.into_iter() {
        match h.to_str() {
            Ok(v) => headers_vec.push(v.to_string()),
            Err(_) => continue,
        }
    }

    let a = match r.peer_addr() {
        Some(a)  => a.ip().to_string(),
        None => String::from(""),
    };

    Response {
        ip: a,
        forwarded_for: headers_vec,
    }
}