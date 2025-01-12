use std::io::{Error, ErrorKind};
use std::path::PathBuf;
use std::str::FromStr;
use std::{fs, io};

use crate::kube::KubeStateData;
use crate::utils;

pub(super) fn change_config_file_content(
    data: &KubeStateData,
    index_file: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Err(err) = backup_config_file(
        data.default_config_name.to_path_buf(),
        data.default_folder_kube.to_path_buf(),
    ) {
        eprintln!("Error backup config file: {}", err);
        return Err(Box::new(Error::new(err.kind(), err.to_string())));
    }
    if let Some(path_file) = data.files.get(index_file - 1) {
        println!("path: {:?}", path_file);
        match fs::read_to_string(path_file.as_path()) {
            Ok(content) => match fs::write(data.default_config_name.as_path(), content) {
                Ok(_) => {
                    println!("Success write");
                }
                Err(e) => {
                    eprintln!("Error write content to config file: {}", e);
                    return Err(Box::new(Error::new(e.kind(), e.to_string())));
                }
            },
            Err(e) => {
                eprintln!("Error read content file index: {}, err: {}", index_file, e);
                return Err(Box::new(Error::new(e.kind(), e.to_string())));
            }
        }
    }
    Ok(())
}

fn backup_config_file(config_path: PathBuf, folder_path: PathBuf) -> io::Result<()> {
    let original_content = fs::read_to_string(config_path.as_path())?;
    let mut backup_file = folder_path;
    backup_file.push("config-bck");
    fs::write(backup_file.as_path(), original_content)?;
    Ok(())
}
