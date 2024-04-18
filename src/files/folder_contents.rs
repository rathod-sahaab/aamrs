use std::{fs, path::PathBuf};

use anyhow::Result;

pub struct EntryData {
    pub path: PathBuf,
    pub name: String,
}

pub enum Entry {
    Dir(EntryData),
    File(EntryData),
}

pub fn get_folder_contents(path: &PathBuf) -> Result<Vec<Entry>> {
    let entries = fs::read_dir(path)?
        .filter_map(|entry| -> Option<Entry> {
            if entry.is_err() {
                return None;
            }

            let path = entry.unwrap().path();

            let name = path.file_name()?.to_str()?.to_string();

            if path.is_dir() {
                Some(Entry::Dir(EntryData { path, name }))
            } else if path.is_file() {
                Some(Entry::File(EntryData { path, name }))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    Ok(entries)
}
