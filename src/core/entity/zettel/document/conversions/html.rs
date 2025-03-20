use crate::core::entity::zettel::document;

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

        fn escape<S: ToString>(text: S) -> String {
            html_escape::encode_safe(&text.to_string()).into_owned()
        }

        match self {
            Node::Newline => "<br/>".to_string(),
            Node::Text(text) => {
                // let text = escape(text);
                html! { (text) }.into_string()
            }
            Node::Tag(tag) => {
                let link = format!("/tags/{}", tag);
                html! { a href=(link) { code { (format!("#{}", tag)) } } }.into_string()
            }
            Node::Code(code) => {
                html! { code { (code) } }.into_string()
            }
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
        let tag_name = format!("h{}", self.level);
        let text = self.nodes.iter().map(|node| node.as_html()).collect::<String>();

        format!(
            "<{}>{}</{}>",
            tag_name,
            text,
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

impl AsHtml for document::block::callout::Callout {
    fn as_html(&self) -> String {
        use maud::html;

        let kind = match self.kind {
            document::block::callout::Kind::Basic => "basic",
            document::block::callout::Kind::Quote => "quote",
            document::block::callout::Kind::Note => "note",
            document::block::callout::Kind::Warning => "warning",
            document::block::callout::Kind::Info => "info",
            document::block::callout::Kind::Error => "error",
        };

        // Use a different background color hue for each kind of callout (use pastel colors, hardcoded as hex codes)
        let (background, border) = match self.kind {
            /*
             * Basic:   Grey-ish
             * Quote:   Grey-ish
             * Note:    Blue-ish
             * Info:    Green-ish
             * Warning: Yellow-ish
             * Error:   Red-ish
             */
            document::block::callout::Kind::Basic => ("#f0f0f0", "#000000"),
            document::block::callout::Kind::Quote => ("#f0f0f0", "#d0d0d0"),
            document::block::callout::Kind::Note => ("#f0f8ff", "#add8e6"),
            document::block::callout::Kind::Info => ("#f0fff0", "#90ee90"),
            document::block::callout::Kind::Warning => ("#ffffe0", "#ffd700"),
            document::block::callout::Kind::Error => ("#ffe0e0", "#ff6961"),
        };

        let blocks = self
            .blocks
            .iter()
            .map(|block| block.as_html())
            .collect::<String>();

        // The way we turn this into HTML is by using a <div> element, setting the background color
        // explicitly, adding a border and a margin, and then rendering the blocks inside

        let border_type = if let document::block::callout::Kind::Basic = self.kind {
            "border-left"
        } else {
            "border"
        };

        html! {
            div style=(format!("{}: 4px solid; margin: 1em 0; padding: 1em; background-color: {}; border-color: {}; border-radius: 0.5em;", border_type, background, border)) {
                (maud::PreEscaped(blocks))
            }
        }.into_string()
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
            Block::Callout(callout) => callout.as_html(),
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
