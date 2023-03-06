use oauth2::basic::{BasicClient, BasicTokenType};
use oauth2::reqwest::http_client;
use oauth2::{
    AuthUrl, ClientId, ClientSecret, EmptyExtraTokenFields, RedirectUrl, Scope,
    StandardTokenResponse, TokenUrl,
};
use url::Url;

use super::config::MsGraphOAuthConfig;

pub fn get_access_token(
    ms_auth_config: MsGraphOAuthConfig,
) -> Result<StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>, Box<dyn std::error::Error>>
{
    let token_url = TokenUrl::new(format!(
        "https://login.microsoftonline.com/{}/oauth2/v2.0/token",
        ms_auth_config.tenant_id
    ))
    .expect("Invalid token endpoint URL");

    let client = BasicClient::new(
        ms_auth_config.graph_client_id,
        Some(ms_auth_config.graph_client_secret),
        AuthUrl::new("https://fake.com".to_string()).unwrap(), // not used with client credentials protocol
        Some(token_url),
    );

    let token_result = client
        .exchange_client_credentials()
        .add_scope(Scope::new(
            "https://graph.microsoft.com/.default".to_string(),
        ))
        .request(http_client)?;

    Ok(token_result)
}

fn generate_admin_consent_link(client_id: &ClientId, redirect_url: &RedirectUrl) -> Url {
    let adminconsent_url = format!(
        "https://login.microsoftonline.com/organizations/adminconsent?client_id={}&redirect_uri={}",
        client_id.as_str(),
        redirect_url.as_str()
    );

    Url::parse(&adminconsent_url).unwrap()

    // TODO in main.rs
    // println!(
    //     "Open this URL in your browser:\n{}\n",
    //     adminconsent_url.to_string()
    // );
}
