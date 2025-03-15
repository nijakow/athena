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
            Node::Reference(link) => {
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

impl AsHtml for document::block::Heading {
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

impl AsHtml for document::block::CodeBlock {
    fn as_html(&self) -> String {
        use maud::html;

        let code = &self.code;

        // Turn a code block into a <pre> element with a <code> element inside
        html! { pre { code { (code) } } }.into_string()
    }
}

impl AsHtml for document::block::Paragraph {
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

impl AsHtml for document::block::Block {
    fn as_html(&self) -> String {
        use document::block::Block;

        match self {
            Block::Heading(heading) => heading.as_html(),
            Block::Line => "<hr>".to_string(),
            Block::CodeBlock(codeblock) => codeblock.as_html(),
            Block::Paragraph(paragraph) => paragraph.as_html(),
        }
    }
}

impl AsHtml for document::Document {
    fn as_html(&self) -> String {
        self.blocks
            .iter()
            .map(|block| block.as_html())
            .collect::<String>()
    }
}
