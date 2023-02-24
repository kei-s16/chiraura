use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct Profile {
    pub version: String,
    pub author: String,
    pub domain: String,
    pub description: String,
    pub website: Vec<Item>,
    pub software: Vec<Item>,
}

#[derive(Deserialize)]
pub struct Item {
    pub title: String,
    pub url: String,
    pub description: String,
}
