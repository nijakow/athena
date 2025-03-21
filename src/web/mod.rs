use actix_web::{http::header, web, App, HttpResponse, HttpServer, Responder};
use maud::{html, DOCTYPE};
use std::{collections::HashMap, sync::Arc};

use crate::core::{
    entity::{
        self,
        zettel::{self, document::conversions::html::AsHtml},
    },
    io::resource,
    vault,
};

fn generate_http_error_response(
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

    let html = html! {
        (DOCTYPE)
        meta charset="utf-8";
        html {
            head {
                title { "Error" }
            }
            body {
                h1 { (headline) }
                p { (status) }
            }
        }
    };

    HttpResponse::build(code).body(html.into_string())
}

fn generate_404() -> HttpResponse {
    generate_http_error_response(actix_web::http::StatusCode::NOT_FOUND, None)
}

async fn list_entities(vault: web::Data<Arc<vault::Vault>>) -> impl Responder {
    let mut zettels: Vec<(entity::Id, String)> = vault
        .list_entities()
        .into_iter()
        .map(|id| {
            let title = vault
                .title_of_entity(&id)
                .unwrap_or_else(|| "Untitled".to_string());
            (id, title)
        })
        .collect::<Vec<_>>();

    zettels.sort_by(|a, b| a.1.cmp(&b.1));

    let html = html! {
        (DOCTYPE)
        meta charset="utf-8";
        html {
            head {
                title { "Zettel" }
            }
            body {
                h1 { "Zettels" }
                ul {
                    @for (id, title) in zettels {
                        li {
                            a href=(format!("{}", id.as_safe_uri())) { (title) }
                        }
                    }
                }
            }
        }
    };

    HttpResponse::Ok().body(html.into_string())
}

fn generate_show_zettel(
    _vault: &Arc<vault::Vault>,
    id: entity::Id,
    zettel: zettel::Zettel,
) -> HttpResponse {
    let title = "Zettel";

    let content = zettel.body_as_document().unwrap().as_html();

    let html = html! {
        (DOCTYPE)
        meta charset="utf-8";
        html {
            head {
                title { (title) }
            }
            body {
                h1 { (title) }
                a href=(format!("{}?action=edit", id.as_safe_uri())) { "Edit" }
                br;
                (maud::PreEscaped(content))
            }
        }
    };

    HttpResponse::Ok().body(html.into_string())
}

fn generate_download_file(file: entity::file::File) -> HttpResponse {
    let mime = file.mime_type().to_string();
    let content = file.extract_content();

    HttpResponse::Ok().content_type(mime).body(content)
}

fn generate_show_file(id: entity::Id, file: entity::file::File) -> HttpResponse {
    let title = file.title().unwrap_or_else(|| "Untitled".to_string());

    let file_type = file.file_type();

    let mime = file_type.mime_type();

    let displayed_content_html = crate::util::embedding::embed_file_for_id(&file, &id, &title);

    let html = html! {
        (DOCTYPE)
        meta charset="utf-8";
        html {
            head {
                title { (title) }
            }
            body {
                h1 { (title) }
                p { "MIME type: " code { (mime) } }
                (displayed_content_html)
            }
        }
    };

    HttpResponse::Ok().body(html.into_string())
}

fn generate_show_entity(vault: &Arc<vault::Vault>, id: entity::Id) -> HttpResponse {
    let entity = vault.load_entity(&id);

    match entity {
        Some(entity::Entity::Zettel(zettel)) => generate_show_zettel(vault, id, zettel),
        Some(entity::Entity::File(file)) => generate_show_file(id, file),
        _ => generate_404(),
    }
}

async fn show_entity(
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
        download_entity(vault, id).await
    }
}

pub async fn edit_zettel(
    vault: web::Data<Arc<vault::Vault>>,
    id: web::Path<String>,
) -> HttpResponse {
    let id = entity::Id::with_id(id.into_inner());
    let zettel = vault.load_zettel(&id);

    let html = html! {
        (DOCTYPE)
        html {
            head {
                title { "Zettel" }
            }
            body {
                h1 { "Zettel" }
                form action=(format!("{}", id.as_safe_uri())) method="post" {
                    textarea name="content" {
                        @if let Some(zettel) = zettel {
                            (zettel.body_as_document().unwrap().as_html())
                        }
                    }
                    button type="submit" { "Save" }
                }
            }
        }
    };

    HttpResponse::Ok().body(html.into_string())
}

async fn post_entity(
    vault: web::Data<Arc<vault::Vault>>,
    id: web::Path<String>,
    body: web::Form<HashMap<String, String>>,
) -> HttpResponse {
    let id = entity::Id::with_id(id.into_inner());
    let content = body.get("content").unwrap();

    println!("Received content: {}", content);

    let zettel = vault.load_zettel(&id);

    match zettel {
        Some(zettel) => {
            println!("Zettel found, updating content");
        }
        None => {
            println!("Zettel not found");
        }
    }

    // We are okay and we are returning the Zettel
    generate_show_entity(&vault, id)
}

async fn process_entity(
    vault: web::Data<Arc<vault::Vault>>,
    id: web::Path<String>,
    query: web::Query<HashMap<String, String>>,
    accept: web::Header<actix_web::http::header::Accept>,
) -> impl Responder {
    let action = query.get("action").map(|s| s.as_str());

    match action {
        Some("edit") => edit_zettel(vault, id).await,
        _ => show_entity(vault, id, accept).await,
    }
}

async fn download_entity(
    vault: web::Data<Arc<vault::Vault>>,
    id: web::Path<String>,
) -> HttpResponse {
    let id = entity::Id::with_id(id.into_inner());

    let entity = vault.load_entity(&id);

    match entity {
        Some(entity::Entity::File(file)) => generate_download_file(file),
        _ => generate_http_error_response(
            actix_web::http::StatusCode::NOT_IMPLEMENTED,
            Some("Download for this type not implemented yet!".to_string()),
        ),
    }
}

pub async fn go(vault: vault::Vault) -> std::io::Result<()> {
    let vault_data = web::Data::new(Arc::new(vault));

    HttpServer::new(move || {
        App::new()
            .app_data(vault_data.clone())
            .route("/", web::get().to(list_entities))
            .route("/entity/{id}", web::get().to(process_entity))
            .route("/entity/{id}", web::post().to(post_entity))
            .route("/raw/{id}", web::get().to(download_entity))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
