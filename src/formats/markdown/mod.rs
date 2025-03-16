mod converter;

pub use model::Document;

pub mod model {
    use crate::core::semantic::tag;

    pub type Spans = Vec<Span>;
    pub type Nodes = Vec<Node>;
    pub type ListItems = Vec<ListItem>;

    pub type Tag = tag::TagHandle;

    #[derive(Debug, Clone)]
    pub struct JournalEntryHeader {
        pub timestamp: String,
        pub title: Option<String>,
        pub tags: Vec<Tag>,
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct Wikilink {
        pub target: String,
        pub label: Option<String>,
        pub embedded: bool,
    }

    #[derive(Debug, Clone)]
    pub enum Span {
        Break,
        Text(String),
        Code(String),
        Link(String, String, Option<String>),
        Image(String, String, Option<String>),

        Emphasis(Spans),
        Strong(Spans),

        Tag(Tag),
        Wikilink(Wikilink),
        Label(String),

        Unknown,
    }

    #[derive(Debug, Clone)]
    pub enum OrderedListType {
        Undefined,
    }

    #[derive(Debug, Clone)]
    pub enum ListItem {
        Simple(Spans),
        Paragraph(Nodes),
    }

    #[derive(Debug, Clone)]
    pub enum Node {
        Header(Spans, usize),
        Paragraph(Spans),
        Blockquote(Option<String>, Nodes),
        CodeBlock(Option<String>, String),
        OrderedList(ListItems, OrderedListType),
        UnorderedList(ListItems),
        Raw(String),
        Hr,

        Unknown,
    }

    #[derive(Debug, Clone)]
    pub struct Document {
        pub properties: Option<yaml_rust::Yaml>,
        pub toplevels: Nodes,
    }

    impl Document {
        pub fn get_property(&self, key: &str) -> Option<&yaml_rust::Yaml> {
            self.properties
                .as_ref()
                .and_then(|p| p.as_hash()?.get(&yaml_rust::Yaml::String(key.to_string())))
        }

        pub fn get_property_string(&self, key: &str) -> Option<String> {
            match self.get_property(key) {
                Some(yaml_rust::Yaml::String(s)) => Some(s.clone()),
                _ => None,
            }
        }

        pub fn get_property_list(&self, key: &str) -> Option<&Vec<yaml_rust::Yaml>> {
            self.get_property(key).and_then(|p| p.as_vec())
        }

        pub fn get_property_list_of_strings(&self, key: &str) -> Option<Vec<String>> {
            self.get_property_list(key).map(|v| {
                v.iter()
                    .filter_map(|item| match item {
                        yaml_rust::Yaml::String(s) => Some(s.clone()),
                        _ => None,
                    })
                    .collect()
            })
        }
    }

    pub mod parser {
        use super::super::converter;

        pub type ParseError = converter::ParseError;

        pub type ParseContext<'a> = converter::ParseContext<'a>;

        pub fn parse_document<S: ToString>(
            input: S,
            context: ParseContext,
        ) -> Result<super::Document, ParseError> {
            converter::parse_document(input, context)
        }
    }
}
