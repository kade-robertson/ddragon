#![cfg_attr(docsrs, doc(cfg(feature = "sync")))]
#![warn(missing_docs)]

#[cfg(test)]
use mockito;

#[cfg(feature = "image")]
use image::{load_from_memory, DynamicImage};

#[cfg(feature = "image")]
use std::io::Read;

use serde::de::DeserializeOwned;
use ureq::Agent;
use url::Url;

use crate::cache_middleware::CacheMiddleware;

#[cfg(feature = "image")]
use crate::models::shared::HasImage;
use crate::models::tft::{
    self, Arenas, Augments, HeroAugments, Queues, Regalia, Tacticians, Traits,
};
use crate::{
    models::{
        champion::ChampionWrapper, Challenges, Champion, Champions, ChampionsFull, Items, Maps,
        MissionAssets, ProfileIcons, Runes, SpellBuffs, SummonerSpells, Translations,
    },
    ClientError,
};

/// Used for building a [Client] with custom options.
pub struct ClientBuilder {
    server: String,
    agent: Option<Agent>,
    cache: Option<String>,
    version: Option<String>,
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
/// let agent = Agent::new_with_defaults();
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
            server: "https://ddragon.leagueoflegends.com".to_owned(),
            agent: None,
            cache: None,
            version: None,
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

    /// Configure the ddragon version for making requests. Normally this should
    /// not be needed, as the latest version is always used.
    pub fn version(mut self, version: &str) -> Self {
        self.version = Some(version.to_owned());
        self
    }

    #[cfg(test)]
    pub fn server(mut self, server: &str) -> Self {
        self.server = server.to_owned();
        self
    }

