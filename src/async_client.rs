#![cfg_attr(docsrs, doc(cfg(feature = "async")))]
#![warn(missing_docs)]

#[cfg(feature = "image")]
use image::{load_from_memory, DynamicImage};

use http_cache_reqwest::{CACacheManager, Cache, CacheMode, HttpCache};
use reqwest::Client;
use reqwest_middleware::{ClientBuilder as MiddlewareClientBuilder, ClientWithMiddleware};
use serde::de::DeserializeOwned;
use url::Url;

#[cfg(test)]
use mockito;

#[cfg(feature = "image")]
use crate::models::shared::HasImage;

use crate::{
    models::{
        champion::ChampionWrapper, tft::Arenas, Challenges, Champion, Champions, ChampionsFull,
        Items, Maps, MissionAssets, ProfileIcons, Runes, SpellBuffs, SummonerSpells, Translations,
    },
    ClientError,
};

#[derive(Clone)]
enum ClientAgent {
    Plain(Client),
    Middleware(ClientWithMiddleware),
}

/// Used for building an [AsyncClient] with custom options.
pub struct AsyncClientBuilder {
    agent: Option<ClientAgent>,
    cache: Option<String>,
}

///
/// # Examples
///
/// Using a new agent with no caching.
///
/// ```no_run
/// # tokio_test::block_on(async {
/// use ddragon::AsyncClientBuilder;
///
/// let client = AsyncClientBuilder::new().build().await.unwrap();
/// # })
/// ```
///
/// Using a provided agent, with no caching (unless a directory is specified
/// with `.cache()`)
///
/// ```no_run
/// # tokio_test::block_on(async {
/// use reqwest::Client;
/// use ddragon::AsyncClientBuilder;
///
/// let agent = Client::new();
/// let client = AsyncClientBuilder::new().agent(agent).build().await.unwrap();
/// # })
/// ```
///
/// Using a provided agent that already has some middleware configured. In
/// this case, all caching is expected to be handled by the provided agent.
///
/// ```no_run
/// # tokio_test::block_on(async {
/// use reqwest::Client;
/// use reqwest_middleware::ClientBuilder;
/// use ddragon::AsyncClientBuilder;
///
/// let agent = ClientBuilder::new(Client::new()).build();
/// let client = AsyncClientBuilder::new().agent_with_middleware(agent).build().await.unwrap();
/// # })
/// ```
///
/// Using a new agent with full caching.
///
/// ```no_run
/// # tokio_test::block_on(async {
/// use ddragon::AsyncClientBuilder;
///
/// let client = AsyncClientBuilder::new().cache("./cache").build().await.unwrap();
/// # })
/// ```
///
/// Note: You can use `AsyncClient::new("./cache").await.unwrap()` as a shortcut for
/// the last example.
impl AsyncClientBuilder {
    /// Creates an [AsyncClientBuilder] with no default options set.
    pub fn new() -> Self {
        Self {
            agent: None,
            cache: None,
        }
    }

    /// Configures a custom [ClientWithMiddleware] for making network requests.
    /// You must manage any desired caching behaviour.
    pub fn agent_with_middleware(mut self, agent: ClientWithMiddleware) -> Self {
        self.agent = Some(ClientAgent::Middleware(agent));
        self
    }

    /// Configures a custom [Client] for making network requests. A caching
    /// middleware will be wrapped around this if a cache directory is specified.
    pub fn agent(mut self, agent: Client) -> Self {
        self.agent = Some(ClientAgent::Plain(agent));
        self
    }

    /// Configures the cache directory to use for anything that gets downlaoded.
    pub fn cache(mut self, cache_dir: &str) -> Self {
        self.cache = Some(cache_dir.to_owned());
        self
    }

    /// Creates a new [AsyncClient] instance with the configured options.
    ///
    /// # Notes
    ///
    /// - If a custom [Client] is specified, not specifying a cache directory will
    /// result in no content being cached.
    pub async fn build(self) -> Result<AsyncClient, ClientError> {
        let agent = match self.agent {
            Some(a) => a,
            None => ClientAgent::Plain(Client::new()),
        };

        #[cfg(not(test))]
        let base_url = Url::parse("https://ddragon.leagueoflegends.com")?;

        #[cfg(test)]
        let base_url = Url::parse(&mockito::server_url())?;

        let version_list = match agent.clone() {
            ClientAgent::Plain(a) => {
                a.get(base_url.join("/api/versions.json")?.as_str())
                    .send()
                    .await?
                    .json::<Vec<String>>()
                    .await?
            }
            ClientAgent::Middleware(a) => {
                a.get(base_url.join("/api/versions.json")?.as_str())
                    .send()
                    .await?
                    .json::<Vec<String>>()
                    .await?
            }
        };

        let latest_version = version_list
            .get(0)
            .ok_or(ClientError::NoLatestVersion)?
            .to_owned();

        let middleware_agent = match agent {
            ClientAgent::Plain(plain_agent) => match self.cache {
                Some(cache_dir) => MiddlewareClientBuilder::new(plain_agent)
                    .with(Cache(HttpCache {
                        mode: CacheMode::ForceCache,
                        manager: CACacheManager { path: cache_dir },
                        options: None,
                    }))
                    .build(),
                None => MiddlewareClientBuilder::new(plain_agent).build(),
            },
            ClientAgent::Middleware(middleware_agent) => middleware_agent,
        };

        Ok(AsyncClient {
            agent: middleware_agent,
            version: latest_version,
            base_url,
        })
    }
}

