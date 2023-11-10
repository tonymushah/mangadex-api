use std::env::{var, VarError};

use mangadex_api::MangaDexClient;
use mangadex_api_schema::v5::oauth::ClientInfo;
use mangadex_api_types::{Password, Username};

pub type VarResult<T, E = std::io::Error> = Result<T, E>;

const CLIENT_ID: &str = "CLIENT_ID";

const CLIENT_SECRET: &str = "CLIENT_SECRET";

const USERNAME: &str = "USERNAME_";

const PASSWORD: &str = "PASSWORD_";

#[derive(Debug)]
struct PreUserInfos {
    username: String,
    password: String,
}

impl PreUserInfos {
    fn new() -> VarResult<Self> {
        Ok(Self {
            username: var(USERNAME).map_err(|e| match e {
                VarError::NotPresent => std::io::Error::new(std::io::ErrorKind::NotFound, USERNAME),
                VarError::NotUnicode(e) => std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    e.to_str().unwrap_or_default().to_string(),
                ),
            })?,
            password: var(PASSWORD).map_err(|e| match e {
                VarError::NotPresent => std::io::Error::new(std::io::ErrorKind::NotFound, PASSWORD),
                VarError::NotUnicode(e) => std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    e.to_str().unwrap_or_default().to_string(),
                ),
            })?,
        })
    }
}

#[derive(Debug)]
struct UserInfos {
    username: Username,
    password: Password,
}

impl TryFrom<PreUserInfos> for UserInfos {
    type Error = mangadex_api_types::error::Error;
    fn try_from(value: PreUserInfos) -> Result<Self, Self::Error> {
        Ok(Self {
            username: Username::parse(value.username)?,
            password: Password::parse(value.password)?,
        })
    }
}

fn get_client_info_from_var() -> VarResult<ClientInfo> {
    Ok(ClientInfo {
        client_id: var(CLIENT_ID).map_err(|e| match e {
            VarError::NotPresent => std::io::Error::new(std::io::ErrorKind::NotFound, CLIENT_ID),
            VarError::NotUnicode(e) => std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                e.to_str().unwrap_or_default().to_string(),
            ),
        })?,
        client_secret: var(CLIENT_SECRET).map_err(|e| match e {
            VarError::NotPresent => {
                std::io::Error::new(std::io::ErrorKind::NotFound, CLIENT_SECRET)
            }
            VarError::NotUnicode(e) => std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                e.to_str().unwrap_or_default().to_string(),
            ),
        })?,
    })
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client_info = get_client_info_from_var()?;
    println!("client_info: {:?}", client_info);
    let user_info: UserInfos = TryFrom::try_from(PreUserInfos::new()?)?;
    println!("user_info : {:?}", user_info);

    let mut client = MangaDexClient::default();
    client.set_client_info(&client_info).await?;

    println!("Fetching your access token");
    let oauth_res = client
        .oauth()
        .login()
        .username(user_info.username)
        .password(user_info.password)
        .send()
        .await?;
    println!(
        "Your token will expire in {} minutes",
        (oauth_res.expires_in / 60)
    );

    println!("Fetching your manga feed");

    let feed = client.user().follows().manga().feed().get().send().await?;
    println!("Fetched");
    println!("{:?}", feed);
    Ok(())
}
