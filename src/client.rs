#![cfg_attr(docsrs, doc(cfg(feature = "sync")))]
#![warn(missing_docs)]

#[cfg(test)]
use mockito;

#[cfg(feature = "image")]
use image::{load_from_memory, DynamicImage};

#[cfg(feature = "image")]
use std::io::Read;

use cacache_sync::{read as cache_read, write as cache_write};
use serde::de::DeserializeOwned;
use ureq::{Agent, AgentBuilder};
use url::Url;

use crate::cache_middleware::CacheMiddleware;

use crate::models::shared::HasImage;
use crate::{
    models::{
        champion::ChampionWrapper, Challenges, Champion, Champions, ChampionsFull, Items, Maps,
        MissionAssets, ProfileIcons, Runes, SpellBuffs, SummonerSpells, Translations,
    },
    DDragonClientError,
};

#[derive(Clone)]
/// Provides access to the ddragon API.
pub struct Client {
    agent: Agent,
    /// The current version of the API data reported back to us from the API.
    pub version: String,
    base_url: Url,
    cache_directory: Option<String>,
}

impl Client {
    fn create(
        agent: Agent,
        base_url: Url,
        cache_directory: Option<String>,
    ) -> Result<Self, DDragonClientError> {
        let version_list = agent
            .get(base_url.join("/api/versions.json")?.as_str())
            .call()
            .map_err(Box::new)?
            .into_json::<Vec<String>>()?;

        let latest_version = version_list
            .get(0)
            .ok_or(DDragonClientError::NoLatestVersion)?;

        Ok(Client {
            agent,
            version: latest_version.to_owned(),
            base_url,
            cache_directory,
        })
    }

    /// Creates a new client using a provided agent, in case you may want to
    /// customize the agent behaviour with additional middlewares (or anything
    /// else you might want to do).
    ///
    /// <p style="background:rgba(255,181,77,0.16);padding:0.75em;">
    /// <strong>Warning:</strong> This effectively turns off all automatically
    /// provided caching, for both images and text. [CacheMiddleware] is not
    /// able to cache images effectively because [ureq::Agent] does not support
    /// creating custom responses with non-string bodies at the moment.
    /// [with_agent_and_cache] is preferred and enables image caching.
    /// </p>
    ///
    /// ```no_run
    /// use ddragon::Client;
    ///
    /// let agent = ureq::AgentBuilder::new().build();
    /// let api = Client::with_agent(agent).unwrap();
    /// ```
    pub fn with_agent(agent: Agent) -> Result<Self, DDragonClientError> {
        #[cfg(not(test))]
        let base_url = "https://ddragon.leagueoflegends.com".to_owned();

        #[cfg(test)]
        let base_url = mockito::server_url();

        Self::create(agent, Url::parse(&base_url)?, None)
    }

    /// Creates a new client using a provided agent, in case you may want to
    /// customize the agent behaviour with additional middlewares (or anything
    /// else you might want to do).
    ///
    /// ```no_run
    /// use ddragon::Client;
    ///
    /// let agent = ureq::AgentBuilder::new().build();
    /// let api = Client::with_agent_and_cache(agent, "./cache").unwrap();
    /// ```
    pub fn with_agent_and_cache(
        agent: Agent,
        cache_directory: &str,
    ) -> Result<Self, DDragonClientError> {
        #[cfg(not(test))]
        let base_url = "https://ddragon.leagueoflegends.com".to_owned();

        #[cfg(test)]
        let base_url = mockito::server_url();

        Self::create(
            agent,
            Url::parse(&base_url)?,
            Some(cache_directory.to_owned()),
        )
    }

    /// Creates a new client with the specified directory as the caching location
    /// for any data the client downloads.
    ///
    /// ```no_run
    /// use ddragon::Client;
    ///
    /// let api = Client::new("./cache").unwrap();
    /// ```
    pub fn new(cache_dir: &str) -> Result<Self, DDragonClientError> {
        let agent = AgentBuilder::new()
            .middleware(CacheMiddleware::new(cache_dir))
            .build();
        Self::with_agent_and_cache(agent, cache_dir)
    }

    #[cfg(test)]
    fn new_no_cache() -> Result<Self, DDragonClientError> {
        let agent = Agent::new();
        Self::with_agent(agent)
    }

    fn get_data_url(&self) -> Result<Url, url::ParseError> {
        self.base_url
            .join(&format!("/cdn/{}/data/en_US/", &self.version))
    }

    fn get_data<T: DeserializeOwned>(&self, endpoint: &str) -> Result<T, DDragonClientError> {
        let joined_url = self.get_data_url()?.join(endpoint)?;
        let request_url = joined_url.as_str();

        self.agent
            .get(request_url)
            .call()
            .map_err(Box::new)?
            .into_json::<T>()
            .map_err(|e| e.into())
    }

    /// Returns challenge data.
    ///
    /// ```no_run
    /// use ddragon::Client;
    ///
    /// let api = Client::new("./cache").unwrap();
    /// let challenges = api.challenges().unwrap();
    /// ```
    pub fn challenges(&self) -> Result<Challenges, DDragonClientError> {
        self.get_data::<Challenges>("./challenges.json")
    }

