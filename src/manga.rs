use crate::{
    character::*,
    common::{DateRange, Images, Pagination},
    misc::*,
    users::*,
    JikanClient, JikanError,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MangaResponse<T> {
    pub data: T,
    pub pagination: Option<Pagination>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manga {
    pub mal_id: i32,
    pub url: String,
    pub images: Images,
    pub title: String,
    pub title_english: Option<String>,
    pub title_japanese: Option<String>,
    pub chapters: Option<i32>,
    pub volumes: Option<i32>,
    pub status: Option<String>,
    pub score: Option<f32>,
    pub synopsis: Option<String>,
    pub published: Option<DateRange>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MangaCharacters {
    pub data: Vec<MangaCharacter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MangaCharacter {
    pub character: Character,
    pub role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MangaNews {
    pub data: Vec<NewsItem>,
    pub pagination: Pagination,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MangaForum {
    pub data: Vec<ForumTopic>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MangaStatistics {
    pub reading: i32,
    pub completed: i32,
    pub on_hold: i32,
    pub dropped: i32,
    pub plan_to_read: i32,
    pub total: i32,
    pub scores: Vec<Score>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoreInfo {
    pub moreinfo: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MangaUserUpdates {
    pub data: Vec<UserUpdate>,
    pub pagination: Pagination,
}

impl JikanClient {
    pub async fn get_manga(&self, id: i32) -> Result<MangaResponse<Manga>, JikanError> {
        self.get(&format!("/manga/{}", id)).await
    }

    pub async fn get_manga_full(&self, id: i32) -> Result<MangaResponse<Manga>, JikanError> {
        self.get(&format!("/manga/{}/full", id)).await
    }

    pub async fn get_manga_characters(&self, id: i32) -> Result<MangaCharacters, JikanError> {
        self.get(&format!("/manga/{}/characters", id)).await
    }

    pub async fn get_manga_news(
        &self,
        id: i32,
        page: Option<u32>,
    ) -> Result<MangaNews, JikanError> {
        let path = match page {
            Some(p) => format!("/manga/{}/news?page={}", id, p),
            None => format!("/manga/{}/news", id),
        };
        self.get(&path).await
    }

    pub async fn get_manga_forum(
        &self,
        id: i32,
        filter: Option<ForumFilter>,
    ) -> Result<MangaForum, JikanError> {
        let path = match filter {
            Some(f) => format!("/manga/{}/forum?filter={:#?}", id, f),
            None => format!("/manga/{}/forum", id),
        };
        self.get(&path).await
    }

    pub async fn get_manga_pictures(
        &self,
        id: i32,
    ) -> Result<MangaResponse<Vec<Picture>>, JikanError> {
        self.get(&format!("/manga/{}/pictures", id)).await
    }

    pub async fn get_manga_statistics(
        &self,
        id: i32,
    ) -> Result<MangaResponse<MangaStatistics>, JikanError> {
        self.get(&format!("/manga/{}/statistics", id)).await
    }

    pub async fn get_manga_moreinfo(&self, id: i32) -> Result<MangaResponse<MoreInfo>, JikanError> {
        self.get(&format!("/manga/{}/moreinfo", id)).await
    }

    pub async fn get_manga_recommendations(
        &self,
        id: i32,
    ) -> Result<MangaResponse<Vec<Recommendation>>, JikanError> {
        self.get(&format!("/manga/{}/recommendations", id)).await
    }

    pub async fn get_manga_userupdates(
        &self,
        id: i32,
        page: Option<u32>,
    ) -> Result<MangaUserUpdates, JikanError> {
        let path = match page {
            Some(p) => format!("/manga/{}/userupdates?page={}", id, p),
            None => format!("/manga/{}/userupdates", id),
        };
        self.get(&path).await
    }

    pub async fn get_manga_reviews(
        &self,
        id: i32,
        page: Option<u32>,
        preliminary: Option<bool>,
        spoilers: Option<bool>,
    ) -> Result<MangaResponse<Vec<Review>>, JikanError> {
        let mut params = Vec::new();

        if let Some(p) = page {
            params.push(format!("page={}", p));
        }
        if let Some(pr) = preliminary {
            params.push(format!("preliminary={}", pr));
        }
        if let Some(sp) = spoilers {
            params.push(format!("spoilers={}", sp));
        }

        let query = if !params.is_empty() {
            format!("?{}", params.join("&"))
        } else {
            String::new()
        };

        self.get(&format!("/manga/{}/reviews{}", id, query)).await
    }

    pub async fn get_manga_relations(
        &self,
        id: i32,
    ) -> Result<MangaResponse<Vec<Relation>>, JikanError> {
        self.get(&format!("/manga/{}/relations", id)).await
    }

    pub async fn get_manga_external(
        &self,
        id: i32,
    ) -> Result<MangaResponse<Vec<ExternalLink>>, JikanError> {
        self.get(&format!("/manga/{}/external", id)).await
    }
}
