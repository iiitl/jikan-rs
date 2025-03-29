//random.rs
use crate::{JikanClient, JikanError, anime::*, character::*, manga::*, people::*, users::*};

impl JikanClient {
    pub async fn get_random_anime(&self) -> Result<AnimeResponse<Anime>, JikanError> {
        self.get("/random/anime").await
    }

    pub async fn get_random_manga(&self) -> Result<MangaResponse<Manga>, JikanError> {
        self.get("/random/manga").await
    }

    pub async fn get_random_user(&self) -> Result<UserResponse<User>, JikanError> {
        self.get("/random/users").await
    }

    pub async fn get_random_character(&self) -> Result<CharacterResponse<Character>, JikanError> {
        self.get("/random/characters").await
    }

    pub async fn get_random_person(&self) -> Result<PeopleResponse<Person>, JikanError> {
        self.get("/random/people").await
    }

    pub async fn get_random_people(&self) -> Result<PeopleResponse<Person>, JikanError> {
        // same as api name
        self.get("/random/people").await
    }
}
