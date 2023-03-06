use oauth2::basic::{BasicClient, BasicTokenType};
use oauth2::reqwest::http_client;
use oauth2::{
    AuthType, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, EmptyExtraTokenFields,
    PkceCodeChallenge, RedirectUrl, Scope, StandardTokenResponse, TokenUrl,
};
use std::io::{BufRead, BufReader, Error, ErrorKind, Write};
use std::net::TcpListener;
use url::Url;

use super::config::MsGraphOAuthConfig;

pub fn get_access_token(
    ms_auth_config: MsGraphOAuthConfig,
) -> Result<StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>, Box<dyn std::error::Error>>
{
    let auth_url = AuthUrl::new(
        "https://login.microsoftonline.com/organizations/oauth2/v2.0/authorize".to_string(),
    )
    .expect("Invalid authorization endpoint URL");
    let token_url = TokenUrl::new(
        "https://login.microsoftonline.com/organizations/oauth2/v2.0/token".to_string(),
    )
    .expect("Invalid token endpoint URL");

    // Set up the config for the Microsoft Graph OAuth2 process.
    let client = BasicClient::new(
        ms_auth_config.graph_client_id,
        Some(ms_auth_config.graph_client_secret),
        auth_url,
        Some(token_url),
    )
    // Microsoft Graph requires client_id and client_secret in URL rather than
    // using Basic authentication.
    .set_auth_type(AuthType::RequestBody)
    // This example will be running its own server at localhost:3003.
    // See below for the server implementation.
    .set_redirect_uri(ms_auth_config.redirect_url);

    // Microsoft Graph supports Proof Key for Code Exchange (PKCE - https://oauth.net/2/pkce/).
    // Create a PKCE code verifier and SHA-256 encode it as a code challenge.
    let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();

    // Generate the authorization URL to which we'll redirect the user.
    let (authorize_url, _csrf_state) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new(
            "User.Read User.ReadBasic.All MailboxSettings.Read".to_string(),
        ))
        .set_pkce_challenge(pkce_code_challenge)
        .url();

    println!(
        "Open this URL in your browser:\n{}\n",
        authorize_url.to_string()
    );

    // Wait for the authorization code.
    let authorization_code = get_authorization_code().unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    });

    println!(
        "Received authorization code: {}\n",
        authorization_code.secret()
    );

    // Exchange the code with a token.
    let token = client
        .exchange_code(authorization_code)
        // Send the PKCE code verifier in the token request
        .set_pkce_verifier(pkce_code_verifier)
        .request(http_client)?;

    Ok(token)
}

fn get_authorization_code() -> Result<AuthorizationCode, Error> {
    // A very naive implementation of the redirect server.
    let mut code = None;
    let listener = TcpListener::bind("127.0.0.1:3003").unwrap();
    for stream in listener.incoming() {
        if let Ok(mut stream) = stream {
            {
                let mut reader = BufReader::new(&stream);

                let mut request_line = String::new();
                reader.read_line(&mut request_line).unwrap();

                let redirect_url = request_line.split_whitespace().nth(1).unwrap();
                let url = Url::parse(&("http://localhost".to_string() + redirect_url)).unwrap();

                let code_pair = url
                    .query_pairs()
                    .find(|pair| {
                        let &(ref key, _) = pair;
                        key == "code"
                    })
                    .unwrap();

                let (_, value) = code_pair;
                code = Some(AuthorizationCode::new(value.into_owned()));
            }

            let message = "Go back to your terminal :)";
            let response = format!(
                "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
                message.len(),
                message
            );
            stream.write_all(response.as_bytes()).unwrap();
            break;
        } else {
            return Err(Error::new(ErrorKind::Other, "Failed to connect"));
        }
    }
    match code {
        Some(code) => Ok(code),
        None => Err(Error::new(ErrorKind::Other, "No code received")),
    }
}
