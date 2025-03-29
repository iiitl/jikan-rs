// character.rs
use crate::{
    anime::*,
    common::{Images, Pagination},
    manga::*,
    people::*,
    JikanClient, JikanError,
};
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
    pub nicknames: Option<Vec<String>>,
    pub favorites: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimeEntry {
    pub role: String,
    pub anime: Anime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullCharacter {
    pub mal_id: i32,
    pub url: String,
    pub images: Images,
    pub name: String,
    pub anime: Option<Vec<AnimeEntry>>,
    pub manga: Option<Vec<MangaEntry>>,
    pub voices: Option<Vec<PersonEntry>>,
    pub favorites: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullCharacterResponse<T> {
    pub data: T,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MangaEntry {
    pub role: String,
    pub manga: Manga,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonEntry {
    pub language: String,
    pub person: Person,
}

pub enum OrderBy {
    MalId,
    Name,
    Favorites,
}

pub enum Sort {
    Asc,
    Desc,
}

impl JikanClient {
    pub async fn get_character_by_id(
        &self,
        id: i32,
    ) -> Result<CharacterResponse<Character>, JikanError> {
        self.get(&format!("/characters/{}", id)).await
    }

    pub async fn get_character_full_by_id(
        &self,
        id: i32,
    ) -> Result<FullCharacterResponse<FullCharacter>, JikanError> {
        self.get(&format!("/characters/{}/full", id)).await
    }

    pub async fn get_character_anime(
        &self,
        id: i32,
    ) -> Result<CharacterResponse<Vec<AnimeEntry>>, JikanError> {
        self.get(&format!("/characters/{}/anime", id)).await
    }

    pub async fn get_character_manga(
        &self,
        id: i32,
    ) -> Result<CharacterResponse<Vec<MangaEntry>>, JikanError> {
        self.get(&format!("/characters/{}/manga", id)).await
    }

    pub async fn get_character_voices(
        &self,
        id: i32,
    ) -> Result<CharacterResponse<Vec<PersonEntry>>, JikanError> {
        self.get(&format!("/characters/{}/voices", id)).await
    }

    pub async fn get_characters(&self) -> Result<CharacterResponse<Vec<Character>>, JikanError> {
        self.get("/characters").await
    }

    pub async fn get_character_pictures(
        &self,
        id: i32,
    ) -> Result<CharacterResponse<Vec<Images>>, JikanError> {
        self.get(&format!("/characters/{}/pictures", id)).await
    }

    pub async fn get_character_search(
        &self,
        page: Option<i32>,
        limit: Option<i32>,
        q: Option<String>,
        order_by: Option<OrderBy>,
        sort: Option<Sort>,
    ) -> Result<CharacterResponse<Vec<Character>>, JikanError> {
        let mut params: Vec<String> = Vec::new();

        if let Some(p) = page {
            params.push(format!("page={}", p));
        }
        if let Some(l) = limit {
            params.push(format!("limit={}", l));
        }
        if let Some(q) = q {
            params.push(format!("q={}", q));
        }
        if let Some(o) = order_by {
            let order = match o {
                OrderBy::MalId => "mal_id",
                OrderBy::Name => "name",
                OrderBy::Favorites => "favorites",
            };
            params.push(format!("order_by={}", order));
        }
        if let Some(s) = sort {
            let sort = match s {
                Sort::Asc => "asc",
                Sort::Desc => "desc",
            };
            params.push(format!("sort={}", sort));
        }
        let query = if !params.is_empty() {
            format!("?{}", params.join("&"))
        } else {
            String::new()
        };
        println!("{}", query);
        self.get(&format!("/characters{}", query)).await
    }
}
