#![cfg_attr(docsrs, doc(cfg(feature = "sync")))]
#![warn(missing_docs)]

#[cfg(test)]
use mockito;

#[cfg(feature = "image")]
use image::{load_from_memory, DynamicImage};

#[cfg(feature = "image")]
use std::io::Read;

#[cfg(feature = "image")]
use cacache_sync::{read as cache_read, write as cache_write};
use serde::de::DeserializeOwned;
use ureq::{Agent, AgentBuilder};
use url::Url;

use crate::cache_middleware::CacheMiddleware;

#[cfg(feature = "image")]
use crate::models::shared::HasImage;
use crate::models::tft::Arenas;
use crate::{
    models::{
        champion::ChampionWrapper, Challenges, Champion, Champions, ChampionsFull, Items, Maps,
        MissionAssets, ProfileIcons, Runes, SpellBuffs, SummonerSpells, Translations,
    },
    ClientError,
};

/// Used for building a [Client] with custom options.
pub struct ClientBuilder {
    agent: Option<Agent>,
    cache: Option<String>,
}

///
/// # Examples
///
/// Using a new agent with no caching.
///
/// ```no_run
/// use ddragon::ClientBuilder;
///
/// let client = ClientBuilder::new().build().unwrap();
/// ```
///
/// Using a provided agent, with no caching (unless you provide an agent with
/// its own caching middleware).
///
/// ```no_run
/// use ureq::Agent;
/// use ddragon::ClientBuilder;
///
/// let agent = Agent::new();
/// let client = ClientBuilder::new().agent(agent).build().unwrap();
/// ```
///
/// Using a new agent with full caching.
///
/// ```no_run
/// use ddragon::ClientBuilder;
///
/// let client = ClientBuilder::new().cache("./cache").build().unwrap();
/// ```
///
/// Note: You can use `Client::new("./cache").unwrap()` as a shortcut for
/// the last example.
impl ClientBuilder {
    /// Creates a [ClientBuilder] with no default options set.
    pub fn new() -> Self {
        Self {
            agent: None,
            cache: None,
        }
    }

    /// Configures a custom [Agent] for making network requests.
    pub fn agent(mut self, agent: Agent) -> Self {
        self.agent = Some(agent);
        self
    }

    /// Configures the cache directory to use for anything that gets downlaoded.
    pub fn cache(mut self, cache_dir: &str) -> Self {
        self.cache = Some(cache_dir.to_owned());
        self
    }

    /// Creates a new [Client] instance with the configured options.
    ///
    /// # Notes
    ///
    /// - If a custom agent is specified, you must configure it with the
    /// [CacheMiddleware] middleware also provided by this crate, or you will
    /// not retain any caching behaviour.
    /// - If a custom agent is specified, not specifying a cache directory will
    /// result in images not being cached if you are using the `image` feature.
    pub fn build(self) -> Result<Client, ClientError> {
        let agent = match self.agent {
            Some(a) => a,
            None => match self.cache.clone() {
                Some(dir) => AgentBuilder::new()
                    .middleware(CacheMiddleware::new(&dir))
                    .build(),
                None => Agent::new(),
            },
        };

        #[cfg(not(test))]
        let base_url = Url::parse("https://ddragon.leagueoflegends.com")?;

        #[cfg(test)]
        let base_url = Url::parse(&mockito::server_url())?;

        let version_list = agent
            .get(base_url.join("/api/versions.json")?.as_str())
            .call()
            .map_err(Box::new)?
            .into_json::<Vec<String>>()?;

        let latest_version = version_list
            .get(0)
            .ok_or(ClientError::NoLatestVersion)?
            .to_owned();

        Ok(Client {
            agent,
            version: latest_version,
            base_url,
            cache_directory: self.cache,
        })
    }
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone)]
/// Provides access to the ddragon API.
pub struct Client {
    agent: Agent,
    /// The current version of the API data reported back to us from the API.
    pub version: String,
    base_url: Url,
    cache_directory: Option<String>,
}

