
use super::VolumeId;


#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct VolumePath {
    volume: VolumeId,
    path: std::path::PathBuf,
}

impl VolumePath {
    pub fn new(volume: VolumeId, path: std::path::PathBuf) -> Self {
        Self { volume, path }
    }

    pub fn volume(&self) -> &VolumeId {
        &self.volume
    }

    pub fn path(&self) -> &std::path::PathBuf {
        &self.path
    }
}

impl serde::Serialize for VolumePath {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let volume_path = format!("{}:{}", self.volume.as_string(), self.path.display());
        serializer.serialize_str(&volume_path)
    }
}

impl<'de> serde::Deserialize<'de> for VolumePath {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let volume_path = String::deserialize(deserializer)?;
        let parts: Vec<&str> = volume_path.split(':').collect();
        if parts.len() != 2 {
            return Err(serde::de::Error::custom("Invalid volume path format"));
        }
        let volume = parts[0].to_string();
        let path = std::path::PathBuf::from(parts[1]);
        let volume_id = VolumeId::from_string(volume).map_err(|_| serde::de::Error::custom("Invalid volume ID format"))?;
        Ok(Self::new(volume_id, path))
    }
}
