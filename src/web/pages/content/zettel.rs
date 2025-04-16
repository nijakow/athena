use std::sync::Arc;

use actix_web::{web, HttpResponse};
use maud::html;

use crate::{core::{entity::{self, zettel::{self, document::conversions::html::AsHtml}}, vault}, web::pages};


/// Generate a HTML table containing the metadata of a zettel.
fn generate_metadata_box(
    header: &zettel::Header,
) -> maud::PreEscaped<String> {
    let yaml = &header.yaml;

    match yaml {
        Some(yaml) => {
            fn yaml_to_string(yaml: &yaml_rust2::Yaml) -> String {
                let mut buffer = String::new(); // Create a buffer to write the serialized YAML
                let mut emitter = yaml_rust2::YamlEmitter::new(&mut buffer); // Create a YamlEmitter
                emitter.dump(yaml).unwrap(); // Serialize the YAML into the buffer
                buffer // Use the buffer directly as it is already a String
            }

            let yaml_string = yaml_to_string(yaml);

            maud::html! {
                pre {
                    code {
                        (yaml_string)
                    }
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
    let title = "Zettel";

    let conversion_context =
        zettel::document::conversions::html::HtmlConversionContext::new(Arc::clone(vault));

    let content = zettel
        .body_as_document()
        .unwrap()
        .as_html(&conversion_context);

    let html = pages::decorate_maud_html(
        title,
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
