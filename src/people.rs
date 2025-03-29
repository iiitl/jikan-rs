// people.rs
use crate::common::Pagination;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeopleResponse<T> {
    pub data: T,
    pub pagination: Option<Pagination>,
}


// to implement