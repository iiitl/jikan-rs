// character.rs
use crate::common::Pagination;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterResponse<T> {
    pub data: T,
    pub pagination: Option<Pagination>,
}


// to implement