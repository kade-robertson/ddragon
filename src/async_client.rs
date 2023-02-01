#![warn(missing_docs)]

use reqwest::Client;
use serde::de::DeserializeOwned;
use thiserror::Error;
use url::Url;

#[cfg(test)]
use mockito;

#[cfg(feature = "local-cache")]
use crate::cache_middleware::CacheMiddleware;

use crate::models::{
    champion::ChampionWrapper, Challenges, Champion, Champions, ChampionsFull, Items, Maps,
    MissionAssets, ProfileIcons, Runes, SpellBuffs, SummonerSpells, Translations,
};

#[derive(Error, Debug)]
/// Any potential error the client may run into during operation.
pub enum AsyncDDragonClientError {
    #[error("Could not parse URL.")]
    /// Indicates the operation failed because parsing a URL via the `url` crate
    /// failed.
    UrlParse(#[from] url::ParseError),
    #[error("Could not complete request.")]
    /// Indicates a request failed, for the same reasons any `reqwest` request may
    /// fail.
    Request(#[from] reqwest::Error),
    #[error("Could not parse JSON data.")]
    /// Indicates a failed attempt at parsing JSON data.
    Parse(#[from] std::io::Error),
    #[error("Could not parse JSON data.")]
    /// Indicates a failed attempt at parsing JSON data.
    JSONParse(#[from] serde_json::Error),
    #[error("Could not find the latest API version.")]
    /// Indicates during instantiation that the version lists provided by the
    /// ddragon API was empty.
    NoLatestVersion,
    #[error("Specific champion data could not be parsed.")]
    /// Indicates data for the requested champion couldn't be found in the
    /// parsed document.
    NoChampionData,
}

/// Provides access to the ddragon API.
pub struct AsyncDDragonClient {
    agent: Client,
    /// The current version of the API data reported back to us from the API.
    pub version: String,
    base_url: Url,
}

impl AsyncDDragonClient {
    async fn create(agent: Client, base_url: Url) -> Result<Self, AsyncDDragonClientError> {
        let version_list = agent
            .get(base_url.join("/api/versions.json")?.as_str())
            .send()
            .await?
            .json::<Vec<String>>()
            .await?;

        let latest_version = version_list
            .get(0)
            .ok_or(AsyncDDragonClientError::NoLatestVersion)?;

        Ok(AsyncDDragonClient {
            agent,
            version: latest_version.to_owned(),
            base_url,
        })
    }

    /// Creates a new client using a provided agent, in case you may want to
    /// customize the agent behaviour with additional middlewares (or anything
    /// else you might want to do)
    pub async fn with_agent(agent: Client) -> Result<Self, AsyncDDragonClientError> {
        #[cfg(not(test))]
        let base_url = "https://ddragon.leagueoflegends.com".to_owned();

        #[cfg(test)]
        let base_url = mockito::server_url();

        Self::create(agent, Url::parse(&base_url)?).await
    }

    #[cfg(feature = "local-cache")]
    /// Creates a new client with the specified directory as the caching location
    /// for any data the client downloads.
    pub async fn new(cache_dir: &str) -> Result<Self, AsyncDDragonClientError> {
        let agent = Client::new();
        Self::with_agent(agent).await
    }

    #[cfg(any(test, not(feature = "local-cache")))]
    async fn new_no_cache() -> Result<Self, AsyncDDragonClientError> {
        let agent = Client::new();
        Self::with_agent(agent).await
    }

    #[cfg(not(feature = "local-cache"))]
    /// Creates a new client without using a local cache.
    pub async fn new() -> Result<Self, AsyncDDragonClientError> {
        Self::new_no_cache().await
    }

    fn get_data_url(&self) -> Result<Url, url::ParseError> {
        self.base_url
            .join(&format!("/cdn/{}/data/en_US/", &self.version))
    }

    async fn get_data<T: DeserializeOwned>(
        &self,
        endpoint: &str,
    ) -> Result<T, AsyncDDragonClientError> {
        let joined_url = self.get_data_url()?.join(endpoint)?;
        let request_url = joined_url.as_str();

        self.agent
            .get(request_url)
            .send()
            .await?
            .json::<T>()
            .await
            .map_err(|e| e.into())
    }

    /// Returns challenge data.
    pub async fn challenges(&self) -> Result<Challenges, AsyncDDragonClientError> {
        self.get_data::<Challenges>("./challenges.json").await
    }

    /// Returns data for a single champion. The champion's name or numeric key
    /// should not be used here -- this should be the key property on the
    /// Champion struct. This is usually the name, but differs in a bunch of
    /// cases (e.x. Wukong's key is MonkeyKing).
    pub async fn champion(&self, key: &str) -> Result<Champion, AsyncDDragonClientError> {
        self.get_data::<ChampionWrapper>(&format!("./champion/{key}.json"))
            .await?
            .data
            .get(key)
            .cloned()
            .ok_or(AsyncDDragonClientError::NoChampionData)
    }

    /// Returns champion data -- short version.
    pub async fn champions(&self) -> Result<Champions, AsyncDDragonClientError> {
        self.get_data::<Champions>("./champion.json").await
    }

    /// Returns champion data -- complete version.
    pub async fn champions_full(&self) -> Result<ChampionsFull, AsyncDDragonClientError> {
        self.get_data::<ChampionsFull>("./championFull.json").await
    }

    /// Returns item data.
    pub async fn items(&self) -> Result<Items, AsyncDDragonClientError> {
        self.get_data::<Items>("./item.json").await
    }

    /// Returns map data.
    pub async fn maps(&self) -> Result<Maps, AsyncDDragonClientError> {
        self.get_data::<Maps>("./map.json").await
    }

    /// Returns mission asset data.
    pub async fn mission_assets(&self) -> Result<MissionAssets, AsyncDDragonClientError> {
        self.get_data::<MissionAssets>("./mission-assets.json")
            .await
    }

    /// Returns profile icon data.
    pub async fn profile_icons(&self) -> Result<ProfileIcons, AsyncDDragonClientError> {
        self.get_data::<ProfileIcons>("./profileicon.json").await
    }

    /// Returns rune data.
    pub async fn runes(&self) -> Result<Runes, AsyncDDragonClientError> {
        self.get_data::<Runes>("./runesReforged.json").await
    }

    /// Returns spell buff data.
    pub async fn spell_buffs(&self) -> Result<SpellBuffs, AsyncDDragonClientError> {
        self.get_data::<SpellBuffs>("./spellbuffs.json").await
    }

    /// Returns summoner spell data.
    pub async fn summoner_spells(&self) -> Result<SummonerSpells, AsyncDDragonClientError> {
        self.get_data::<SummonerSpells>("./summoner.json").await
    }

    /// Returns translation data.
    pub async fn translations(&self) -> Result<Translations, AsyncDDragonClientError> {
        self.get_data::<Translations>("./language.json").await
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use mockito::mock;
    use tokio_test::block_on;

    impl Default for AsyncDDragonClient {
        fn default() -> Self {
            Self {
                agent: Client::new(),
                version: "0.0.0".to_owned(),
                base_url: Url::parse(&mockito::server_url()).unwrap(),
            }
        }
    }

    mod create {
        use super::*;

        #[test]
        fn result_ok_if_at_least_one_version() {
            let _mock = mock("GET", "/api/versions.json")
                .with_status(200)
                .with_header("Content-Type", "application/json")
                .with_body(r#"["0.0.0"]"#)
                .create();

            let maybe_client = block_on(AsyncDDragonClient::new_no_cache());

            assert!(maybe_client.is_ok());
            assert_eq!(maybe_client.unwrap().version, "0.0.0");
        }

        #[test]
        fn result_ok_first_version_in_list() {
            let _mock = mock("GET", "/api/versions.json")
                .with_status(200)
                .with_header("Content-Type", "application/json")
                .with_body(r#"["0.0.0", "1.1.1", "2.2.2"]"#)
                .create();

            let maybe_client = block_on(AsyncDDragonClient::new_no_cache());

            assert!(maybe_client.is_ok());
            assert_eq!(maybe_client.unwrap().version, "0.0.0");
        }

        #[test]
        fn result_err_server_unavailable() {
            assert!(block_on(AsyncDDragonClient::new_no_cache()).is_err());
        }

        #[test]
        fn result_err_no_versions_in_list() {
            let _mock = mock("GET", "/api/versions.json")
                .with_status(200)
                .with_header("Content-Type", "application/json")
                .with_body(r#"[]"#)
                .create();

            assert!(block_on(AsyncDDragonClient::new_no_cache()).is_err());
        }

        #[test]
        fn result_err_cannot_deserialize() {
            let _mock = mock("GET", "/api/versions.json")
                .with_status(200)
                .with_body(r#"some non-deserializable content"#)
                .create();

            assert!(block_on(AsyncDDragonClient::new_no_cache()).is_err());
        }
    }

    mod requests {
        use super::*;

        #[test]
        fn get_data_url_constructs_expected_baseurl() {
            let client = AsyncDDragonClient::default();
            assert_eq!(
                client.get_data_url().unwrap().as_str(),
                format!("{}/cdn/0.0.0/data/en_US/", mockito::server_url())
            );
        }

        #[test]
        fn get_data_err_if_server_unavailable() {
            let client = AsyncDDragonClient::default();
            assert!(block_on(client.get_data::<serde_json::Value>("/fake-endpoint")).is_err());
        }

        #[test]
        fn get_data_err_if_data_not_deserializable() {
            let _mock = mock("GET", "/cdn/0.0.0/data/en_US/data.json")
                .with_status(200)
                .with_header("Content-Type", "application/json")
                .with_body(r#"no chance to deserialize this"#)
                .create();

            let client = AsyncDDragonClient::default();
            assert!(block_on(client.get_data::<serde_json::Value>("./data.json")).is_err());
        }

        #[test]
        fn get_data_ok_deserializes_to_type() {
            let _mock = mock("GET", "/cdn/0.0.0/data/en_US/data.json")
                .with_status(200)
                .with_header("Content-Type", "application/json")
                .with_body(r#"["value"]"#)
                .create();

            let client = AsyncDDragonClient::default();
            assert_eq!(
                block_on(client.get_data::<Vec<String>>("./data.json")).unwrap(),
                vec!["value".to_owned()]
            );
        }
    }
}
