use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use maud::{html, DOCTYPE};
use std::{collections::HashMap, sync::Arc};

use crate::core::{vault, zettel::{self, document::conversions::html::AsHtml}};


async fn list_zettels(vault: web::Data<Arc<vault::Vault>>) -> impl Responder {
    let mut zettels: Vec<(zettel::Id, String)> = vault.list_zettels().into_iter().map(|id| {
        let title = vault.title_of(&id).unwrap_or_else(|| "Untitled".to_string());
        (id, title)
    }).collect::<Vec<_>>();

    zettels.sort_by(|(a, _), (b, _)| a.id().cmp(b.id()));

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
    vault: &Arc<vault::Vault>,
    id: zettel::Id,
) -> HttpResponse {
    let zettel = vault.load(&id);

    match zettel {
        Some(zettel) => {
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
        None => {
            let html = html! {
                (DOCTYPE)
                meta charset="utf-8";
                html {
                    head {
                        title { "Zettel" }
                    }
                    body {
                        h1 { "Zettel " i { (id.id()) } " not found" }
                        p { "Not found" }
                    }
                }
            };

            HttpResponse::NotFound().body(html.into_string())
        }
    }
}

async fn show_zettel(
    vault: web::Data<Arc<vault::Vault>>,
    id: web::Path<String>,
) -> HttpResponse {
    let id = zettel::Id::with_id(id.into_inner());
    
    generate_show_zettel(&vault, id)
}

pub async fn edit_zettel(
    vault: web::Data<Arc<vault::Vault>>,
    id: web::Path<String>,
) -> HttpResponse {
    let id = zettel::Id::with_id(id.into_inner());
    let zettel = vault.load(&id);

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

async fn post_zettel(
    vault: web::Data<Arc<vault::Vault>>,
    id: web::Path<String>,
    body: web::Form<HashMap<String, String>>,
) -> HttpResponse {
    let id = zettel::Id::with_id(id.into_inner());
    let content = body.get("content").unwrap();

    println!("Received content: {}", content);

    let zettel = vault.load(&id);

    match zettel {
        Some(zettel) => {
            println!("Zettel found, updating content");
        }
        None => {
            println!("Zettel not found");
        }
    }

    // We are okay and we are returning the Zettel
    generate_show_zettel(&vault, id)
}

async fn process_zettel(
    vault: web::Data<Arc<vault::Vault>>,
    id: web::Path<String>,
    query: web::Query<HashMap<String, String>>,
) -> impl Responder {
    let action = query.get("action").map(|s| s.as_str());

    match action {
        Some("edit") => edit_zettel(vault, id).await,
        _            => show_zettel(vault, id).await,
    }
}


pub async fn go(vault: vault::Vault) -> std::io::Result<()> {
    let vault_data = web::Data::new(Arc::new(vault));

    HttpServer::new(move || {
        App::new()
            .app_data(vault_data.clone())
            .route("/", web::get().to(list_zettels))
            .route("/zettel/{id}", web::get().to(process_zettel))
            .route("/zettel/{id}", web::post().to(post_zettel))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