    /// Returns data for a single champion. The champion's name or numeric key
    /// should not be used here -- this should be the key property on the
    /// [Champion] struct. This is usually the name, but differs in a bunch of
    /// cases (e.x. Wukong's key is MonkeyKing).
    ///
    /// ```no_run
    /// use ddragon::Client;
    ///
    /// let api = Client::new("./cache").unwrap();
    /// let wukong = api.champion("MonkeyKing").unwrap();
    /// ```
    pub fn champion(&self, key: &str) -> Result<Champion, DDragonClientError> {
        self.get_data::<ChampionWrapper>(&format!("./champion/{key}.json"))?
            .data
            .get(key)
            .cloned()
            .ok_or(DDragonClientError::NoChampionData)
    }

    /// Returns champion data -- short version.
    ///
    /// ```no_run
    /// use ddragon::Client;
    ///
    /// let api = Client::new("./cache").unwrap();
    /// let champions = api.champions().unwrap();
    /// ```
    pub fn champions(&self) -> Result<Champions, DDragonClientError> {
        self.get_data::<Champions>("./champion.json")
    }

    /// Returns champion data -- complete version.
    ///
    /// ```no_run
    /// use ddragon::Client;
    ///
    /// let api = Client::new("./cache").unwrap();
    /// let champions_full = api.champions_full().unwrap();
    /// ```
    pub fn champions_full(&self) -> Result<ChampionsFull, DDragonClientError> {
        self.get_data::<ChampionsFull>("./championFull.json")
    }

    /// Returns item data.
    ///
    /// ```no_run
    /// use ddragon::Client;
    ///
    /// let api = Client::new("./cache").unwrap();
    /// let items = api.items().unwrap();
    /// ```
    pub fn items(&self) -> Result<Items, DDragonClientError> {
        self.get_data::<Items>("./item.json")
    }

    /// Returns map data.
    ///
    /// ```no_run
    /// use ddragon::Client;
    ///
    /// let api = Client::new("./cache").unwrap();
    /// let maps = api.maps().unwrap();
    /// ```
    pub fn maps(&self) -> Result<Maps, DDragonClientError> {
        self.get_data::<Maps>("./map.json")
    }

    /// Returns mission asset data.
    ///
    /// ```no_run
    /// use ddragon::Client;
    ///
    /// let api = Client::new("./cache").unwrap();
    /// let mission_assets = api.mission_assets().unwrap();
    /// ```
    pub fn mission_assets(&self) -> Result<MissionAssets, DDragonClientError> {
        self.get_data::<MissionAssets>("./mission-assets.json")
    }

    /// Returns profile icon data.
    ///
    /// ```no_run
    /// use ddragon::Client;
    ///
    /// let api = Client::new("./cache").unwrap();
    /// let profile_icons = api.profile_icons().unwrap();
    /// ```
    pub fn profile_icons(&self) -> Result<ProfileIcons, DDragonClientError> {
        self.get_data::<ProfileIcons>("./profileicon.json")
    }

    /// Returns rune data.
    ///
    /// ```no_run
    /// use ddragon::Client;
    ///
    /// let api = Client::new("./cache").unwrap();
    /// let runes = api.runes().unwrap();
    /// ```
    pub fn runes(&self) -> Result<Runes, DDragonClientError> {
        self.get_data::<Runes>("./runesReforged.json")
    }

    /// Returns spell buff data.
    ///
    /// ```no_run
    /// use ddragon::Client;
    ///
    /// let api = Client::new("./cache").unwrap();
    /// let spell_buffs = api.spell_buffs().unwrap();
    /// ```
    pub fn spell_buffs(&self) -> Result<SpellBuffs, DDragonClientError> {
        self.get_data::<SpellBuffs>("./spellbuffs.json")
    }

    /// Returns summoner spell data.
    ///
    /// ```no_run
    /// use ddragon::Client;
    ///
    /// let api = Client::new("./cache").unwrap();
    /// let summoner_spells = api.summoner_spells().unwrap();
    /// ```
    pub fn summoner_spells(&self) -> Result<SummonerSpells, DDragonClientError> {
        self.get_data::<SummonerSpells>("./summoner.json")
    }

    /// Returns translation data.
    ///
    /// ```no_run
    /// use ddragon::Client;
    ///
    /// let api = Client::new("./cache").unwrap();
    /// let translations = api.translations().unwrap();
    /// ```
    pub fn translations(&self) -> Result<Translations, DDragonClientError> {
        self.get_data::<Translations>("./language.json")
    }

