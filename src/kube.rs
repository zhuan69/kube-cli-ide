use std::io::Error;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::{env, fs, io};

pub mod command;
pub mod spec;

#[derive(Debug)]
pub(super) struct KubeStateData {
    pub folders: Vec<Arc<PathBuf>>,
    pub files: Vec<Arc<PathBuf>>,
    pub default_folder_kube: PathBuf,
    pub default_config_name: PathBuf,
}

impl KubeStateData {
    pub fn new_kube_data() -> KubeStateData {
        let mut path_literal = env::var("HOME").expect("no home variables defined");
        path_literal.push_str("/.kube");
        let default_folder_kube = PathBuf::from(path_literal.clone());
        path_literal.push_str("/config");
        let default_config_name = PathBuf::from(path_literal.clone());
        KubeStateData {
            folders: Vec::with_capacity(25),
            files: Vec::with_capacity(25),
            default_folder_kube,
            default_config_name,
        }
    }

    pub fn walking_dir_kube(&mut self) {
        let _ = self.walking_dir_recursive(self.default_folder_kube.clone());
    }

    fn walking_dir_recursive(&mut self, path: PathBuf) -> Result<(), Error> {
        match fs::read_dir(path) {
            Ok(entries) => {
                for e in entries {
                    let path_entry = e?;
                    let dir_path = Arc::new(path_entry.path());
                    let dir_path_clone = Arc::clone(&dir_path);
                    if let Some(str_path) = dir_path.to_str() {
                        if str_path.contains("cache") {
                            continue;
                        }
                    }
                    if dir_path.is_dir() {
                        self.folders.push(dir_path.clone());
                        self.walking_dir_recursive(dir_path_clone.to_path_buf())?;
                    }
                    if dir_path.is_file() {
                        self.files.push(dir_path.clone());
                    }
                }
                Ok(())
            }
            Err(err) => {
                println!("{err}");
                Err(Error::new(err.kind(), err))
            }
        }
    }
}
