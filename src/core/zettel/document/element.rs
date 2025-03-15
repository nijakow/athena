#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Heading {
    pub level: u8,
    pub text: String,
}

impl Heading {
    pub fn new(level: u8, text: String) -> Heading {
        Heading { level, text }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Paragraph {
    pub nodes: super::Nodes,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Element {
    Heading(Heading),
    Line,
    Paragraph(Paragraph),
}
