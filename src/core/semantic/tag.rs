

pub struct Tag {
    name: String,
}

impl Tag {
    pub fn new(name: String) -> Tag {
        Tag {
            name,
        }
    }
}

impl std::fmt::Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "#{}", self.name)
    }
}

impl std::fmt::Debug for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "#{}", self.name)
    }
}


pub type TagHandle = std::rc::Rc<Tag>;

pub struct Tags {
    tags: std::collections::HashMap<String, TagHandle>,
}

impl Tags {
    pub fn new() -> Tags {
        Tags {
            tags: std::collections::HashMap::new(),
        }
    }

    pub fn get_tag(&self, name: &str) -> Option<TagHandle> {
        self.tags.get(name).map(|tag| tag.clone())
    }

    fn add_tag(&mut self, tag: Tag) -> TagHandle {
        let tag_handle = std::rc::Rc::new(tag);
        self.tags.insert(tag_handle.name.clone(), tag_handle.clone());
        tag_handle
    }

    pub fn get_or_add_tag(&mut self, name: &str) -> TagHandle {
        match self.get_tag(name) {
            Some(tag) => tag,
            None => self.add_tag(Tag::new(name.to_string())),
        }
    }
}

