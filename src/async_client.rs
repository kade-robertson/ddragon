#![warn(missing_docs)]

use http_cache_reqwest::{CACacheManager, Cache, CacheMode, HttpCache};
use reqwest::Client;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use serde::de::DeserializeOwned;
use url::Url;

#[cfg(test)]
use mockito;

use crate::{
    models::{
        champion::ChampionWrapper, Challenges, Champion, Champions, ChampionsFull, Items, Maps,
        MissionAssets, ProfileIcons, Runes, SpellBuffs, SummonerSpells, Translations,
    },
    DDragonClientError,
};

/// Provides access to the ddragon API.
pub struct AsyncDDragonClient {
    agent: ClientWithMiddleware,
    /// The current version of the API data reported back to us from the API.
    pub version: String,
    base_url: Url,
}

impl AsyncDDragonClient {
    async fn create(
        agent: ClientWithMiddleware,
        base_url: Url,
    ) -> Result<Self, DDragonClientError> {
        let version_list = agent
            .get(base_url.join("/api/versions.json")?.as_str())
            .send()
            .await?
            .json::<Vec<String>>()
            .await?;

        let latest_version = version_list
            .get(0)
            .ok_or(DDragonClientError::NoLatestVersion)?;

        Ok(AsyncDDragonClient {
            agent,
            version: latest_version.to_owned(),
            base_url,
        })
    }

    /// Creates a new client using a provided agent, in case you may want to
    /// customize the agent behaviour with additional middlewares (or anything
    /// else you might want to do)
    ///
    /// ```no_run
    /// let plain_agent = reqwest::Client::new();
    /// let agent = reqwest_middleware::ClientBuilder::new(plain_agent).build();
    /// let api = AsyncDDragonClient::with_agent(agent);
    /// ```
    pub async fn with_agent(agent: ClientWithMiddleware) -> Result<Self, DDragonClientError> {
        #[cfg(not(test))]
        let base_url = "https://ddragon.leagueoflegends.com".to_owned();

        #[cfg(test)]
        let base_url = mockito::server_url();

        Self::create(agent, Url::parse(&base_url)?).await
    }

    /// Creates a new client using a provided agent, in case you want to bypass
    /// any caching mechanics.
    ///
    /// ```no_run
    /// let plain_agent = reqwest::Client::new();
    /// let api = AsyncDDragonClient::with_plain_agent(plain_agent);
    /// ```
    pub async fn with_plain_agent(agent: Client) -> Result<Self, DDragonClientError> {
        Self::with_agent(ClientBuilder::new(agent).build()).await
    }

    /// Creates a new client with the specified directory as the caching location
    /// for any data the client downloads.
    ///
    /// ```no_run
    /// let api = AsyncDDragonClient::new("./cache");
    /// ```
    pub async fn new(cache_dir: &str) -> Result<Self, DDragonClientError> {
        let agent = ClientBuilder::new(Client::new())
            .with(Cache(HttpCache {
                mode: CacheMode::ForceCache,
                manager: CACacheManager {
                    path: cache_dir.to_owned(),
                },
                options: None,
            }))
            .build();
        Self::with_agent(agent).await
    }

    fn get_data_url(&self) -> Result<Url, url::ParseError> {
        self.base_url
            .join(&format!("/cdn/{}/data/en_US/", &self.version))
    }

