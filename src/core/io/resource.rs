

pub enum Type {
    Athena,
}

impl Type {
    pub fn from_extension(extension: &str) -> Option<Type> {
        match extension {
            "athena-json" => Some(Type::Athena),
            _             => None,
        }
    }
}


pub struct Metadata {
    pub resource_type: Option<Type>,
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

    pub fn read_to_json(&self) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let file = std::fs::File::open(&self.path)?;
        let reader = std::io::BufReader::new(file);

        Ok(serde_json::from_reader(reader)?)
    }
}
