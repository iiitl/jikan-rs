use crate::{JikanClient, JikanError};

pub enum GenreFilter {
    Genres,
    ExplicitGenres,
    Themes,
    Demographics,
}

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
    pub async fn get_anime_genres(
        &self,
        filter: Option<GenreFilter>,
    ) -> Result<GenreResponse, JikanError> {
        let mut params = Vec::new();

        if let Some(f) = filter {
            let filter = match f {
                GenreFilter::Genres => "genres",
                GenreFilter::ExplicitGenres => "explicit_genres",
                GenreFilter::Themes => "themes",
                GenreFilter::Demographics => "demographics",
            };
            params.push(format!("filter={}", filter));
        }

        let query = if !params.is_empty() {
            format!("?{}", params.join("&"))
        } else {
            String::new()
        };

        self.get(&format!("/genres/anime{}", query)).await
    }

    pub async fn get_manga_genres(
        &self,
        filter: Option<GenreFilter>,
    ) -> Result<GenreResponse, JikanError> {
        let mut params = Vec::new();

        if let Some(f) = filter {
            let filter = match f {
                GenreFilter::Genres => "genres",
                GenreFilter::ExplicitGenres => "explicit_genres",
                GenreFilter::Themes => "themes",
                GenreFilter::Demographics => "demographics",
            };
            params.push(format!("filter={}", filter));
        }

        let query = if !params.is_empty() {
            format!("?{}", params.join("&"))
        } else {
            String::new()
        };

        self.get(&format!("/genres/manga{}", query)).await
    }
}