    #[cfg(feature = "image")]
    fn get_image(&self, path: Url) -> Result<DynamicImage, DDragonClientError> {
        let cache_key = path.as_str();

        if let Some(cache_dir) = &self.cache_directory {
            if let Ok(image_data) = cache_read(cache_dir, cache_key) {
                return image::load_from_memory(&image_data).map_err(|e| e.into());
            }
        }

        let response = self
            .agent
            .get(cache_key)
            .call()
            .map_err(|e| std::convert::Into::<DDragonClientError>::into(Box::new(e)))?;

        // We don't want to assume we can just read_to_end cleanly, so ideally
        // we get a header telling us how many bytes we can read. If we can't,
        // using 1 as a default means parsing will always fail and produce an
        // error that can be handled elsewhere.
        let image_size_bytes = response
            .header("Content-Length")
            .unwrap_or("1")
            .parse::<u64>()
            .unwrap_or(1);

        let mut image_buffer: Vec<u8> = vec![];
        response
            .into_reader()
            .take(image_size_bytes)
            .read_to_end(&mut image_buffer)?;
        let image_result = load_from_memory(&image_buffer).map_err(|e| e.into());

        if let Some(cache_dir) = &self.cache_directory {
            let _ = cache_write(cache_dir, cache_key, image_buffer);
        }

        image_result
    }

    /// Returns an [image::DynamicImage].
    ///
    /// ```no_run
    /// use ddragon::Client;
    ///
    /// let api = Client::new("./cache").unwrap();
    /// let champion = api.champion("MonkeyKing").unwrap();
    /// let image = api.image_of(&champion).unwrap();
    /// ```
    #[cfg(feature = "image")]
    #[cfg_attr(docsrs, doc(cfg(feature = "image")))]
    pub fn image_of<T: HasImage>(&self, item: &T) -> Result<DynamicImage, DDragonClientError> {
        self.get_image(self.base_url.join(&format!(
            "/cdn/{}/img/{}",
            &self.version,
            item.image_path()
        ))?)
    }

    /// Returns an [image::DynamicImage].
    ///
    /// Keep in mind that this is a spritesheet image. You will have to cut out
    /// the appropriate piece using the information on the [Image](crate::models::shared::Image).
    ///
    /// ```no_run
    /// use ddragon::Client;
    ///
    /// let api = Client::new("./cache").unwrap();
    /// let champion = api.champion("MonkeyKing").unwrap();
    /// let sprite = api.sprite_of(&champion).unwrap();
    /// ```
    #[cfg(feature = "image")]
    #[cfg_attr(docsrs, doc(cfg(feature = "image")))]
    pub fn sprite_of<T: HasImage>(&self, item: &T) -> Result<DynamicImage, DDragonClientError> {
        self.get_image(self.base_url.join(&format!(
            "/cdn/{}/img/{}",
            &self.version,
            item.sprite_path()
        ))?)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use mockito::mock;

    impl Default for Client {
        fn default() -> Self {
            Self {
                agent: ureq::Agent::new(),
                version: "0.0.0".to_owned(),
                base_url: Url::parse(&mockito::server_url()).unwrap(),
                cache_directory: None,
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

            let maybe_client = Client::new_no_cache();

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

            let maybe_client = Client::new_no_cache();

            assert!(maybe_client.is_ok());
            assert_eq!(maybe_client.unwrap().version, "0.0.0");
        }

        #[test]
        fn result_err_server_unavailable() {
            assert!(Client::new_no_cache().is_err());
        }

        #[test]
        fn result_err_no_versions_in_list() {
            let _mock = mock("GET", "/api/versions.json")
                .with_status(200)
                .with_header("Content-Type", "application/json")
                .with_body(r#"[]"#)
                .create();

            assert!(Client::new_no_cache().is_err());
        }

        #[test]
        fn result_err_cannot_deserialize() {
            let _mock = mock("GET", "/api/versions.json")
                .with_status(200)
                .with_body(r#"some non-deserializable content"#)
                .create();

            assert!(Client::new_no_cache().is_err());
        }
    }

    mod requests {
        use super::*;

        #[test]
        fn get_data_url_constructs_expected_baseurl() {
            let client = Client::default();
            assert_eq!(
                client.get_data_url().unwrap().as_str(),
                format!("{}/cdn/0.0.0/data/en_US/", mockito::server_url())
            );
        }

        #[test]
        fn get_data_err_if_server_unavailable() {
            let client = Client::default();
            assert!(client.get_data::<String>("/fake-endpoint").is_err());
        }

        #[test]
        fn get_data_err_if_data_not_deserializable() {
            let _mock = mock("GET", "/cdn/0.0.0/data/en_US/data.json")
                .with_status(200)
                .with_header("Content-Type", "application/json")
                .with_body(r#"no chance to deserialize this"#)
                .create();

            let client = Client::default();
            assert!(client.get_data::<String>("./data.json").is_err());
        }

        #[test]
        fn get_data_ok_deserializes_to_type() {
            let _mock = mock("GET", "/cdn/0.0.0/data/en_US/data.json")
                .with_status(200)
                .with_header("Content-Type", "application/json")
                .with_body(r#"["value"]"#)
                .create();

            let client = Client::default();
            assert_eq!(
                client.get_data::<Vec<String>>("./data.json").unwrap(),
                vec!["value".to_owned()]
            );
        }
    }
}
