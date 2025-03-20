#[derive(Debug, Clone, Copy)]
pub enum Type {
    Athena,
    Obsidian,
    Pdf,
}

impl Type {
    pub fn from_extension(extension: &str) -> Option<Type> {
        match extension {
            "zson" => Some(Type::Athena),
            "md" => Some(Type::Obsidian),
            "pdf" => Some(Type::Pdf),
            _ => None,
        }
    }

    pub fn to_extension(&self) -> Option<&'static str> {
        Some(match self {
            Type::Athena => "zson",
            Type::Obsidian => "md",
            Type::Pdf => "pdf",
        })
    }

    pub fn mime_type(&self) -> Option<&'static str> {
        Some(match self {
            Type::Athena => "application/json",
            Type::Obsidian => "text/markdown",
            Type::Pdf => "application/pdf",
        })
    }

    pub fn all() -> Vec<Type> {
        vec![Type::Athena, Type::Obsidian, Type::Pdf]
    }

    pub fn all_extensions() -> Vec<&'static str> {
        Self::all()
            .iter()
            .map(|t| t.to_extension().unwrap())
            .collect()
    }
}


pub struct Metadata {
    pub resource_type: Option<Type>,
}

pub struct SplitJson {
    pub header: serde_json::Value,
    pub body: serde_json::Value,
}

impl SplitJson {
    pub fn parse<S: ToString>(content: S) -> Result<SplitJson, Box<dyn std::error::Error>> {
        /*
         * The SplitJson format is a file that begins with `---`, followed by a JSON object, followed by `---`, followed by another JSON object.
         */

        let content = content.to_string();

        // Find the position of the first `---`
        let first_delimiter = content
            .find("---")
            .ok_or("Failed to find first delimiter")?;
        let second_delimiter = content[first_delimiter + 3..]
            .find("---")
            .ok_or("Failed to find second delimiter")?;

        // Get the header and body JSON objects
        let header = &content[first_delimiter + 3..first_delimiter + 3 + second_delimiter];
        let body = &content[first_delimiter + 3 + second_delimiter + 3..];

        // Parse the JSON objects
        let header = serde_json::from_str(header)?;
        let body = serde_json::from_str(body)?;

        Ok(SplitJson { header, body })
    }
}

pub struct Resource {
    path: std::path::PathBuf,
}

impl Resource {
    pub fn from_path(path: std::path::PathBuf) -> Resource {
        Resource { path }
    }

    pub fn metadata(&self) -> Metadata {
        let extension = self.path.extension().and_then(|e| e.to_str());
        let resource_type = extension.and_then(|e| Type::from_extension(e));

        Metadata { resource_type }
    }

    pub fn open_for_reading(&self) -> Result<Box<dyn std::io::Read>, std::io::Error> {
        std::fs::File::open(&self.path).map(|f| Box::new(f) as Box<dyn std::io::Read>)
    }

    pub fn read_to_string(&self) -> Result<String, std::io::Error> {
        std::fs::read_to_string(&self.path)
    }

    pub fn read_to_split_json(&self) -> Result<SplitJson, Box<dyn std::error::Error>> {
        // Get the content from the bytes as UTF-8
        let content = std::fs::read_to_string(&self.path)?;

        // Parse the content into a SplitJson object
        SplitJson::parse(content)
    }

    pub fn read_to_obsidian_markdown(
        &self,
    ) -> Result<crate::formats::markdown::Document, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(&self.path)?;

        crate::formats::markdown::parser::parse_document(content)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    }
}
