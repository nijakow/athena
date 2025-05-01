use crate::core::entity::{self, zettel::document::conversions::html::HtmlConversionContext};

use crate::core::vault::resource;

fn content_not_displayed() -> maud::Markup {
    maud::html! {
        p { "Content not displayed" }
    }
}

fn decorate_with_link_button<S: ToString>(
    uri: S,
    content: maud::PreEscaped<String>,
) -> maud::Markup {
    // Create a div element that overlays a link button at the top right corner of the embedded content. Inline the CSS.

    let uri = uri.to_string();

    maud::html! {
        div style="position: relative" {
            (content)
            a href=(uri) style="position: absolute; top: 0; right: 0; padding: 0.5em; background-color: #000; color: #fff; text-decoration: none" { "Open" }
        }
    }
}

fn get_youtube_embed_url(url: &url::Url) -> Option<String> {
    if url.domain()? == "www.youtube.com" && url.path() == "/watch" {
        if let Some(video_id) = url.query_pairs().find(|(key, _)| key == "v").map(|(_, value)| value) {
            return Some(format!("https://www.youtube.com/embed/{}", video_id));
        }
    } else if url.domain()? == "youtu.be" {
        return Some(format!("https://www.youtube.com/embed{}", url.path()));
    }
    None
}

pub fn generate_embed_for_url(url: &url::Url) -> Option<maud::PreEscaped<String>> {
    // Check if the URL is a YouTube video
    if let Some(embed_url) = get_youtube_embed_url(url) {
        return Some(maud::html! {
            iframe src=(embed_url) width="100%" height="315" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowfullscreen {}
        });
    }

    let guessed_type = resource::Type::from_url(url)?;
    let mime = guessed_type.mime_type().to_string();
    let uri = url;

    match guessed_type {
        resource::Type::Image(_) => Some(maud::html! {
            img src=(uri) width="100%" {}
        }),
        resource::Type::Audio(_) => Some(maud::html! {
            audio controls {
                source src=(uri) type=(mime) {}
            }
        }),
        resource::Type::Video(_) => Some(maud::html! {
            video controls width="100%" {
                source src=(uri) type=(mime) {}
            }
        }),
        _ => None,
    }
}

pub fn generate_embed_for_url_or_inline(url: &url::Url) -> maud::PreEscaped<String> {
    if let Some(embed) = generate_embed_for_url(&url) {
        embed
    } else {
        // This is still not perfect, but it is better than nothing
        maud::html! {
            iframe src=(url) width="100%" height="500" style="border: none; overflow: hidden" {}
        }
    }
}

pub fn generate_embed_for_url_or_not_displayed(url: &url::Url) -> maud::PreEscaped<String> {
    if let Some(embed) = generate_embed_for_url(&url) {
        embed
    } else {
        content_not_displayed()
    }
}

pub fn embed_file_for_id<S: ToString>(
    file: &crate::core::vault::resource::file::FileContent,
    id: &entity::Id,
    title: S,
    download: bool,
) -> maud::PreEscaped<String> {
    let title = title.to_string();
    let mime = file.metadata().mime_type().to_string();

    let uri = if download {
        id.as_safe_download_uri()
    } else {
        id.as_safe_uri()
    };

    match file.metadata().file_type() {
        resource::Type::Document(resource::types::DocumentType::PlainText) => {
            if let Ok(content) = std::str::from_utf8(file.content()) {
                maud::html! {
                    pre { (content) }
                }
            } else {
                content_not_displayed()
            }
        }
        resource::Type::Document(resource::types::DocumentType::Pdf) => {
            let content = maud::html! {
                // Keep the link to the original URI in the tag, so that the PDF can be opened in a new tab
                object class="pdf" data=(id.as_safe_download_uri()) type="application/pdf" width="100%" style="aspect-ratio: 4 / 3" {}
            };

            if download {
                content
            } else {
                maud::html! {
                    (decorate_with_link_button(uri, content))
                }
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
        resource::Type::Video(_) => {
            maud::html! {
                video controls width="100%" {
                    source src=(uri) type=(mime) {}
                }
            }
        }
        _ => content_not_displayed(),
    }
}

pub fn embed_entity_for_id(
    entity: &entity::Entity,
    id: &entity::Id,
    conversion_context: &HtmlConversionContext,
) -> maud::PreEscaped<String> {
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
