use oauth2::{ClientId, ClientSecret, RedirectUrl};
use std::{collections::HashMap, error::Error, fs};
use url::Url;
#[derive(Debug, Clone)]
pub struct MsGraphOAuthConfig {
    pub graph_client_id: ClientId,
    pub graph_client_secret: ClientSecret,
    pub redirect_url: RedirectUrl,
    pub tenant_id: String,
}

impl MsGraphOAuthConfig {
    pub fn new<S: ToString>(
        graph_client_id: S,
        graph_client_secret: S,
        redirect_url: Url,
        tenant_id: S,
    ) -> MsGraphOAuthConfig {
        MsGraphOAuthConfig {
            graph_client_id: ClientId::new(graph_client_id.to_string()),
            graph_client_secret: ClientSecret::new(graph_client_secret.to_string()),
            redirect_url: RedirectUrl::from_url(redirect_url),
            tenant_id: tenant_id.to_string(),
        }
    }

    pub fn from_file(config_file_path: &str) -> Result<MsGraphOAuthConfig, Box<dyn Error>> {
        let contents = fs::read_to_string(config_file_path)?;
        let map = MsGraphOAuthConfig::parse_env_file(&contents)?;
        let redirect_url = Url::parse(map.get("REDIRECT_URL").unwrap())?;
        Ok(MsGraphOAuthConfig::new(
            map.get("MSGRAPH_CLIENT_ID").unwrap(),
            map.get("MSGRAPH_CLIENT_SECRET").unwrap(),
            redirect_url,
            map.get("TENANT_ID").unwrap(),
        ))
    }

    fn parse_env_file(contents: &str) -> Result<HashMap<&str, &str>, &str> {
        let mut map = HashMap::new();

        for line in contents.lines() {
            let mut parts = line.split('=');
            let key = parts.next().unwrap().clone();
            let value = parts.next().unwrap().clone();

            match key {
                "MSGRAPH_CLIENT_ID" => map.insert("MSGRAPH_CLIENT_ID", value),
                "MSGRAPH_CLIENT_SECRET" => map.insert("MSGRAPH_CLIENT_SECRET", value),
                "REDIRECT_URL" => map.insert("REDIRECT_URL", value),
                "TENANT_ID" => map.insert("TENANT_ID", value),
                _ => None,
            };
        }

        if map.len() != 4 {
            return Err("Missing environment variables in config file");
        }

        Ok(map)
    }
}
