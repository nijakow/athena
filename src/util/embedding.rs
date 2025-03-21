use crate::core::{entity::{self, zettel::document::conversions::html::HtmlConversionContext}, io::resource};


fn content_not_displayed() -> maud::Markup {
    maud::html! {
        p { "Content not displayed" }
    }
}

pub fn embed_file_for_id<S: ToString>(file: &entity::file::File, id: &entity::Id, title: S, download: bool) -> maud::PreEscaped<String> {
    let title = title.to_string();
    let mime = file.metadata().mime_type().to_string();

    let uri = if download {
        id.as_safe_download_uri()
    } else {
        id.as_safe_uri()
    };

    match file.metadata().file_type() {
        resource::Type::Document(resource::DocumentType::PlainText) => {
            if let Ok(content) = std::str::from_utf8(file.content()) {
               maud::html! {
                    pre { (content) }
                }
            } else {
                content_not_displayed()
            }
        }
        resource::Type::Document(resource::DocumentType::Pdf) => {
            // Make sure that the PDF has a certain height, it should not be crushed vertically
            maud::html! {
                object class="pdf" data=(uri) type="application/pdf" width="100%" style="aspect-ratio: 4 / 3" {}
            }
        }
        resource::Type::Image(_) => {
            maud::html! {
                img src=(uri) alt=(title) width="100%" {}
            }
        }
        resource::Type::Audio(_) => {
            maud::html! {
                audio controls {
                    source src=(uri) type=(mime) {}
                }
            }
        }
        _ => content_not_displayed(),
    }
}

pub fn embed_entity_for_id(entity: &entity::Entity, id: &entity::Id, conversion_context: &HtmlConversionContext) -> maud::PreEscaped<String> {
    use entity::zettel::document::conversions::html::AsHtml;

    match entity {
        entity::Entity::File(file) => embed_file_for_id(file, id, "Untitled", false),
        entity::Entity::Zettel(zettel) => {
            if let Some(content) = zettel.body_as_document() {
                let html = content.as_html(conversion_context);
                maud::html! {
                    (maud::PreEscaped(html))
                }
            } else {
                content_not_displayed()
            }
        }
    }
}