impl Default for AsyncClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone)]
/// Provides access to the ddragon API.
pub struct AsyncClient {
    agent: ClientWithMiddleware,
    /// The current version of the API data reported back to us from the API.
    pub version: String,
    base_url: Url,
}

macro_rules! create_endpoint {
    ($name:ident, $kind:literal, $path:literal, $ret:ty) => {
        #[doc = concat!(" Returns ", $kind, " data.")]
        #[doc = ""]
        #[doc = " ```no_run"]
        #[doc = " # tokio_test::block_on(async {"]
        #[doc = " use ddragon::AsyncClient;"]
        #[doc = ""]
        #[doc = " let api = AsyncClient::new(\"./cache\").await.unwrap();"]
        #[doc = concat!(" let ", stringify!($name), " = api.", stringify!($name), "().await.unwrap();")]
        #[doc = " # })"]
        #[doc = " ```"]
        pub async fn $name(&self) -> Result<$ret, ClientError> {
            self.get_data::<$ret>(concat!("./", $path, ".json")).await
        }
    };
}

impl AsyncClient {
    #[deprecated(
        note = "Use `AsyncClientBuilder::new().agent_with_middleware(agent).build()` instead."
    )]
    /// Creates a new client using a provided agent, in case you may want to
    /// customize the agent behaviour with additional middlewares (or anything
    /// else you might want to do)
    ///
    /// ```no_run
    /// # tokio_test::block_on(async {
    /// use ddragon::AsyncClient;
    ///
    /// let plain_agent = reqwest::Client::new();
    /// let agent = reqwest_middleware::ClientBuilder::new(plain_agent).build();
    /// let api = AsyncClient::with_agent(agent).await.unwrap();
    /// # })
    /// ```
    pub async fn with_agent(agent: ClientWithMiddleware) -> Result<Self, ClientError> {
        AsyncClientBuilder::new()
            .agent_with_middleware(agent)
            .build()
            .await
    }

    #[deprecated(note = "Use `AsyncClientBuilder::new().agent(agent).build()` instead.")]
    /// Creates a new client using a provided agent, in case you want to bypass
    /// any caching mechanics.
    ///
    /// ```no_run
    /// # tokio_test::block_on(async {
    /// use ddragon::AsyncClient;
    ///
    /// let plain_agent = reqwest::Client::new();
    /// let api = AsyncClient::with_plain_agent(plain_agent).await.unwrap();
    /// # })
    /// ```
    pub async fn with_plain_agent(agent: Client) -> Result<Self, ClientError> {
        AsyncClientBuilder::new().agent(agent).build().await
    }

    /// Creates a new client with the specified directory as the caching location
    /// for any data the client downloads.
    ///
    /// ```no_run
    /// # tokio_test::block_on(async {
    /// use ddragon::AsyncClient;
    ///
    /// let api = AsyncClient::new("./cache").await.unwrap();
    /// # })
    /// ```
    pub async fn new(cache_dir: &str) -> Result<Self, ClientError> {
        AsyncClientBuilder::new().cache(cache_dir).build().await
    }

    fn get_data_url(&self) -> Result<Url, url::ParseError> {
        self.base_url
            .join(&format!("/cdn/{}/data/en_US/", &self.version))
    }

    async fn get_data<T: DeserializeOwned>(&self, endpoint: &str) -> Result<T, ClientError> {
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
    /// Champion struct. This is usually the name, but differs in a bunch of
    /// cases (e.x. Wukong's key is MonkeyKing).
    ///
    /// ```no_run
    /// # tokio_test::block_on(async {
    /// use ddragon::AsyncClient;
    ///
    /// let api = AsyncClient::new("./cache").await.unwrap();
    /// let wukong = api.champion("MonkeyKing").await.unwrap();
    /// # })
    /// ```
    pub async fn champion(&self, key: &str) -> Result<Champion, ClientError> {
        self.get_data::<ChampionWrapper>(&format!("./champion/{key}.json"))
            .await?
            .data
            .get(key)
            .cloned()
            .ok_or(ClientError::NoChampionData)
    }

    #[cfg(feature = "image")]
    async fn get_image(&self, path: Url) -> Result<DynamicImage, ClientError> {
        let response = self
            .agent
            .get(path.as_str())
            .send()
            .await
            .map_err(std::convert::Into::<ClientError>::into)?;

        load_from_memory(&response.bytes().await?).map_err(|e| e.into())
    }

    /// Returns an [image::DynamicImage].
    ///
    /// ```no_run
    /// # tokio_test::block_on(async {
    /// use ddragon::AsyncClient;
    ///
    /// let api = AsyncClient::new("./cache").await.unwrap();
    /// let champion = api.champion("MonkeyKing").await.unwrap();
    /// let image = api.image_of(&champion).await.unwrap();
    /// # })
    /// ```
    #[cfg(feature = "image")]
    #[cfg_attr(docsrs, doc(cfg(feature = "image")))]
    pub async fn image_of<T: HasImage>(&self, item: &T) -> Result<DynamicImage, ClientError> {
        self.get_image(self.base_url.join(&format!(
            "/cdn/{}/img/{}",
            &self.version,
            item.image_path()
        ))?)
        .await
    }

    /// Returns an [image::DynamicImage].
    ///
    /// Keep in mind that this is a spritesheet image. You will have to cut out
    /// the appropriate piece using the information on the [Image](crate::models::shared::Image).
    ///
    /// ```no_run
    /// # tokio_test::block_on(async {
    /// use ddragon::AsyncClient;
    ///
    /// let api = AsyncClient::new("./cache").await.unwrap();
    /// let champion = api.champion("MonkeyKing").await.unwrap();
    /// let sprite = api.sprite_of(&champion).await.unwrap();
    /// # })
    /// ```
    #[cfg(feature = "image")]
    #[cfg_attr(docsrs, doc(cfg(feature = "image")))]
    pub async fn sprite_of<T: HasImage>(&self, item: &T) -> Result<DynamicImage, ClientError> {
        self.get_image(self.base_url.join(&format!(
            "/cdn/{}/img/{}",
            &self.version,
            item.sprite_path()
        ))?)
        .await
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use mockito::mock;
    use tokio_test::block_on;

    impl Default for AsyncClient {
        fn default() -> Self {
            Self {
                agent: MiddlewareClientBuilder::new(Client::new()).build(),
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

            let maybe_client = block_on(AsyncClientBuilder::new().build());

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

            let maybe_client = block_on(AsyncClientBuilder::new().build());

            assert!(maybe_client.is_ok());
            assert_eq!(maybe_client.unwrap().version, "0.0.0");
        }

        #[test]
        fn result_err_server_unavailable() {
            assert!(block_on(AsyncClientBuilder::new().build()).is_err());
        }

        #[test]
        fn result_err_no_versions_in_list() {
            let _mock = mock("GET", "/api/versions.json")
                .with_status(200)
                .with_header("Content-Type", "application/json")
                .with_body(r#"[]"#)
                .create();

            assert!(block_on(AsyncClientBuilder::new().build()).is_err());
        }

        #[test]
        fn result_err_cannot_deserialize() {
            let _mock = mock("GET", "/api/versions.json")
                .with_status(200)
                .with_body(r#"some non-deserializable content"#)
                .create();

            assert!(block_on(AsyncClientBuilder::new().build()).is_err());
        }
    }

    mod requests {
        use super::*;

        #[test]
        fn get_data_url_constructs_expected_baseurl() {
            let client = AsyncClient::default();
            assert_eq!(
                client.get_data_url().unwrap().as_str(),
                format!("{}/cdn/0.0.0/data/en_US/", mockito::server_url())
            );
        }

        #[test]
        fn get_data_err_if_server_unavailable() {
            let client = AsyncClient::default();
            assert!(block_on(client.get_data::<String>("/fake-endpoint")).is_err());
        }

        #[test]
        fn get_data_err_if_data_not_deserializable() {
            let _mock = mock("GET", "/cdn/0.0.0/data/en_US/data.json")
                .with_status(200)
                .with_header("Content-Type", "application/json")
                .with_body(r#"no chance to deserialize this"#)
                .create();

            let client = AsyncClient::default();
            assert!(block_on(client.get_data::<String>("./data.json")).is_err());
        }

        #[test]
        fn get_data_ok_deserializes_to_type() {
            let _mock = mock("GET", "/cdn/0.0.0/data/en_US/data.json")
                .with_status(200)
                .with_header("Content-Type", "application/json")
                .with_body(r#"["value"]"#)
                .create();

            let client = AsyncClient::default();
            assert_eq!(
                block_on(client.get_data::<Vec<String>>("./data.json")).unwrap(),
                vec!["value".to_owned()]
            );
        }
    }
}
