use actix_web::HttpResponse;
use maud::html;

use crate::web::pages;


pub fn generate_http_error_response(
    code: actix_web::http::StatusCode,
    message: Option<String>,
) -> HttpResponse {
    let headline = match code {
        actix_web::http::StatusCode::NOT_FOUND => "Not found",
        _ => "Error",
    };

    let status = match message {
        Some(message) => format!("{}: {}", code, message),
        None => format!("{}", code),
    };

    let html = pages::decorate_maud_html(
        "Error",
        html! {
            h1 { (headline) }
            p { (status) }
        },
    );

    HttpResponse::build(code).body(html.into_string())
}

pub fn generate_404() -> HttpResponse {
    generate_http_error_response(actix_web::http::StatusCode::NOT_FOUND, None)
}
