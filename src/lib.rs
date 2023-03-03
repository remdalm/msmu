use auth::config::MsGraphOAuthConfig;
use oauth2::{AccessToken, TokenResponse};
use serde::{Deserialize, Serialize};

pub mod auth;

pub fn run(ms_auth_config: MsGraphOAuthConfig) -> Result<(), Box<dyn std::error::Error>> {
    let binding = auth::get_access_token(ms_auth_config)?;
    let access_token = binding.access_token();
    let client = reqwest::blocking::Client::new();

    list_users(&client, &access_token)?;

    Ok(())
}

// List users in the tenant using the Microsoft Graph API
fn list_users(
    client: &reqwest::blocking::Client,
    access_token: &AccessToken,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://graph.microsoft.com/beta/users";
    let response = client.get(url).bearer_auth(access_token.secret()).send()?;

    if !response.status().is_success() {
        return Err(format!("request failed: {}", response.status()).into());
    }

    println!("response: {:#?}", response);

    let parsed_response: MsGraphBetaBody<MsGraphUser> = response.json()?;
    //let parsed_response: serde_json::Value = response.json()?;

    println!("response: {:#?}", parsed_response);

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct MsGraphUser {
    id: String,
    #[serde(rename = "displayName")]
    display_name: Option<String>,
    mail: Option<String>,
    // #[serde(rename = "userType")]
    // user_type: Option<String>,
}

#[derive(Debug, Deserialize)]
struct MsGraphBetaBody<T> {
    #[serde(rename = "@odata.context")]
    odata_context: String,
    value: Vec<T>,
}
