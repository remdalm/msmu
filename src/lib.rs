use auth::config::MsOAuthConfig;
use auth::get_access_token;

pub mod auth;

pub fn run(ms_auth_config: MsOAuthConfig) -> Result<(), Box<dyn std::error::Error>> {
    let access_token = auth::get_access_token(ms_auth_config)?;

    Ok(())
}
