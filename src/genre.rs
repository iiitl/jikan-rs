use crate::{
    common::{DateRange, Images, Pagination},
    JikanClient, JikanError,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Genre {
    pub mal_id: u32,
    pub name: String,
    pub url: String,
    pub count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenreResponse {
    pub data: Vec<Genre>,
}

impl JikanClient {
    pub async fn get_anime_genres(&self) -> Result<GenreResponse, JikanError> {
        self.get("/genres/anime").await
    }

    pub async fn get_manga_genres(&self) -> Result<GenreResponse, JikanError> {
        self.get("/genres/manga").await
    }
}
