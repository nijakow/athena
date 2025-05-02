use crate::util::hashing::Sha256;

pub struct DataStorage<T>
where
    T: serde::Serialize + for<'de> serde::Deserialize<'de>,
{
    base_path: std::path::PathBuf,
    _marker: std::marker::PhantomData<T>,
}

impl<T> DataStorage<T>
where
    T: serde::Serialize + for<'de> serde::Deserialize<'de>,
{
    pub fn open(base_path: std::path::PathBuf, create: bool) -> Result<Self, ()> {
        fn check_preconditions(base_path: &std::path::Path, create: bool) -> bool {
            if base_path.exists() {
                base_path.is_dir()
            } else {
                create && std::fs::create_dir_all(base_path).is_ok()
            }
        }

        if check_preconditions(&base_path, create) {
            let storage = Self {
                base_path,
                _marker: std::marker::PhantomData,
            };
            Ok(storage)
        } else {
            Err(())
        }
    }

    fn local_path_for_key(&self, key: &Sha256) -> std::path::PathBuf {
        let encoded = key.as_string();

        let first_dir = &encoded[0..2];

        {
            let mut path = self.base_path.clone();

            path.push(first_dir);
            path.push(format!("{}.json", encoded));

            path
        }
    }

    pub fn read(&self, key: &Sha256) -> Result<T, ()> {
        let path = self.local_path_for_key(key);

        if path.exists() {
            let file = std::fs::File::open(path).map_err(|_| ())?;
            let reader = std::io::BufReader::new(file);
            let data: T = serde_json::from_reader(reader).map_err(|_| ())?;
            Ok(data)
        } else {
            Err(())
        }
    }

    pub fn write(&self, key: &Sha256, data: &T) -> Result<(), ()> {
        let path = self.local_path_for_key(key);

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|_| ())?;
        }

        let file = std::fs::File::create(path).map_err(|_| ())?;
        let writer = std::io::BufWriter::new(file);
        serde_json::to_writer(writer, data).map_err(|_| ())?;

        Ok(())
    }
}
