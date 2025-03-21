use crate::core::{entity, io::resource};


pub fn embed_file_for_id<S: ToString>(file: &entity::file::File, id: &entity::Id, title: S) -> maud::PreEscaped<String> {
    let title = title.to_string();
    let mime = file.metadata().mime_type().to_string();

    match file.metadata().file_type() {
        resource::Type::Document(resource::DocumentType::PlainText) => {
            let content = std::str::from_utf8(file.content()).unwrap_or("Content not displayed");
            maud::html! {
                pre { (content) }
            }
        }
        resource::Type::Document(resource::DocumentType::Pdf) => {
            // Make sure that the PDF has a certain height, it should not be crushed vertically
            maud::html! {
                object class="pdf" data=(id.as_safe_uri()) type="application/pdf" width="100%" style="aspect-ratio: 4 / 3" {}
            }
        }
        resource::Type::Image(_) => {
            maud::html! {
                img src=(id.as_safe_uri()) alt=(title) width="100%" {}
            }
        }
        resource::Type::Audio(_) => {
            maud::html! {
                audio controls {
                    source src=(id.as_safe_uri()) type=(mime) {}
                }
            }
        }
        _ => maud::html! {
            p { "Content not displayed" }
        },
    }
}
