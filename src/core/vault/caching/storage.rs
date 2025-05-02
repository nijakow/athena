use crate::util::hashing::Sha256;

pub struct DataStorage<T>
where
    T: serde::Serialize + for<'de> serde::Deserialize<'de> + Default,
{
    base_path: std::path::PathBuf,
    cached: std::collections::HashMap<Sha256, T>,
}

impl<T> DataStorage<T>
where
    T: serde::Serialize + for<'de> serde::Deserialize<'de> + Default,
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
                cached: std::collections::HashMap::new(),
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

    fn read(&mut self, key: &Sha256) -> Result<T, ()> {
        let path = self.local_path_for_key(key);

        if path.exists() {
            let file = std::fs::File::open(path).map_err(|_| ())?;
            let reader = std::io::BufReader::new(file);
            let data: T = serde_json::from_reader(reader).map_err(|_| ())?;
            Ok(data)
        } else {
            // If the file doesn't exist, return a default value
            Ok(T::default())
        }
    }

    fn write(&mut self, key: &Sha256, data: &T) -> Result<(), ()> {
        let path = self.local_path_for_key(key);

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|_| ())?;
        }

        let file = std::fs::File::create(path).map_err(|_| ())?;
        let writer = std::io::BufWriter::new(file);
        serde_json::to_writer(writer, data).map_err(|_| ())?;

        Ok(())
    }

    fn ensure_in_cache(&mut self, key: &Sha256) -> Result<(), ()> {
        if !self.cached.contains_key(key) {
            let data = self.read(key)?;
            self.cached.insert(key.clone(), data);
        }
        Ok(())
    }

    fn get(&mut self, key: &Sha256) -> Result<&mut T, ()> {
        self.ensure_in_cache(key)?;
        self.cached.get_mut(key).ok_or(())
    }

    pub fn modify<F>(&mut self, key: &Sha256, f: F) -> Result<(), ()>
    where
        F: FnOnce(&mut T),
    {
        let mut data = self.read(key).map_err(|_| ())?;
        f(&mut data);
        self.write(key, &data)
    }

    pub fn flush_cache(&mut self) -> Result<(), ()> {
        let pairs = self.cached.drain().collect::<Vec<_>>();

        for (key, data) in pairs {
            self.write(&key, &data).map_err(|_| ())?;
        }

        Ok(())
    }
}

impl<T> Drop for DataStorage<T>
where
    T: serde::Serialize + for<'de> serde::Deserialize<'de> + Default,
{
    fn drop(&mut self) {
        if let Err(_) = self.flush_cache() {
            eprintln!("Failed to flush cache on drop");
        }
    }
}
