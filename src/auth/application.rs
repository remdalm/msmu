use oauth2::basic::{BasicClient, BasicTokenType};
use oauth2::reqwest::http_client;
use oauth2::{
    AuthUrl, ClientId, ClientSecret, EmptyExtraTokenFields, Scope, StandardTokenResponse,
    TokenResponse, TokenUrl,
};
use reqwest::header::AUTHORIZATION;
use serde::{Deserialize, Serialize};

use super::config::MsGraphOAuthConfig;

pub fn get_access_token(
    ms_auth_config: MsGraphOAuthConfig,
) -> Result<StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>, Box<dyn std::error::Error>>
{
    let graph_client_id = ClientId::new(ms_auth_config.graph_client_id);
    let graph_client_secret = ClientSecret::new(ms_auth_config.graph_client_secret);

    let auth_url =
        AuthUrl::new("https://fake.com".to_string()).expect("Invalid authorization endpoint URL");
    let token_url = TokenUrl::new(format!(
        "https://login.microsoftonline.com/{}/oauth2/v2.0/token",
        ms_auth_config.tenant_id
    ))
    .expect("Invalid token endpoint URL");

    // let adminconsent_url = format!(
    //     "https://login.microsoftonline.com/organizations/adminconsent?client_id={}&redirect_uri={}",
    //     graph_client_id.as_str(),
    //     ms_auth_config.redirect_url
    // );
    // println!(
    //     "Open this URL in your browser:\n{}\n",
    //     adminconsent_url.to_string()
    // );

    let client = BasicClient::new(
        graph_client_id,
        Some(graph_client_secret),
        auth_url,
        Some(token_url),
    );

    let token_result = client
        .exchange_client_credentials()
        .add_scope(Scope::new(
            "https://graph.microsoft.com/.default".to_string(),
        ))
        .request(http_client)?;

    // let client = reqwest::blocking::Client::new();
    // let url = "https://login.microsoftonline.com/organization/oauth2/v2.0/token";
    // let token_result = client
    //     .post(url)
    //     .form(&[
    //         ("client_id", graph_client_id.as_str()),
    //         ("client_secret", graph_client_secret.secret()),
    //         ("grant_type", "client_credentials"),
    //         ("scope", "https://graph.microsoft.com/.default"),
    //     ])
    //     .send()?;

    // let token_result = client
    //     .post(url)
    //     .basic_auth(graph_client_id.as_str(), Some(graph_client_secret.secret()))
    //     .form(&[("grant_type", "client_credentials"), ("scope", scope)])
    //     .send();

    // let url = format!(
    //     "https://login.microsoftonline.com/organization/oauth2/v2.0/token?client_id={}&client_secret={}&grant_type=client_credentials&scope={}",
    //     graph_client_id.as_str(),
    //     graph_client_secret.secret(),
    //     scope
    // );

    // // let token_result = reqwest::blocking::get(url)

    Ok(token_result)
}
