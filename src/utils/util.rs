use std::fs;
use std::path::Path;
use crate::schema::entity::Entity;

pub const FILE_PATH: &str = "storage/storage.json"; 

pub fn load_storage() -> Vec<Entity> {
    if !Path::new(FILE_PATH).exists() {
        panic!("The storage path does not exist");
    }

    let content = fs::read_to_string(FILE_PATH).expect("Failed to read file");
    
    // let result:Vec<Entity> = serde_json::from_str(&content).expect("Failed to deserialize, no brackets `[]`");
    let result = match serde_json::from_str(&content) {
        Ok(e) => e,
        Err(_) => {
            fs::write(FILE_PATH, "[]");
            vec![]
        }
    };
    
    result
}

pub fn save_data(data: &[Entity]) -> std::io::Result<()> {
    let json = match serde_json::to_string_pretty(data) {
        Ok(data) => data,
        Err(_) => return Err(std::io::Error::other("Failed to serialize file"))
    };

    let write = match  fs::write(FILE_PATH, json) {
        Ok(()) => (),
        Err(_) => return Err(std::io::Error::other("Failed to write file"))
    };

    Ok(write)
}