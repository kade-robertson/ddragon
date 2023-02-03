#![cfg_attr(docsrs, doc(cfg(feature = "sync")))]
#![warn(missing_docs)]

use serde::de::DeserializeOwned;
use url::Url;

#[cfg(test)]
use mockito;

use crate::cache_middleware::CacheMiddleware;

use crate::models::shared::HasImage;
use crate::{
    models::{
        champion::ChampionWrapper, Challenges, Champion, Champions, ChampionsFull, Items, Maps,
        MissionAssets, ProfileIcons, Runes, SpellBuffs, SummonerSpells, Translations,
    },
    DDragonClientError,
};

/// Provides access to the ddragon API.
pub struct DDragonClient {
    agent: ureq::Agent,
    /// The current version of the API data reported back to us from the API.
    pub version: String,
    base_url: Url,
}

impl DDragonClient {
    fn create(agent: ureq::Agent, base_url: Url) -> Result<Self, DDragonClientError> {
        let version_list = agent
            .get(base_url.join("/api/versions.json")?.as_str())
            .call()
            .map_err(Box::new)?
            .into_json::<Vec<String>>()?;

        let latest_version = version_list
            .get(0)
            .ok_or(DDragonClientError::NoLatestVersion)?;

        Ok(DDragonClient {
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
    /// use ddragon::DDragonClient;
    ///
    /// let agent = ureq::AgentBuilder::new().build();
    /// let api = DDragonClient::with_agent(agent).unwrap();
    /// ```
    pub fn with_agent(agent: ureq::Agent) -> Result<Self, DDragonClientError> {
        #[cfg(not(test))]
        let base_url = "https://ddragon.leagueoflegends.com".to_owned();

        #[cfg(test)]
        let base_url = mockito::server_url();

        Self::create(agent, Url::parse(&base_url)?)
    }

    /// Creates a new client with the specified directory as the caching location
    /// for any data the client downloads.
    ///
    /// ```no_run
    /// use ddragon::DDragonClient;
    ///
    /// let api = DDragonClient::new("./cache").unwrap();
    /// ```
    pub fn new(cache_dir: &str) -> Result<Self, DDragonClientError> {
        let agent = ureq::AgentBuilder::new()
            .middleware(CacheMiddleware::new(cache_dir))
            .build();
        Self::with_agent(agent)
    }

    #[cfg(test)]
    fn new_no_cache() -> Result<Self, DDragonClientError> {
        let agent = ureq::Agent::new();
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
    /// use ddragon::DDragonClient;
    ///
    /// let api = DDragonClient::new("./cache").unwrap();
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
    /// use ddragon::DDragonClient;
    ///
    /// let api = DDragonClient::new("./cache").unwrap();
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
    /// use ddragon::DDragonClient;
    ///
    /// let api = DDragonClient::new("./cache").unwrap();
    /// let champions = api.champions().unwrap();
    /// ```
    pub fn champions(&self) -> Result<Champions, DDragonClientError> {
        self.get_data::<Champions>("./champion.json")
    }

    /// Returns champion data -- complete version.
    ///
    /// ```no_run
    /// use ddragon::DDragonClient;
    ///
    /// let api = DDragonClient::new("./cache").unwrap();
    /// let champions_full = api.champions_full().unwrap();
    /// ```
    pub fn champions_full(&self) -> Result<ChampionsFull, DDragonClientError> {
        self.get_data::<ChampionsFull>("./championFull.json")
    }

    /// Returns item data.
    ///
    /// ```no_run
    /// use ddragon::DDragonClient;
    ///
    /// let api = DDragonClient::new("./cache").unwrap();
    /// let items = api.items().unwrap();
    /// ```
    pub fn items(&self) -> Result<Items, DDragonClientError> {
        self.get_data::<Items>("./item.json")
    }

    /// Returns map data.
    ///
    /// ```no_run
    /// use ddragon::DDragonClient;
    ///
    /// let api = DDragonClient::new("./cache").unwrap();
    /// let maps = api.maps().unwrap();
    /// ```
    pub fn maps(&self) -> Result<Maps, DDragonClientError> {
        self.get_data::<Maps>("./map.json")
    }

    /// Returns mission asset data.
    ///
    /// ```no_run
    /// use ddragon::DDragonClient;
    ///
    /// let api = DDragonClient::new("./cache").unwrap();
    /// let mission_assets = api.mission_assets().unwrap();
    /// ```
    pub fn mission_assets(&self) -> Result<MissionAssets, DDragonClientError> {
        self.get_data::<MissionAssets>("./mission-assets.json")
    }

    /// Returns profile icon data.
    ///
    /// ```no_run
    /// use ddragon::DDragonClient;
    ///
    /// let api = DDragonClient::new("./cache").unwrap();
    /// let profile_icons = api.profile_icons().unwrap();
    /// ```
    pub fn profile_icons(&self) -> Result<ProfileIcons, DDragonClientError> {
        self.get_data::<ProfileIcons>("./profileicon.json")
    }

    /// Returns rune data.
    ///
    /// ```no_run
    /// use ddragon::DDragonClient;
    ///
    /// let api = DDragonClient::new("./cache").unwrap();
    /// let runes = api.runes().unwrap();
    /// ```
    pub fn runes(&self) -> Result<Runes, DDragonClientError> {
        self.get_data::<Runes>("./runesReforged.json")
    }

    /// Returns spell buff data.
    ///
    /// ```no_run
    /// use ddragon::DDragonClient;
    ///
    /// let api = DDragonClient::new("./cache").unwrap();
    /// let spell_buffs = api.spell_buffs().unwrap();
    /// ```
    pub fn spell_buffs(&self) -> Result<SpellBuffs, DDragonClientError> {
        self.get_data::<SpellBuffs>("./spellbuffs.json")
    }

    /// Returns summoner spell data.
    ///
    /// ```no_run
    /// use ddragon::DDragonClient;
    ///
    /// let api = DDragonClient::new("./cache").unwrap();
    /// let summoner_spells = api.summoner_spells().unwrap();
    /// ```
    pub fn summoner_spells(&self) -> Result<SummonerSpells, DDragonClientError> {
        self.get_data::<SummonerSpells>("./summoner.json")
    }

    /// Returns translation data.
    ///
    /// ```no_run
    /// use ddragon::DDragonClient;
    ///
    /// let api = DDragonClient::new("./cache").unwrap();
    /// let translations = api.translations().unwrap();
    /// ```
    pub fn translations(&self) -> Result<Translations, DDragonClientError> {
        self.get_data::<Translations>("./language.json")
    }

    fn get_image(&self, path: Url) -> Result<ureq::Response, DDragonClientError> {
        self.agent
            .get(path.as_str())
            .call()
            .map_err(|e| Box::new(e).into())
    }

    /// Returns a `ureq::Response` from the request to retrieve image data.
    /// You likely want to use this as a reader (via `.into_reader(&mut buffer)`).
    ///
    /// ```no_run
    /// use ddragon::DDragonClient;
    ///
    /// let api = DDragonClient::new("./cache").unwrap();
    /// let champion = api.champion("MonkeyKing").unwrap();
    /// let image = api.image_of(champion).unwrap();
    /// ```
    pub fn image_of<T: HasImage>(&self, item: &T) -> Result<ureq::Response, DDragonClientError> {
        self.get_image(self.base_url.join(&format!(
            "/cdn/{}/img/{}",
            &self.version,
            item.image_path()
        ))?)
    }

    /// Returns a `ureq::Response` from the request to retrieve sprite data.
    /// You likely want to use this as a reader (via `.into_reader(&mut buffer)`).
    /// Keep in mind that this response will contain a spritesheet image. You
    /// will have to cut out the appropriate piece using the information on
    /// the [Image](crate::models::shared::Image).
    ///
    /// ```no_run
    /// use ddragon::DDragonClient;
    ///
    /// let api = DDragonClient::new("./cache").unwrap();
    /// let champion = api.champion("MonkeyKing").unwrap();
    /// let sprite = api.sprite_of(champion).unwrap();
    /// ```
    pub fn sprite_of<T: HasImage>(&self, item: &T) -> Result<ureq::Response, DDragonClientError> {
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

    impl Default for DDragonClient {
        fn default() -> Self {
            Self {
                agent: ureq::Agent::new(),
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

            let maybe_client = DDragonClient::new_no_cache();

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

            let maybe_client = DDragonClient::new_no_cache();

            assert!(maybe_client.is_ok());
            assert_eq!(maybe_client.unwrap().version, "0.0.0");
        }

        #[test]
        fn result_err_server_unavailable() {
            assert!(DDragonClient::new_no_cache().is_err());
        }

        #[test]
        fn result_err_no_versions_in_list() {
            let _mock = mock("GET", "/api/versions.json")
                .with_status(200)
                .with_header("Content-Type", "application/json")
                .with_body(r#"[]"#)
                .create();

            assert!(DDragonClient::new_no_cache().is_err());
        }

        #[test]
        fn result_err_cannot_deserialize() {
            let _mock = mock("GET", "/api/versions.json")
                .with_status(200)
                .with_body(r#"some non-deserializable content"#)
                .create();

            assert!(DDragonClient::new_no_cache().is_err());
        }
    }

    mod requests {
        use super::*;

        #[test]
        fn get_data_url_constructs_expected_baseurl() {
            let client = DDragonClient::default();
            assert_eq!(
                client.get_data_url().unwrap().as_str(),
                format!("{}/cdn/0.0.0/data/en_US/", mockito::server_url())
            );
        }

        #[test]
        fn get_data_err_if_server_unavailable() {
            let client = DDragonClient::default();
            assert!(client.get_data::<String>("/fake-endpoint").is_err());
        }

        #[test]
        fn get_data_err_if_data_not_deserializable() {
            let _mock = mock("GET", "/cdn/0.0.0/data/en_US/data.json")
                .with_status(200)
                .with_header("Content-Type", "application/json")
                .with_body(r#"no chance to deserialize this"#)
                .create();

            let client = DDragonClient::default();
            assert!(client.get_data::<String>("./data.json").is_err());
        }

        #[test]
        fn get_data_ok_deserializes_to_type() {
            let _mock = mock("GET", "/cdn/0.0.0/data/en_US/data.json")
                .with_status(200)
                .with_header("Content-Type", "application/json")
                .with_body(r#"["value"]"#)
                .create();

            let client = DDragonClient::default();
            assert_eq!(
                client.get_data::<Vec<String>>("./data.json").unwrap(),
                vec!["value".to_owned()]
            );
        }
    }
}
