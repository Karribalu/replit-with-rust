use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use tracing::{error, info};
#[derive(Debug, Serialize, Deserialize)]
pub enum FILETYPE{
    FILE,
    DIR
}
#[derive(Debug, Serialize)]
pub struct File {
    #[serde(rename = "type")]
    file_type: FILETYPE,
    name: String
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileResponse{
    #[serde(rename = "type")]
    file_type: FILETYPE,
    name: String,
    path: String
}
pub fn fetch_dir(dir: &PathBuf, base_dir: &String) -> Vec<FileResponse>{
    let result = std::fs::read_dir(dir);
    match result {
        Ok(files) => {
            let op_files: Vec<FileResponse> = files.map(|item| {
                let entry = item.unwrap();
                let path;
                if(base_dir.eq("/")){
                    path = format!("{}{}", base_dir, entry.file_name().to_str().unwrap())
                }else{
                    path = format!("{}/{}", base_dir, entry.file_name().to_str().unwrap())
                }
                FileResponse{
                    file_type: if entry.file_type().unwrap().is_dir() {
                        FILETYPE::DIR
                    }else{
                        FILETYPE::FILE
                    },
                    name: entry.file_name().into_string().unwrap(),
                    path
                }
            }).collect();
            return op_files;
            info!("all files {:?}", op_files);
        }
        Err(e) => {
            error!("error in fetch dir {}", e);
            vec![]
        }
    }
}

pub fn fetch_file_content(file: &PathBuf) -> String{
    String::from_utf8(std::fs::read(file).unwrap()).unwrap()
}

pub fn save_file(file: String, content: String){
    std::fs::write(file, content).unwrap()
}