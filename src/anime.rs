// anime.rs
use crate::{
    character::*,
    common::{DateRange, Images, Pagination},
    misc::*,
    people::*,
    users::*,
    JikanClient, JikanError,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimeResponse<T> {
    pub data: T,
    pub pagination: Option<Pagination>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Anime {
    pub mal_id: i32,
    pub url: String,
    pub images: Images,
    pub title: String,
    pub title_english: Option<String>,
    pub title_japanese: Option<String>,
    pub episodes: Option<i32>,
    pub status: Option<String>,
    pub score: Option<f32>,
    pub synopsis: Option<String>,
    pub aired: DateRange,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimeCharacters {
    pub data: Vec<AnimeCharacter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimeCharacter {
    pub character: Character,
    pub role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimeStaff {
    pub data: Vec<StaffMember>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaffMember {
    pub person: Person,
    pub positions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimeEpisodes {
    pub data: Vec<Episode>,
    pub pagination: Pagination,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Episode {
    pub mal_id: i32,
    pub url: Option<String>,
    pub title: String,
    pub episode: Option<String>,
    pub aired: Option<String>,
    pub score: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimeVideos {
    pub data: Videos,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Videos {
    pub promo: Vec<PromoVideo>,
    pub episodes: Vec<EpisodeVideo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromoVideo {
    pub title: String,
    pub trailer: VideoMeta,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoMeta {
    pub youtube_id: Option<String>,
    pub url: Option<String>,
    pub embed_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpisodeVideo {
    pub episode: String,
    pub url: String,
    pub title: String,
}

impl JikanClient {
    pub async fn get_anime(&self, id: i32) -> Result<AnimeResponse<Anime>, JikanError> {
        self.get(&format!("/anime/{}", id)).await
    }

    pub async fn get_anime_full(&self, id: i32) -> Result<AnimeResponse<Anime>, JikanError> {
        self.get(&format!("/anime/{}/full", id)).await
    }

    pub async fn get_anime_characters(&self, id: i32) -> Result<AnimeCharacters, JikanError> {
        self.get(&format!("/anime/{}/characters", id)).await
    }

    pub async fn get_anime_staff(&self, id: i32) -> Result<AnimeStaff, JikanError> {
        self.get(&format!("/anime/{}/staff", id)).await
    }

    pub async fn get_anime_episodes(
        &self,
        id: i32,
        page: Option<u32>,
    ) -> Result<AnimeEpisodes, JikanError> {
        let path = match page {
            Some(p) => format!("/anime/{}/episodes?page={}", id, p),
            None => format!("/anime/{}/episodes", id),
        };
        self.get(&path).await
    }

    pub async fn get_anime_episode(
        &self,
        id: i32,
        episode: i32,
    ) -> Result<AnimeResponse<Episode>, JikanError> {
        self.get(&format!("/anime/{}/episodes/{}", id, episode))
            .await
    }

    pub async fn get_anime_videos(&self, id: i32) -> Result<AnimeVideos, JikanError> {
        self.get(&format!("/anime/{}/videos", id)).await
    }

    pub async fn get_anime_statistics(
        &self,
        id: i32,
    ) -> Result<AnimeResponse<AnimeStatistics>, JikanError> {
        self.get(&format!("/anime/{}/statistics", id)).await
    }

    pub async fn get_anime_themes(
        &self,
        id: i32,
    ) -> Result<AnimeResponse<AnimeThemes>, JikanError> {
        self.get(&format!("/anime/{}/themes", id)).await
    }

    pub async fn get_anime_external(
        &self,
        id: i32,
    ) -> Result<AnimeResponse<Vec<ExternalLink>>, JikanError> {
        self.get(&format!("/anime/{}/external", id)).await
    }

    pub async fn get_anime_streaming(
        &self,
        id: i32,
    ) -> Result<AnimeResponse<Vec<ExternalLink>>, JikanError> {
        self.get(&format!("/anime/{}/streaming", id)).await
    }

    pub async fn get_anime_news(
        &self,
        id: i32,
        page: Option<u32>,
    ) -> Result<AnimeNews, JikanError> {
        let path = match page {
            Some(p) => format!("/anime/{}/news?page={}", id, p),
            None => format!("/anime/{}/news", id),
        };
        self.get(&path).await
    }

    pub async fn get_anime_forum(
        &self,
        id: i32,
        filter: Option<ForumFilter>,
    ) -> Result<AnimeForum, JikanError> {
        let path = match filter {
            Some(f) => format!("/anime/{}/forum?filter={:#?}", id, f),
            None => format!("/anime/{}/forum", id),
        };
        self.get(&path).await
    }

    pub async fn get_anime_videos_episodes(
        &self,
        id: i32,
        page: Option<u32>,
    ) -> Result<AnimeVideosEpisodes, JikanError> {
        let path = match page {
            Some(p) => format!("/anime/{}/videos/episodes?page={}", id, p),
            None => format!("/anime/{}/videos/episodes", id),
        };
        self.get(&path).await
    }

    pub async fn get_anime_pictures(
        &self,
        id: i32,
    ) -> Result<AnimeResponse<Vec<Picture>>, JikanError> {
        self.get(&format!("/anime/{}/pictures", id)).await
    }

    pub async fn get_anime_moreinfo(&self, id: i32) -> Result<AnimeResponse<MoreInfo>, JikanError> {
        self.get(&format!("/anime/{}/moreinfo", id)).await
    }

    pub async fn get_anime_recommendations(
        &self,
        id: i32,
    ) -> Result<AnimeResponse<Vec<Recommendation>>, JikanError> {
        self.get(&format!("/anime/{}/recommendations", id)).await
    }

    pub async fn get_anime_userupdates(
        &self,
        id: i32,
        page: Option<u32>,
    ) -> Result<AnimeUserUpdates, JikanError> {
        let path = match page {
            Some(p) => format!("/anime/{}/userupdates?page={}", id, p),
            None => format!("/anime/{}/userupdates", id),
        };
        self.get(&path).await
    }

    pub async fn get_anime_reviews(
        &self,
        id: i32,
        page: Option<u32>,
        preliminary: Option<bool>,
        spoilers: Option<bool>,
    ) -> Result<AnimeResponse<Vec<Review>>, JikanError> {
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

        self.get(&format!("/anime/{}/reviews{}", id, query)).await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimeThemes {
    pub openings: Vec<String>,
    pub endings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimeNews {
    pub data: Vec<NewsItem>,
    pub pagination: Pagination,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimeForum {
    pub data: Vec<ForumTopic>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimeVideosEpisodes {
    pub data: Vec<EpisodeVideo>,
    pub pagination: Pagination,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Picture {
    pub images: Images,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoreInfo {
    pub moreinfo: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimeUserUpdates {
    pub data: Vec<UserUpdate>,
    pub pagination: Pagination,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimeStatistics {
    pub watching: i32,
    pub completed: i32,
    pub on_hold: i32,
    pub dropped: i32,
    pub plan_to_watch: i32,
    pub total: i32,
    pub scores: Vec<Score>,
}