    async fn get_data<T: DeserializeOwned>(&self, endpoint: &str) -> Result<T, DDragonClientError> {
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
    ///
    /// ```no_run
    /// let api = AsyncDDragonClient::new("./cache");
    /// let challenges = api.challenges().await.unwrap();
    /// ```
    pub async fn challenges(&self) -> Result<Challenges, DDragonClientError> {
        self.get_data::<Challenges>("./challenges.json").await
    }

    /// Returns data for a single champion. The champion's name or numeric key
    /// should not be used here -- this should be the key property on the
    /// Champion struct. This is usually the name, but differs in a bunch of
    /// cases (e.x. Wukong's key is MonkeyKing).
    ///
    /// ```no_run
    /// let api = AsyncDDragonClient::new("./cache");
    /// let wukong = api.champion("MonkeyKing").await.unwrap();
    /// ```
    pub async fn champion(&self, key: &str) -> Result<Champion, DDragonClientError> {
        self.get_data::<ChampionWrapper>(&format!("./champion/{key}.json"))
            .await?
            .data
            .get(key)
            .cloned()
            .ok_or(DDragonClientError::NoChampionData)
    }

    /// Returns champion data -- short version.
    ///
    /// ```no_run
    /// let api = AsyncDDragonClient::new("./cache");
    /// let champions = api.champions().await.unwrap();
    /// ```
    pub async fn champions(&self) -> Result<Champions, DDragonClientError> {
        self.get_data::<Champions>("./champion.json").await
    }

    /// Returns champion data -- complete version.
    ///
    /// ```no_run
    /// let api = AsyncDDragonClient::new("./cache");
    /// let champions_full = api.champions_full().await.unwrap();
    /// ```
    pub async fn champions_full(&self) -> Result<ChampionsFull, DDragonClientError> {
        self.get_data::<ChampionsFull>("./championFull.json").await
    }

    /// Returns item data.
    ///
    /// ```no_run
    /// let api = AsyncDDragonClient::new("./cache");
    /// let items = api.items().await.unwrap();
    /// ```
    pub async fn items(&self) -> Result<Items, DDragonClientError> {
        self.get_data::<Items>("./item.json").await
    }

    /// Returns map data.
    ///
    /// ```no_run
    /// let api = AsyncDDragonClient::new("./cache");
    /// let maps = api.maps().await.unwrap();
    /// ```
    pub async fn maps(&self) -> Result<Maps, DDragonClientError> {
        self.get_data::<Maps>("./map.json").await
    }

    /// Returns mission asset data.
    ///
    /// ```no_run
    /// let api = AsyncDDragonClient::new("./cache");
    /// let mission_assets = api.mission_assets().await.unwrap();
    /// ```
    pub async fn mission_assets(&self) -> Result<MissionAssets, DDragonClientError> {
        self.get_data::<MissionAssets>("./mission-assets.json")
            .await
    }

    /// Returns profile icon data.
    ///
    /// ```no_run
    /// let api = AsyncDDragonClient::new("./cache");
    /// let profile_icons = api.profile_icons().await.unwrap();
    /// ```
    pub async fn profile_icons(&self) -> Result<ProfileIcons, DDragonClientError> {
        self.get_data::<ProfileIcons>("./profileicon.json").await
    }

    /// Returns rune data.
    ///
    /// ```no_run
    /// let api = AsyncDDragonClient::new("./cache");
    /// let runes = api.runes().await.unwrap();
    /// ```
    pub async fn runes(&self) -> Result<Runes, DDragonClientError> {
        self.get_data::<Runes>("./runesReforged.json").await
    }

    /// Returns spell buff data.
    ///
    /// ```no_run
    /// let api = AsyncDDragonClient::new("./cache");
    /// let spell_buffs = api.spell_buffs().await.unwrap();
    /// ```
    pub async fn spell_buffs(&self) -> Result<SpellBuffs, DDragonClientError> {
        self.get_data::<SpellBuffs>("./spellbuffs.json").await
    }

    /// Returns summoner spell data.
    ///
    /// ```no_run
    /// let api = AsyncDDragonClient::new("./cache");
    /// let summoner_spells = api.summoner_spells().await.unwrap();
    /// ```
    pub async fn summoner_spells(&self) -> Result<SummonerSpells, DDragonClientError> {
        self.get_data::<SummonerSpells>("./summoner.json").await
    }

    /// Returns translation data.
    ///
    /// ```no_run
    /// let api = AsyncDDragonClient::new("./cache");
    /// let translations = api.translations().await.unwrap();
    /// ```
    pub async fn translations(&self) -> Result<Translations, DDragonClientError> {
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
                agent: ClientBuilder::new(Client::new()).build(),
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

            let maybe_client = block_on(AsyncDDragonClient::with_plain_agent(Client::new()));

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

            let maybe_client = block_on(AsyncDDragonClient::with_plain_agent(Client::new()));

            assert!(maybe_client.is_ok());
            assert_eq!(maybe_client.unwrap().version, "0.0.0");
        }

        #[test]
        fn result_err_server_unavailable() {
            assert!(block_on(AsyncDDragonClient::with_plain_agent(Client::new())).is_err());
        }

        #[test]
        fn result_err_no_versions_in_list() {
            let _mock = mock("GET", "/api/versions.json")
                .with_status(200)
                .with_header("Content-Type", "application/json")
                .with_body(r#"[]"#)
                .create();

            assert!(block_on(AsyncDDragonClient::with_plain_agent(Client::new())).is_err());
        }

        #[test]
        fn result_err_cannot_deserialize() {
            let _mock = mock("GET", "/api/versions.json")
                .with_status(200)
                .with_body(r#"some non-deserializable content"#)
                .create();

            assert!(block_on(AsyncDDragonClient::with_plain_agent(Client::new())).is_err());
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
            assert!(block_on(client.get_data::<String>("/fake-endpoint")).is_err());
        }

        #[test]
        fn get_data_err_if_data_not_deserializable() {
            let _mock = mock("GET", "/cdn/0.0.0/data/en_US/data.json")
                .with_status(200)
                .with_header("Content-Type", "application/json")
                .with_body(r#"no chance to deserialize this"#)
                .create();

            let client = AsyncDDragonClient::default();
            assert!(block_on(client.get_data::<String>("./data.json")).is_err());
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
