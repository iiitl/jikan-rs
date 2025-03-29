// people.rs
use crate::common::{Images, Pagination};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeopleResponse<T> {
    pub data: T,
    pub pagination: Option<Pagination>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    pub mal_id: i32,
    pub url: String,
    pub images: Images,
    pub name: String,
}

// to implement
