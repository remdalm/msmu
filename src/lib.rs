use auth::config::MsGraphOAuthConfig;
use oauth2::{AccessToken, TokenResponse};
use serde::{Deserialize, Serialize};

pub mod auth;
#[derive(Debug)]
enum UserPurpose {
    User,
    Linked,
    Shared,
    Room,
    Equipment,
    Others,
    UnknownFutureValue,
}

impl UserPurpose {
    fn from_str(s: &str) -> Self {
        match s {
            "user" => UserPurpose::User,
            "linked" => UserPurpose::Linked,
            "shared" => UserPurpose::Shared,
            "room" => UserPurpose::Room,
            "equipment" => UserPurpose::Equipment,
            "others" => UserPurpose::Others,
            "unknownFutureValue" => UserPurpose::UnknownFutureValue,
            _ => panic!("Unknown user purpose: {}", s),
        }
    }
}

pub fn run(ms_auth_config: MsGraphOAuthConfig) -> Result<(), Box<dyn std::error::Error>> {
    let binding = auth::application::get_access_token(ms_auth_config)?;
    println!("Auth Sucessful!");
    let access_token = binding.access_token();
    let client = reqwest::blocking::Client::new();

    let raw_users = list_users(&client, &access_token)?;

    let mut users: Vec<User> = Vec::new();
    let mut shared_mailboxes: Vec<SharedMailbox> = Vec::new();
    // loop raw_users and print get_user_purpose for each user
    for user in raw_users {
        let user_purpose = get_user_purpose(&client, &access_token, &user.id)?;
        match user_purpose {
            Some(UserPurpose::User) => users.push(User::from_ms_graph_user(user)),
            Some(UserPurpose::Shared) => {
                shared_mailboxes.push(SharedMailbox::from_ms_graph_user(user))
            }
            _ => {}
        }
    }

    Ok(())
}

// List users in the tenant using the Microsoft Graph API
fn list_users(
    client: &reqwest::blocking::Client,
    access_token: &AccessToken,
) -> Result<(Vec<MsGraphUser>), Box<dyn std::error::Error>> {
    let url = "https://graph.microsoft.com/beta/users";
    let response = client.get(url).bearer_auth(access_token.secret()).send()?;

    if !response.status().is_success() {
        return Err(format!(
            "request failed in list_users function: {}",
            response.status()
        )
        .into());
    }

    let parsed_response: MsGraphBetaBody<Vec<MsGraphUser>> = response.json()?;

    Ok(parsed_response
        .value
        .into_iter()
        .filter(|user| user.mail.is_some())
        .collect())
}

fn get_user_purpose(
    client: &reqwest::blocking::Client,
    access_token: &AccessToken,
    guid: &str,
) -> Result<Option<UserPurpose>, Box<dyn std::error::Error>> {
    let url = format!(
        "https://graph.microsoft.com/beta/users/{}/mailboxSettings/userPurpose",
        guid
    );
    let response = client.get(url).bearer_auth(access_token.secret()).send()?;

    if !response.status().is_success() {
        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Ok(None);
        }
        return Err(format!(
            "request failed in get_user_purpose function: {}",
            response.status()
        )
        .into());
    }

    let parsed_response: MsGraphBetaBody<String> = response.json()?;

    Ok(Some(UserPurpose::from_str(&parsed_response.value)))
}

#[derive(Debug, Serialize, Deserialize)]
struct MsGraphUser {
    id: String,
    #[serde(rename = "displayName")]
    display_name: Option<String>,
    mail: Option<String>,
}

#[derive(Debug, Deserialize)]
struct MsGraphBetaBody<T> {
    #[serde(rename = "@odata.context")]
    odata_context: String,
    value: T,
}
#[derive(Debug, Serialize, Deserialize)]
struct MsGraphSharedMailbox {
    id: String,
    #[serde(rename = "displayName")]
    display_name: Option<String>,
    mail: Option<String>,
}

trait FromMsGraphUser {
    fn from_ms_graph_user(user: MsGraphUser) -> Self;
}
#[derive(Debug)]
struct User {
    id: String,
    display_name: Option<String>,
    mail: String,
}

impl FromMsGraphUser for User {
    fn from_ms_graph_user(user: MsGraphUser) -> Self {
        User {
            id: user.id,
            display_name: user.display_name,
            mail: user.mail.unwrap(),
        }
    }
}

#[derive(Debug)]
struct SharedMailbox {
    id: String,
    display_name: Option<String>,
    mail: String,
}

impl FromMsGraphUser for SharedMailbox {
    fn from_ms_graph_user(user: MsGraphUser) -> Self {
        SharedMailbox {
            id: user.id,
            display_name: user.display_name,
            mail: user.mail.unwrap(),
        }
    }
}