macro_rules! create_endpoint {
    ($name:ident, $kind:literal, $path:literal, $ret:ty) => {
        #[doc = concat!(" Returns ", $kind, " data.")]
        #[doc = ""]
        #[doc = " ```no_run"]
        #[doc = " use ddragon::Client;"]
        #[doc = ""]
        #[doc = " let api = Client::new(\"./cache\").unwrap();"]
        #[doc = concat!(" let ", stringify!($name), " = api.", stringify!($name), "().unwrap();")]
        #[doc = " ```"]
        pub fn $name(&self) -> Result<$ret, ClientError> {
            self.get_data::<$ret>(concat!("./", $path, ".json"))
        }
    };
}

impl Client {
    #[deprecated(note = "Use `ClientBuilder::new().agent(agent).build()` instead.")]
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
    pub fn with_agent(agent: Agent) -> Result<Self, ClientError> {
        ClientBuilder::new().agent(agent).build()
    }

    #[deprecated(
        note = "Use `ClientBuilder::new().agent(agent).cache_directory(dir).build()` instead."
    )]
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
    pub fn with_agent_and_cache(agent: Agent, cache_directory: &str) -> Result<Self, ClientError> {
        ClientBuilder::new()
            .agent(agent)
            .cache(cache_directory)
            .build()
    }

    /// Creates a new client with the specified directory as the caching location
    /// for any data the client downloads.
    ///
    /// ```no_run
    /// use ddragon::Client;
    ///
    /// let api = Client::new("./cache").unwrap();
    /// ```
    pub fn new(cache_dir: &str) -> Result<Self, ClientError> {
        ClientBuilder::new().cache(cache_dir).build()
    }

    #[cfg(test)]
    fn new_no_cache() -> Result<Self, ClientError> {
        ClientBuilder::new().build()
    }

    fn get_data_url(&self) -> Result<Url, url::ParseError> {
        self.base_url
            .join(&format!("/cdn/{}/data/en_US/", &self.version))
    }

    fn get_data<T: DeserializeOwned>(&self, endpoint: &str) -> Result<T, ClientError> {
        let joined_url = self.get_data_url()?.join(endpoint)?;
        let request_url = joined_url.as_str();

        self.agent
            .get(request_url)
            .call()
            .map_err(Box::new)?
            .into_json::<T>()
            .map_err(|e| e.into())
    }

    create_endpoint!(challenges, "challenge", "challenges", Challenges);
    create_endpoint!(champions, "champion", "champion", Champions);
    create_endpoint!(
        champions_full,
        "complete champion",
        "championFull",
        ChampionsFull
    );
    create_endpoint!(items, "item", "item", Items);
    create_endpoint!(maps, "map", "map", Maps);
    create_endpoint!(
        mission_assets,
        "mission asset",
        "mission-assets",
        MissionAssets
    );
    create_endpoint!(profile_icons, "profile icon", "profileicon", ProfileIcons);
    create_endpoint!(runes, "rune", "runesReforged", Runes);
    create_endpoint!(spell_buffs, "spell buff", "spellbuffs", SpellBuffs);
    create_endpoint!(
        summoner_spells,
        "summoner_spells",
        "summoner",
        SummonerSpells
    );
    create_endpoint!(translations, "translation", "language", Translations);
    create_endpoint!(tft_arenas, "TFT arena", "tft-arena", Arenas);

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
    pub fn champion(&self, key: &str) -> Result<Champion, ClientError> {
        self.get_data::<ChampionWrapper>(&format!("./champion/{key}.json"))?
            .data
            .get(key)
            .cloned()
            .ok_or(ClientError::NoChampionData)
    }

    #[cfg(feature = "image")]
    fn get_image(&self, path: Url) -> Result<DynamicImage, ClientError> {
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
            .map_err(|e| std::convert::Into::<ClientError>::into(Box::new(e)))?;

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
    pub fn image_of<T: HasImage>(&self, item: &T) -> Result<DynamicImage, ClientError> {
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
    pub fn sprite_of<T: HasImage>(&self, item: &T) -> Result<DynamicImage, ClientError> {
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
