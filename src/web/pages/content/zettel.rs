use std::sync::Arc;

use actix_web::{web, HttpResponse};
use maud::html;

use crate::{
    core::{
        entity::{
            self,
            zettel::{self, document::conversions::html::AsHtml},
        },
        vault,
    },
    web::pages,
};

/// Generate a HTML table containing the metadata of a zettel.
fn generate_metadata_box(header: &zettel::Header) -> maud::PreEscaped<String> {
    let yaml = &header.yaml;

    match yaml {
        Some(yaml) => {
            fn yaml_to_table(yaml: &yaml_rust2::Yaml) -> maud::Markup {
                maud::html! {
                    table {
                        @for (key, value) in yaml.as_hash().map(|h| h.iter()).into_iter().flatten() {
                            tr {
                                td style="font-weight: bold; padding-right: 1em;" {
                                    (key.as_str().unwrap_or("Unknown"))
                                }
                                td {
                                    pre {
                                        code {
                                            (value.as_str().unwrap_or(&format!("{:?}", value)))
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            let metadata_table = yaml_to_table(yaml);

            maud::html! {
                div class="metadata-box" {
                    h3 { "Metadata" }
                    (metadata_table)
                }
            }
        }
        None => maud::PreEscaped(String::new()),
    }
}

pub fn generate_show_zettel(
    vault: &Arc<vault::Vault>,
    id: entity::Id,
    zettel: zettel::Zettel,
) -> HttpResponse {
    let title = zettel
        .title()
        .map(String::from)
        .unwrap_or_else(|| id.as_readable_string());

    let conversion_context =
        zettel::document::conversions::html::HtmlConversionContext::new(Arc::clone(vault));

    let content = zettel
        .body_as_document()
        .unwrap()
        .as_html(&conversion_context);

    let html = pages::decorate_maud_html(
        &title,
        html! {
            h1 { (title) }
            a href=(format!("{}?action=edit", id.as_safe_uri())) { "Edit" }
            br;
            (generate_metadata_box(&zettel.header))
            br;
            (maud::PreEscaped(content))
        },
    );

    HttpResponse::Ok().body(html.into_string())
}

pub async fn edit_zettel(
    vault: web::Data<Arc<vault::Vault>>,
    id: web::Path<String>,
) -> HttpResponse {
    let id = entity::Id::with_id(id.into_inner());
    let zettel = vault.load_zettel(&id);

    let vault_ref = vault.get_ref();

    let conversion_context =
        zettel::document::conversions::html::HtmlConversionContext::new(Arc::clone(vault_ref));

    let html = pages::decorate_maud_html(
        "Zettel",
        html! {
            h1 { "Zettel" }
            form action=(format!("{}", id.as_safe_uri())) method="post" {
                textarea name="content" {
                    @if let Some(zettel) = zettel {
                        (zettel.body_as_document().unwrap().as_html(&conversion_context))
                    }
                }
                button type="submit" { "Save" }
            }
        },
    );

    HttpResponse::Ok().body(html.into_string())
}
