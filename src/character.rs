// character.rs
use crate::{common::Images, common::Pagination};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterResponse<T> {
    pub data: T,
    pub pagination: Option<Pagination>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    pub mal_id: i32,
    pub url: String,
    pub images: Images,
    pub name: String,
}

// to implement
