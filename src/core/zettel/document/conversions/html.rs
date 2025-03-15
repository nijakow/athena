use crate::core::zettel::document;

pub trait AsHtml {
    fn as_html(&self) -> String;
}

fn convert_style(style: &document::node::Style) -> &'static str {
    match style {
        document::node::Style::Bold => "b",
        document::node::Style::Italic => "i",
        document::node::Style::Underline => "u",
        document::node::Style::Strikethrough => "s",
    }
}

impl AsHtml for document::node::Node {
    fn as_html(&self) -> String {
        use document::node::Node;
        use maud::html;

        match self {
            Node::Text(text) => html! { (text) }.into_string(),
            Node::Styled(style, node) => {
                let tag_name = convert_style(style);
                let html = node.as_html();

                format!("<{}>{}</{}>", tag_name, html, tag_name)
            }
            Node::Link(link) => {
                let target = &link.target;
                let caption = &link.caption;

                format!("<a href=\"{}\">{}</a>", target.as_safe_uri(), caption)
            }
            Node::Grouped(nodes) => {
                let html = nodes.iter().map(|node| node.as_html()).collect::<String>();

                html! { (maud::PreEscaped(html)) }.into_string()
            }
        }
    }
}

impl AsHtml for document::element::Heading {
    fn as_html(&self) -> String {
        use maud::html;

        let tag_name = format!("h{}", self.level);
        let text = &self.text;

        format!(
            "<{}>{}</{}>",
            tag_name,
            html! { (text) }.into_string(),
            tag_name
        )
    }
}

impl AsHtml for document::element::Paragraph {
    fn as_html(&self) -> String {
        use maud::html;

        let html = self
            .nodes
            .iter()
            .map(|node| node.as_html())
            .collect::<String>();

        html! { p { (maud::PreEscaped(html)) } }.into_string()
    }
}

impl AsHtml for document::element::Element {
    fn as_html(&self) -> String {
        use document::element::Element;

        match self {
            Element::Heading(heading) => heading.as_html(),
            Element::Line => "<hr>".to_string(),
            Element::Paragraph(paragraph) => paragraph.as_html(),
        }
    }
}

impl AsHtml for document::Document {
    fn as_html(&self) -> String {
        self.elements
            .iter()
            .map(|element| element.as_html())
            .collect::<String>()
    }
}
