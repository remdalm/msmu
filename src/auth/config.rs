use std::{collections::HashMap, error::Error, fs};

#[derive(Debug, Clone)]
pub struct MsOAuthConfig {
    pub graph_client_id: String,
    pub graph_client_secret: String,
    pub return_url: String,
}

impl MsOAuthConfig {
    pub fn new<S: ToString>(
        graph_client_id: S,
        graph_client_secret: S,
        return_url: S,
    ) -> MsOAuthConfig {
        MsOAuthConfig {
            graph_client_id: graph_client_id.to_string(),
            graph_client_secret: graph_client_secret.to_string(),
            return_url: return_url.to_string(),
        }
    }

    pub fn from_file(config_file_path: &str) -> Result<MsOAuthConfig, Box<dyn Error>> {
        let contents = fs::read_to_string(config_file_path)?;
        let map = MsOAuthConfig::parse_env_file(&contents)?;
        Ok(MsOAuthConfig::new(
            map.get("MSGRAPH_CLIENT_ID").unwrap(),
            map.get("MSGRAPH_CLIENT_SECRET").unwrap(),
            map.get("RETURN_URL").unwrap(),
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
                "RETURN_URL" => map.insert("RETURN_URL", value),
                _ => None,
            };
        }

        if map.len() != 3 {
            return Err("Missing environment variables in config file");
        }

        Ok(map)
    }
}
