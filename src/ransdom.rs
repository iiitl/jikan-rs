use crate::{
    anime::{Anime, Character, Person, ReviewUser},
    manga::Manga,
    common::Pagination,
    JikanClient, JikanError,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct randomreq<T> {
    pub data: T,
    pub pagination: Option<Pagination>,
}

impl JikanClient {
    pub async fn random_anime(&self) -> Result<randomreq<Anime>, JikanError> {
        self.get("/random/anime").await
    }

    pub async fn random_manga(&self) -> Result<randomreq<Manga>, JikanError> {
        self.get("/random/manga").await
    }

    pub async fn random_character(&self) -> Result<randomreq<Character>, JikanError> {
        self.get("/random/characters").await
    }

    pub async fn random_person(&self) -> Result<randomreq<Person>, JikanError> {
        self.get("/random/people").await
    }

    pub async fn random_user(&self) -> Result<randomreq<ReviewUser>, JikanError> {
        self.get("/random/users").await
    }
}
