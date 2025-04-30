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
    web::pages::{self, decorate_content_page},
};

/// Generate a HTML table containing the metadata of a zettel.
fn generate_metadata_box(header: &zettel::Header) -> maud::PreEscaped<String> {
    let yaml = &header.yaml;

    match yaml {
        Some(yaml) => {
            fn yaml_to_table(yaml: &yaml_rust2::Yaml) -> maud::Markup {
                use yaml_rust2::Yaml;

                fn render_yaml_value(value: &Yaml) -> maud::Markup {
                    match value {
                        Yaml::String(s) => maud::html! {
                            span { (s) }
                        },
                        Yaml::Array(arr) => maud::html! {
                            ul {
                                @for item in arr {
                                    li { (render_yaml_value(item)) }
                                }
                            }
                        },
                        Yaml::Hash(_hash) => yaml_to_table(value),
                        Yaml::Integer(i) => maud::html! {
                            span { (i) }
                        },
                        Yaml::Real(r) => maud::html! {
                            span { (r) }
                        },
                        Yaml::Boolean(b) => maud::html! {
                            span { (b) }
                        },
                        Yaml::Null => maud::html! {
                            span { "null" }
                        },
                        _ => maud::html! {
                            span { "Unsupported type" }
                        },
                    }
                }

                maud::html! {
                    table style="border-collapse: collapse; width: 100%; border: 1px solid black;" {
                        @for (key, value) in yaml.as_hash().into_iter().flat_map(|h| h.iter()) {
                            tr style="border: 1px solid black;" {
                                td style="font-weight: bold; padding: 0.5em; border: 1px solid black;" {
                                    (key.as_str().unwrap_or("Unknown"))
                                }
                                td style="padding: 0.5em; border: 1px solid black;" {
                                    (render_yaml_value(value))
                                }
                            }
                        }
                    }
                }
            }

            let metadata_table = yaml_to_table(yaml);

            maud::html! {
                div class="metadata-box" style="margin-top: 1em;" {
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
        decorate_content_page(html! {
            h1 { (title) }
            a href=(format!("{}?action=edit", id.as_safe_uri())) { "Edit" }
            br;
            (generate_metadata_box(&zettel.header))
            br;
            (maud::PreEscaped(content))
        }),
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
