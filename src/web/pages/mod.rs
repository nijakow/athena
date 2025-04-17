use actix_web::{web, HttpResponse};
use maud::{html, DOCTYPE};
use std::sync::Arc;

use crate::core::{
    entity,
    vault,
};

use crate::volt;

pub mod error;
pub mod content;


pub fn decorate_maud_html(title: &str, content: maud::PreEscaped<String>) -> maud::PreEscaped<String> {
    html! {
        (DOCTYPE)
        meta charset="utf-8";
        html {
            head {
                title { (title) }
                link rel="stylesheet" href="/web/css.css";
            }
            body {
                (content)
            }
        }
    }
}



pub fn generate_download_resource(resource: volt::resource::Resource) -> HttpResponse {
    let mime = resource
        .metadata()
        .resource_type
        .map(|rt| rt.mime_type())
        .unwrap_or_else(|| "application/octet-stream");
    let content = resource.read_to_bytes().unwrap();

    let mime = if mime == "text/plain" || mime == "text/markdown" {
        format!("text/plain; charset=utf-8")
    } else {
        mime.to_string()
    };

    HttpResponse::Ok().content_type(mime).body(content)
}

pub fn generate_show_file(id: entity::Id, file: volt::resource::file::FileContent) -> HttpResponse {
    let title = file
        .metadata()
        .title()
        .unwrap_or_else(|| "Untitled".to_string());

    let file_type = file.metadata().file_type();

    let mime = file_type.mime_type();

    let displayed_content_html =
        crate::util::embedding::embed_file_for_id(&file, &id, &title, true);

    let html = decorate_maud_html(
        &title,
        html! {
            h1 { (title) }
            p { "MIME type: " code { (mime) } }
            (displayed_content_html)
        },
    );

    HttpResponse::Ok().body(html.into_string())
}

pub fn generate_show_entity(vault: &Arc<vault::Vault>, id: entity::Id) -> HttpResponse {
    let entity = vault.load_entity(&id);

    match entity {
        Some(entity::Entity::Zettel(zettel)) => content::zettel::generate_show_zettel(vault, id, zettel),
        Some(entity::Entity::File(file)) => generate_show_file(id, file),
        _ => error::generate_404(),
    }
}

pub async fn show_entity(
    vault: web::Data<Arc<vault::Vault>>,
    id: web::Path<String>,
    header: web::Header<actix_web::http::header::Accept>,
) -> HttpResponse {
    // If we accept HTML, we will show the entity. Otherwise, we will download it.

    let accept_html: bool = header
        .iter()
        .any(|accept| accept.to_string().to_lowercase().contains("text/html"));

    if accept_html {
        generate_show_entity(&vault, entity::Id::with_id(id.into_inner()))
    } else {
        super::routes::download_entity(vault, id).await // TODO: Move this to us!
    }
}
