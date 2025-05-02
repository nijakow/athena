use crate::util::hashing::Sha256;
use std::collections::HashMap;

pub struct EntryInfo {
    pub dirty: bool,
}

pub struct DataStorage<T>
where
    T: serde::Serialize + for<'de> serde::Deserialize<'de> + Default,
{
    base_path: std::path::PathBuf,
    cached: HashMap<Sha256, T>,
    entry_info: HashMap<Sha256, EntryInfo>, // New map to track entry info
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
                cached: HashMap::new(),
                entry_info: HashMap::new(),
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

    fn read(&self, key: &Sha256) -> Result<T, ()> {
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

    fn write(&self, key: &Sha256, data: &T) -> Result<(), ()> {
        let path = self.local_path_for_key(key);

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|_| ())?;
        }

        let file = std::fs::File::create(path).map_err(|_| ())?;
        let writer = std::io::BufWriter::new(file);
        serde_json::to_writer(writer, data).map_err(|_| ())?;

        println!("Wrote data for key {}", key.as_string());

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
        // TODO: Flush parts of the cache if it gets too large
        self.cached.get_mut(key).ok_or(())
    }

    pub fn access<K, F, R>(&mut self, key: K, f: F) -> Result<R, ()>
    where
        K: Into<Sha256>,
        F: FnOnce(&T) -> R,
    {
        let key = key.into();
        let data = self.get(&key)?;
        Ok(f(data))
    }

    pub fn modify<K, F>(&mut self, key: K, f: F) -> Result<(), ()>
    where
        K: Into<Sha256>,
        F: FnOnce(&mut T),
    {
        let key = key.into();
        let mut data = self.read(&key).map_err(|_| ())?;
        f(&mut data);

        // Mark the entry as dirty
        self.entry_info
            .entry(key.clone())
            .or_insert_with(|| EntryInfo { dirty: false })
            .dirty = true;

        self.cached.insert(key, data);
        Ok(())
    }

    pub fn flush_cache(&mut self) -> Result<(), ()> {
        for (key, data) in self.cached.iter() {
            if let Some(info) = self.entry_info.get(key) {
                if info.dirty {
                    self.write(key, data).map_err(|_| ())?;
                    // Mark the entry as clean
                    self.entry_info.get_mut(key).unwrap().dirty = false;
                }
            }
        }

        Ok(())
    }

    pub fn flush_and_clear_cache(&mut self) -> Result<(), ()> {
        let pairs = self.cached.drain().collect::<Vec<_>>();

        for (key, data) in pairs {
            if let Some(info) = self.entry_info.get(&key) {
                if info.dirty {
                    self.write(&key, &data).map_err(|_| ())?;
                }
            }
        }

        self.entry_info.clear(); // Clear entry info

        Ok(())
    }
}

impl<T> Drop for DataStorage<T>
where
    T: serde::Serialize + for<'de> serde::Deserialize<'de> + Default,
{
    fn drop(&mut self) {
        if let Err(_) = self.flush_and_clear_cache() {
            eprintln!("Failed to flush cache on drop");
        }
    }
}
