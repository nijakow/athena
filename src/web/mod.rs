use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use maud::{html, DOCTYPE};
use std::sync::Arc;

use crate::core::{vault, zettel::{self, document::conversions::html::AsHtml}};


async fn list_zettels(vault: web::Data<Arc<vault::Vault>>) -> impl Responder {
    let mut zettels: Vec<(zettel::Id, String)> = vault.list_zettels().into_iter().map(|id| {
        (id, String::from("Hello"))
    }).collect::<Vec<_>>();

    zettels.sort_by(|(a, _), (b, _)| a.id().cmp(b.id()));

    let html = html! {
        (DOCTYPE)
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

async fn show_zettel(
    vault: web::Data<Arc<vault::Vault>>,
    id: web::Path<String>,
) -> impl Responder {
    let id = zettel::Id::with_id(id.into_inner());
    let zettel = vault.load(&id);

    match zettel {
        Some(zettel) => {
            let title = "Zettel";

            let content = zettel.body_as_document().unwrap().as_html();

            let html = html! {
                (DOCTYPE)
                html {
                    head {
                        title { (title) }
                    }
                    body {
                        h1 { (title) }
                        (maud::PreEscaped(content))
                    }
                }
            };

            HttpResponse::Ok().body(html.into_string())
        }
        None => {
            let html = html! {
                (DOCTYPE)
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

pub async fn go(vault: vault::Vault) -> std::io::Result<()> {
    let vault_data = web::Data::new(Arc::new(vault));

    HttpServer::new(move || {
        App::new()
            .app_data(vault_data.clone())
            .route("/", web::get().to(list_zettels))
            .route("/zettel/{id}", web::get().to(show_zettel))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
