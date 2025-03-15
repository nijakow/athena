
pub mod conversions;

pub mod element;
pub mod node;


pub type Elements = Vec<element::Element>;
pub type Nodes = Vec<node::Node>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Document {
    elements: Elements,
}

impl Document {
    pub fn test() -> Self {
        let paragraph = element::Paragraph {
            nodes: vec![
                node::Node::Text("Hello, ".to_string()),
                node::Node::Styled(
                    node::Style::Bold,
                    Box::new(node::Node::Text("world".to_string())),
                ),
                node::Node::Text("!".to_string()),
            ],
        };

        let paragraph2 = element::Paragraph {
            nodes: vec![
                node::Node::Text("Goodbye, ".to_string()),
                node::Node::Styled(
                    node::Style::Italic,
                    Box::new(node::Node::Link(node::Link {
                        target: crate::core::zettel::Id::with_id("b"),
                        caption: "world".to_string(),
                    })),
                ),
                node::Node::Text("!".to_string()),
            ],
        };

        let elements = vec![
            element::Element::Heading(element::Heading::new(1, "Hello, world!".to_string())),
            element::Element::Paragraph(paragraph),
            element::Element::Line,
            element::Element::Paragraph(paragraph2),
            element::Element::Heading(element::Heading::new(2, "Goodbye, world!".to_string())),
        ];

        Document { elements }
    }
}