    /// Creates a new [Client] instance with the configured options.
    ///
    /// # Notes
    ///
    /// - If a custom agent is specified, you must configure it with the
    ///   [CacheMiddleware] middleware also provided by this crate, or you will
    ///   not retain any caching behaviour.
    /// - If a custom agent is specified, not specifying a cache directory will
    ///   result in images not being cached if you are using the `image` feature.
    pub fn build(self) -> Result<Client, ClientError> {
        let agent = match self.agent {
            Some(a) => a,
            None => match self.cache.clone() {
                Some(dir) => {
                    Agent::config_builder().middleware(CacheMiddleware::new(&dir)).build().into()
                }
                None => Agent::new_with_defaults(),
            },
        };

        let base_url = Url::parse(&self.server)?;
        let latest_version = if let Some(version) = self.version {
            version
        } else {
            let version_list = agent
                .get(base_url.join("/api/versions.json")?.as_str())
                .call()
                .map_err(Box::new)?
                .into_body()
                .read_json::<Vec<String>>()
                .map_err(Box::new)?;

            version_list.first().ok_or(ClientError::NoLatestVersion)?.to_owned()
        };

        Ok(Client { agent, version: latest_version, base_url })
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

    fn get_data_url(&self) -> Result<Url, url::ParseError> {
        self.base_url.join(&format!("/cdn/{}/data/en_US/", &self.version))
    }

    fn get_data<T: DeserializeOwned>(&self, endpoint: &str) -> Result<T, ClientError> {
        let joined_url = self.get_data_url()?.join(endpoint)?;
        let request_url = joined_url.as_str();

        self.agent
            .get(request_url)
            .call()
            .map_err(Box::new)?
            .into_body()
            .read_json::<T>()
            .map_err(|e| Box::new(e).into())
    }

    create_endpoint!(challenges, "challenge", "challenges", Challenges);
    create_endpoint!(champions, "champion", "champion", Champions);
    create_endpoint!(champions_full, "complete champion", "championFull", ChampionsFull);
    create_endpoint!(items, "item", "item", Items);
    create_endpoint!(maps, "map", "map", Maps);
    create_endpoint!(mission_assets, "mission asset", "mission-assets", MissionAssets);
    create_endpoint!(profile_icons, "profile icon", "profileicon", ProfileIcons);
    create_endpoint!(runes, "rune", "runesReforged", Runes);
    create_endpoint!(spell_buffs, "spell buff", "spellbuffs", SpellBuffs);
    create_endpoint!(summoner_spells, "summoner_spells", "summoner", SummonerSpells);
    create_endpoint!(translations, "translation", "language", Translations);
    create_endpoint!(tft_arenas, "TFT arena", "tft-arena", Arenas);
    create_endpoint!(tft_augments, "TFT augment", "tft-augments", Augments);
    create_endpoint!(tft_champions, "TFT champion", "tft-champion", tft::Champions);
    create_endpoint!(tft_hero_augments, "TFT hero augment", "tft-hero-augments", HeroAugments);
    create_endpoint!(tft_items, "TFT item", "tft-item", tft::Items);
    create_endpoint!(tft_queues, "TFT queue", "tft-queues", Queues);
    create_endpoint!(tft_regalia, "TFT regalia", "tft-regalia", Regalia);
    create_endpoint!(tft_tacticians, "TFT tactician", "tft-tactician", Tacticians);
    create_endpoint!(tft_traits, "TFT trait", "tft-trait", Traits);

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
        let response = self
            .agent
            .get(path.as_str())
            .call()
            .map_err(|e| std::convert::Into::<ClientError>::into(Box::new(e)))?;

        // We don't want to assume we can just read_to_end cleanly, so ideally
        // we get a header telling us how many bytes we can read. If we can't,
        // using 1 as a default means parsing will always fail and produce an
        // error that can be handled elsewhere.
        let image_size_bytes = response
            .headers()
            .get("Content-Length")
            .map(|s| s.to_str().unwrap_or("1"))
            .unwrap_or("1")
            .parse::<u64>()
            .unwrap_or(1);

        let mut image_buffer: Vec<u8> = vec![];
        response.into_body().into_reader().take(image_size_bytes).read_to_end(&mut image_buffer)?;

        load_from_memory(&image_buffer).map_err(|e| e.into())
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
    use mockito::{Server, ServerGuard};

    fn create_mock_client() -> (ServerGuard, String, Client) {
        let server = Server::new();
        let url = server.url();
        (
            server,
            url.clone(),
            Client {
                agent: Agent::new_with_defaults(),
                version: "0.0.0".to_owned(),
                base_url: Url::parse(&url).unwrap(),
            },
        )
    }

    mod create {
        use super::*;

        #[test]
        fn result_ok_if_at_least_one_version() {
            let mut server = Server::new();
            let _mock = server
                .mock("GET", "/api/versions.json")
                .with_status(200)
                .with_header("Content-Type", "application/json")
                .with_body(r#"["0.0.0"]"#)
                .create();

            let maybe_client = ClientBuilder::new().server(&server.url()).build();

            assert!(maybe_client.is_ok());
            assert_eq!(maybe_client.unwrap().version, "0.0.0");
        }

        #[test]
        fn result_ok_first_version_in_list() {
            let mut server = Server::new();
            let _mock = server
                .mock("GET", "/api/versions.json")
                .with_status(200)
                .with_header("Content-Type", "application/json")
                .with_body(r#"["0.0.0", "1.1.1", "2.2.2"]"#)
                .create();

            let maybe_client = ClientBuilder::new().server(&server.url()).build();

            assert!(maybe_client.is_ok());
            assert_eq!(maybe_client.unwrap().version, "0.0.0");
        }

        #[test]
        fn result_err_server_unavailable() {
            assert!(ClientBuilder::new().server("https://a-very-fake.urltogoto").build().is_err());
        }

        #[test]
        fn result_err_no_versions_in_list() {
            let mut server = Server::new();
            let _mock = server
                .mock("GET", "/api/versions.json")
                .with_status(200)
                .with_header("Content-Type", "application/json")
                .with_body(r#"[]"#)
                .create();

            assert!(ClientBuilder::new().server(&server.url()).build().is_err());
        }

        #[test]
        fn result_err_cannot_deserialize() {
            let mut server = Server::new();
            let _mock = server
                .mock("GET", "/api/versions.json")
                .with_status(200)
                .with_body(r#"some non-deserializable content"#)
                .create();

            assert!(ClientBuilder::new().server(&server.url()).build().is_err());
        }

        #[test]
        fn result_ok_manual_version() {
            let mut server = Server::new();
            let _mock = server
                .mock("GET", "/api/versions.json")
                .with_status(200)
                .with_header("Content-Type", "application/json")
                .with_body(r#"["0.0.0", "1.1.1", "2.2.2"]"#)
                .create();

            let maybe_client = ClientBuilder::new().server(&server.url()).version("3.3.3").build();

            assert!(maybe_client.is_ok());
            assert_eq!(maybe_client.unwrap().version, "3.3.3");
        }
    }

    mod requests {
        use super::*;

        #[test]
        fn get_data_url_constructs_expected_baseurl() {
            let (_server, url, client) = create_mock_client();
            assert_eq!(
                client.get_data_url().unwrap().as_str(),
                format!("{}/cdn/0.0.0/data/en_US/", url)
            );
        }

        #[test]
        fn get_data_err_if_server_unavailable() {
            let (_server, _url, client) = create_mock_client();
            assert!(client.get_data::<String>("/fake-endpoint").is_err());
        }

        #[test]
        fn get_data_err_if_data_not_deserializable() {
            let (mut server, _url, client) = create_mock_client();
            let _mock = server
                .mock("GET", "/cdn/0.0.0/data/en_US/data.json")
                .with_status(200)
                .with_header("Content-Type", "application/json")
                .with_body(r#"no chance to deserialize this"#)
                .create();

            assert!(client.get_data::<String>("./data.json").is_err());
        }

        #[test]
        fn get_data_ok_deserializes_to_type() {
            let (mut server, _url, client) = create_mock_client();
            let _mock = server
                .mock("GET", "/cdn/0.0.0/data/en_US/data.json")
                .with_status(200)
                .with_header("Content-Type", "application/json")
                .with_body(r#"["value"]"#)
                .create();

            assert_eq!(
                client.get_data::<Vec<String>>("./data.json").unwrap(),
                vec!["value".to_owned()]
            );
        }
    }
}
