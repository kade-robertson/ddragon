use anyhow::Context;

#[cfg(test)]
use mockito;

pub struct DDragonClient {
    request_agent: ureq::Agent,
    version: String,
    base_url: String,
}

impl DDragonClient {
    pub fn with_agent(agent: ureq::Agent, base_url: String) -> anyhow::Result<Self> {
        let version_list = agent
            .get(&format!("{base_url}/api/versions.json"))
            .call()?
            .into_json::<Vec<String>>()?;

        let latest_version = version_list
            .get(0)
            .context("Unable to parse an API version.")?;

        Ok(DDragonClient {
            request_agent: agent,
            version: latest_version.to_owned(),
            base_url,
        })
    }

    pub fn new() -> anyhow::Result<Self> {
        let agent = ureq::Agent::new();

        #[cfg(not(test))]
        let base_url = "https://ddragon.leagueoflegends.com".to_owned();

        #[cfg(test)]
        let base_url = mockito::server_url();

        Self::with_agent(agent, base_url)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use mockito::mock;

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
}
