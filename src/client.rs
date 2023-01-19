use anyhow::Context;
use serde::de::DeserializeOwned;
use url::Url;

#[cfg(test)]
use mockito;

use crate::models::{champions::Champions, items::Items, translations::Translations};

pub struct DDragonClient {
    agent: ureq::Agent,
    pub version: String,
    base_url: Url,
}

impl DDragonClient {
    pub fn with_agent(agent: ureq::Agent, base_url: Url) -> anyhow::Result<Self> {
        let version_list = agent
            .get(base_url.join("/api/versions.json")?.as_str())
            .call()?
            .into_json::<Vec<String>>()?;

        let latest_version = version_list
            .get(0)
            .context("Unable to parse an API version.")?;

        Ok(DDragonClient {
            agent,
            version: latest_version.to_owned(),
            base_url,
        })
    }

    pub fn new() -> anyhow::Result<Self> {
        let agent = ureq::Agent::new();

        #[cfg(not(test))]
        let base_url = "https://ddragon.leagueoflegends.com";

        #[cfg(test)]
        let base_url = mockito::server_url();

        Self::with_agent(agent, Url::parse(&base_url)?)
    }

    fn get_data_url(&self) -> Result<Url, url::ParseError> {
        self.base_url
            .join(&format!("/cdn/{}/data/en_US/", &self.version))
    }

    fn get_data<T: DeserializeOwned>(&self, endpoint: &str) -> anyhow::Result<T> {
        self.agent
            .get(self.get_data_url()?.join(endpoint)?.as_str())
            .call()?
            .into_json::<T>()
            .map_err(anyhow::Error::msg)
    }

    pub fn champions(&self) -> anyhow::Result<Champions> {
        self.get_data::<Champions>("./champion.json")
    }

    pub fn items(&self) -> anyhow::Result<Items> {
        self.get_data::<Items>("./item.json")
    }

    pub fn translations(&self) -> anyhow::Result<Translations> {
        self.get_data::<Translations>("./language.json")
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

            let maybe_client = DDragonClient::new();

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

            let maybe_client = DDragonClient::new();

            assert!(maybe_client.is_ok());
            assert_eq!(maybe_client.unwrap().version, "0.0.0");
        }

        #[test]
        fn result_err_server_unavailable() {
            assert!(DDragonClient::new().is_err());
        }

        #[test]
        fn result_err_no_versions_in_list() {
            let _mock = mock("GET", "/api/versions.json")
                .with_status(200)
                .with_header("Content-Type", "application/json")
                .with_body(r#"[]"#)
                .create();

            assert!(DDragonClient::new().is_err());
        }

        #[test]
        fn result_err_cannot_deserialize() {
            let _mock = mock("GET", "/api/versions.json")
                .with_status(200)
                .with_body(r#"some non-deserializable content"#)
                .create();

            assert!(DDragonClient::new().is_err());
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
            assert!(client
                .get_data::<serde_json::Value>("/fake-endpoint")
                .is_err());
        }

        #[test]
        fn get_data_err_if_data_not_deserializable() {
            let _mock = mock("GET", "/cdn/0.0.0/data/en_US/data.json")
                .with_status(200)
                .with_header("Content-Type", "application/json")
                .with_body(r#"no chance to deserialize this"#)
                .create();

            let client = DDragonClient::default();
            assert!(client.get_data::<serde_json::Value>("./data.json").is_err());
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
