use crate::{
    core::entity::{self, link::reference::Reference},
    semantic,
};

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
                let end = text
                    .find("|")
                    .unwrap_or_else(|| text.find("]]").unwrap_or(text.len()));
                let reference = &text[2..end];
                let id = entity::Id::from_string(reference).ok()?;
                Some(Reference::Entity(id))
            } else {
                None
            }
        }

        fn try_parse_url_reference(text: &str) -> Option<Reference> {
            url::Url::parse(text).ok().and_then(|url| {
                if url.scheme() == "obsidian" {
                    let id = entity::Id::from_string(url.path()).ok()?;
                    Some(Reference::Entity(id))
                } else {
                    Some(Reference::Url(url))
                }
            })
        }

        fn try_parse_time(text: &str) -> Option<Time> {
            if let Ok(date) = chrono::NaiveDate::parse_from_str(text, "%Y-%m-%d") {
                Some(Time::Date(date))
            } else if let Ok(datetime) =
                chrono::NaiveDateTime::parse_from_str(text, "%Y-%m-%d %H:%M")
            {
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
            yaml_rust2::Yaml::String(s) => Some(Self::parse_string(s)),
            yaml_rust2::Yaml::Boolean(b) => Some(Element::Boolean(*b)),
            _ => None,
        }
    }
}

impl semantic::Scannable for Element {
    fn iterate_info_items<F: FnMut(semantic::InfoItem)>(&self, func: &mut F) {
        match self {
            Element::Reference(reference) => {
                func(semantic::InfoItem::Link(reference.clone()));
            }
            _ => {}
        }
    }
}


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

impl semantic::Scannable for Line {
    fn iterate_info_items<F: FnMut(semantic::InfoItem)>(&self, func: &mut F) {
        match self {
            Line::Single(element) => element.iterate_info_items(func),
            Line::Multi(elements) => {
                for element in elements {
                    element.iterate_info_items(func);
                }
            }
        }
    }
}


pub struct Header {
    pub title: Option<String>,
    pub lines: Vec<(String, Line)>,
    pub yaml: Option<yaml_rust2::Yaml>,
}

impl Header {
    pub fn new(title: Option<String>) -> Self {
        Self {
            title,
            lines: Vec::new(),
            yaml: None,
        }
    }

    pub fn from_yaml(yaml: yaml_rust2::Yaml) -> Self {
        let title = yaml["title"].as_str().map(|s| s.to_string());

        let lines = match &yaml {
            yaml_rust2::Yaml::Hash(hash) => {
                let lines = hash
                    .iter()
                    .filter_map(|(k, v)| {
                        let key = k.as_str()?;
                        let value = Line::from_yaml(v)?;
                        Some((key.to_string(), value))
                    })
                    .collect::<Vec<_>>();

                lines
            }
            _ => Vec::new(),
        };

        Self {
            title,
            lines,
            yaml: Some(yaml.clone()),
        }
    }
}

impl Default for Header {
    fn default() -> Self {
        Self::new(None)
    }
}

impl semantic::Scannable for Header {
    fn iterate_info_items<F: FnMut(semantic::InfoItem)>(&self, func: &mut F) {
        for (_, line) in &self.lines {
            line.iterate_info_items(func);
        }
    }
}
