use std::path::PathBuf;

const DATA_PATH: &'static str = "../../data";
const TEST_DATA_PATH: &'static str = "../../tmp/test/data";

pub fn get_cards(collection_name: Option<&str>) -> Result<Vec<Card>, Box<dyn std::error::Error>> {
    Ok(Vec::new())
}

pub fn get_card(id: &str) -> Result<Card, Box<dyn std::error::Error>> {
    Ok(Card::default())
}

pub fn create_card(collection_name: &str, front: &str, back: &str) -> Result<String, Box<dyn std::error::Error>> {
    Ok(String::from("hello"))
}

pub fn update_card(id: &str, front: Option<&str>, back: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

pub fn remove_card(id: &str) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

pub fn remove_cards(ids: Vec<&str>) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

pub fn get_collection(name: &str) -> Result<Collection, Box<dyn std::error::Error>> {
    Ok(Collection { name: String::from("default") })
}

pub fn get_collections() -> Result<Vec<Collection>, Box<dyn std::error::Error>> {
    Ok(Vec::new())
}

pub fn create_collection(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

pub fn add_to_collection(name: &str, data: &str) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

pub fn remove_collection(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

pub fn review_collection(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

pub struct Card {
    pub id: Option<String>,
    pub front: Option<String>,
    pub back: Option<String>,
    pub collection_name: Option<String>,
}

impl Default for Card {
    fn default() -> Self {
        Self {
            id: None,
            front: None,
            back: None,
            collection_name: None
        }
    }
}

pub struct Collection {
    pub name: String
}

pub fn get_data_path(env: Env) -> PathBuf {
    match env {
        Env::Test => PathBuf::from(TEST_DATA_PATH),
        _ => PathBuf::from(DATA_PATH)
    }
}

pub enum Env {
    Test,
    Production,
    Staging
}
