use actix_web::{web, HttpResponse, Responder};
use maud::html;
use std::{collections::HashMap, sync::Arc};

use crate::core::{entity, vault};

use super::pages::{self, decorate_content_page};

pub async fn web_file(_vault: web::Data<Arc<vault::Vault>>, id: web::Path<String>) -> HttpResponse {
    let file_name = id.into_inner();

    match file_name.as_str() {
        "css.css" => {
            let css = include_str!("../../../static/css.css");
            HttpResponse::Ok().content_type("text/css").body(css)
        }
        _ => pages::error::generate_404(),
    }
}

pub async fn list_entities(vault: web::Data<Arc<vault::Vault>>) -> impl Responder {
    let mut zettels: Vec<(entity::Id, String)> = match vault.list_entities() {
        Some(it) => it
            .into_iter()
            .map(|id| {
                let title = vault
                    .title_of_entity(&id)
                    .unwrap_or_else(|| "Untitled".to_string());
                (id, title)
            })
            .collect::<Vec<_>>(),
        None => vec![],
    };

    zettels.sort_by(|a, b| a.1.cmp(&b.1));

    let html = pages::decorate_maud_html(
        "Zettel",
        decorate_content_page(html! {
            ul {
                @for (id, title) in zettels {
                    li {
                        a href=(format!("{}", id.as_safe_uri())) { (title) }
                    }
                }
            }
        }),
    );

    HttpResponse::Ok().body(html.into_string())
}

pub async fn process_entity(
    vault: web::Data<Arc<vault::Vault>>,
    id: web::Path<String>,
    query: web::Query<HashMap<String, String>>,
    accept: web::Header<actix_web::http::header::Accept>,
) -> impl Responder {
    let action = query.get("action").map(|s| s.as_str());

    match action {
        Some("edit") => pages::content::zettel::edit_zettel(vault, id).await,
        _ => pages::show_entity(vault, id, accept).await,
    }
}

pub async fn download_entity(
    vault: web::Data<Arc<vault::Vault>>,
    id: web::Path<String>,
) -> HttpResponse {
    let id = entity::Id::with_id(id.into_inner());

    if let Some(resource) = vault.load_resource(&id) {
        pages::generate_download_resource(resource)
    } else {
        pages::error::generate_http_error_response(
            actix_web::http::StatusCode::NOT_IMPLEMENTED,
            Some("Download for this type not implemented yet!".to_string()),
        )
    }
}

pub async fn post_entity(
    vault: web::Data<Arc<vault::Vault>>,
    id: web::Path<String>,
    body: web::Form<HashMap<String, String>>,
) -> HttpResponse {
    let id = entity::Id::with_id(id.into_inner());
    let content = body.get("content").unwrap();

    println!("Received content: {}", content);

    let zettel = vault.load_zettel(&id);

    match zettel {
        Some(_zettel) => {
            println!("Zettel found, updating content");
        }
        None => {
            println!("Zettel not found");
        }
    }

    // We are okay and we are returning the Zettel
    pages::generate_show_entity(&vault, id)
}
