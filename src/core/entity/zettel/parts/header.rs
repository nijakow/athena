use crate::core::entity::{self, link::reference::Reference};


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Time {
    DateTime(chrono::NaiveDateTime),
    Date(chrono::NaiveDate),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Element {
    String(String),
    Reference(Reference),
    Time(Time),
    Boolean(bool),
}

impl Element {
    pub fn parse_string(text: &str) -> Self {
        fn try_parse_entity_reference(text: &str) -> Option<Reference> {
            if text.starts_with("[[") {
                let end = text.find("]]").unwrap_or_else(|| text.find('|').unwrap_or(text.len()));
                let reference = &text[2..end];
                let id = entity::Id::from_string(reference).ok()?;
                Some(Reference::Entity(id))
            } else {
                None
            }
        }

        fn try_parse_url_reference(text: &str) -> Option<Reference> {
            url::Url::parse(text)
                .ok()
                .and_then(|url| {
                    if url.scheme() == "obsidian" {
                        let id = entity::Id::from_string(url.path()).ok()?;
                        Some(Reference::Entity(id))
                    } else {
                        None
                    }
                })
        }

        fn try_parse_time(text: &str) -> Option<Time> {
            if let Ok(date) = chrono::NaiveDate::parse_from_str(text, "%Y-%m-%d") {
                Some(Time::Date(date))
            } else if let Ok(datetime) = chrono::NaiveDateTime::parse_from_str(text, "%Y-%m-%d %H:%M:%S") {
                Some(Time::DateTime(datetime))
            } else {
                None
            }
        }

        if let Some(reference) = try_parse_entity_reference(text) {
            Element::Reference(reference)
        } else if let Some(reference) = try_parse_url_reference(text) {
            Element::Reference(reference)
        } else if let Some(time) = try_parse_time(text) {
            Element::Time(time)
        } else {
            Element::String(text.to_string())
        }
    }

    pub fn from_yaml(yaml: &yaml_rust2::Yaml) -> Option<Self> {
        match yaml {
            yaml_rust2::Yaml::String(s) => Some(Element::String(s.clone())),
            yaml_rust2::Yaml::Boolean(b) => Some(Element::Boolean(*b)),
            _ => None,
        }
    }
}


pub enum Line {
    Single(Element),
    Multi(Vec<Element>),
}

impl Line {
    pub fn from_yaml(yaml: &yaml_rust2::Yaml) -> Option<Self> {
        match yaml {
            yaml_rust2::Yaml::Array(arr) => {
                let elements = arr.iter().filter_map(Element::from_yaml).collect();
                Some(Line::Multi(elements))
            }
            _ => Some(Line::Single(Element::from_yaml(yaml)?)),
        }
    }
}


pub struct Header {
    pub title: Option<String>,
    pub yaml: Option<yaml_rust2::Yaml>,
}

impl Header {
    pub fn new(title: Option<String>) -> Self {
        Self { title, yaml: None }
    }

    pub fn from_yaml(yaml: yaml_rust2::Yaml) -> Self {
        let title = yaml["title"].as_str().map(|s| s.to_string());

        Self {
            title,
            yaml: Some(yaml.clone()),
        }
    }
}

impl Default for Header {
    fn default() -> Self {
        Self::new(None)
    }
}
