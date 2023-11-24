use mangadex_api::{v5::upload::upload_session_id::post::UploadImage, MangaDexClient};
use mangadex_api_schema::v5::oauth::ClientInfo;
use mangadex_api_types::{MangaDexDateTime, Password, Username};
use uuid::Uuid;

use std::{
    collections::HashMap,
    env::{set_var, var, VarError},
    path::{Path, PathBuf},
};

use mangadex_api_schema::v5::AuthTokens;

pub type VarResult<T, E = std::io::Error> = Result<T, E>;

const CLIENT_ID: &str = "CLIENT_ID";

const REFRESH_TOKEN: &str = "REFRESH_TOKEN";

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

fn get_refresh_token_from_var() -> VarResult<String> {
    var(REFRESH_TOKEN).map_err(|e| match e {
        VarError::NotPresent => std::io::Error::new(std::io::ErrorKind::NotFound, REFRESH_TOKEN),
        VarError::NotUnicode(e) => std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            e.to_str().unwrap_or_default().to_string(),
        ),
    })
}

async fn init_client() -> anyhow::Result<MangaDexClient> {
    let client_info = get_client_info_from_var()?;
    let mut client = MangaDexClient::default();
    client.set_client_info(&client_info).await?;
    Ok(client)
}

async fn login(client: &MangaDexClient) -> anyhow::Result<()> {
    let user_info: UserInfos = TryFrom::try_from(PreUserInfos::new()?)?;
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
    println!("{}", oauth_res.refresh_token);
    set_var(REFRESH_TOKEN, oauth_res.refresh_token);
    println!("Your refresh token is now settled to the environment variable");
    Ok(())
}

async fn refresh_token(client: &mut MangaDexClient, refresh_token: String) -> anyhow::Result<()> {
    println!("Fetching your access token");
    client
        .set_auth_tokens(&AuthTokens {
            session: Default::default(),
            refresh: refresh_token,
        })
        .await?;
    let oauth_res = client.oauth().refresh().send().await?;
    println!(
        "Your token will expire in {} minutes",
        (oauth_res.expires_in / 60)
    );
    set_var(REFRESH_TOKEN, oauth_res.refresh_token);
    println!("Your refresh token is now settled to the environment variable");
    Ok(())
}

async fn show_user_name(client: &MangaDexClient) -> anyhow::Result<()> {
    let user_info = client.user().me().get().send().await?;
    println!(
        "Welcome User {}/{}!",
        user_info.data.attributes.username, user_info.data.id
    );
    Ok(())
}

async fn init() -> anyhow::Result<MangaDexClient> {
    let mut client = init_client().await?;
    //println!("client_info: {:?}", client_info);

    //println!("user_info : {:?}", user_info);
    if let Ok(refresh) = get_refresh_token_from_var() {
        refresh_token(&mut client, refresh).await?;
    } else {
        println!("{} Not found", REFRESH_TOKEN);
        println!("using login");
        login(&client).await?;
    }

    show_user_name(&client).await?;
    Ok(client)
}

#[derive(Debug, thiserror::Error)]
enum CheckSessionError {
    #[error("An upload session {0} already exists")]
    AlreadyExists(Uuid),
    #[error(transparent)]
    MangadexApiError(#[from] mangadex_api_types::error::Error),
}

async fn check_session(client: &MangaDexClient) -> Result<(), CheckSessionError> {
    match client.upload().get().send().await {
        Ok(i) => Err(CheckSessionError::AlreadyExists(i.body.data.id)),
        Err(e) => {
            if let mangadex_api_types::error::Error::Api(error) = &e {
                if error.errors.iter().any(|er| er.status == 404) {
                    return Ok(());
                }
            }
            Err(CheckSessionError::MangadexApiError(e))
        }
    }
}

async fn check_and_abandon_session_if_exists(
    client: &MangaDexClient,
) -> Result<(), mangadex_api_types::error::Error> {
    if let Err(e) = check_session(client).await {
        match e {
            CheckSessionError::AlreadyExists(id) => abandon(id, client).await?,
            CheckSessionError::MangadexApiError(error) => return Err(error),
        };
    }
    Ok(())
}

async fn abandon(
    session: Uuid,
    client: &MangaDexClient,
) -> Result<(), mangadex_api_types::error::Error> {
    client
        .upload()
        .upload_session_id(session)
        .delete()
        .send()
        .await?;
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = init().await?;
    check_and_abandon_session_if_exists(&client).await?;

    println!("Starting upload session...");

    let _upload = client
        .upload()
        .begin()
        .post()
        .manga_id(Uuid::parse_str("f9c33607-9180-4ba6-b85c-e4b5faee7192")?)
        .add_group_id(Uuid::parse_str("18dadd0b-cbce-41c4-a8a9-5e653780b9ff")?)
        .send()
        .await?;

    let upload_session_id = _upload.data.id;

    println!("Started!");

    let files = vec![
        Path::new("upload-test/1.png").to_path_buf(),
        Path::new("upload-test/2.png").to_path_buf(),
    ];
    println!("Uploading your files...");

    let res_upload = client
        .upload()
        .upload_session_id(upload_session_id)
        .post()
        .files(
            files
                .iter()
                .flat_map(<UploadImage as TryFrom<&PathBuf>>::try_from)
                .collect::<Vec<UploadImage>>(),
        )
        .send()
        .await?;

    if res_upload.errors.is_empty() || !res_upload.data.is_empty() {
        let files_id: HashMap<String, Uuid> = {
            let mut d = res_upload.data.clone();
            d.sort_by(|a, b| {
                a.attributes
                    .original_file_name
                    .cmp(&b.attributes.original_file_name)
            });
            d
        }
        .iter()
        .map(|e| (e.attributes.original_file_name.to_owned(), e.id))
        .collect();

        println!("Uploaded!");
        println!("files_id: {:?}", files_id);
        println!("Commiting...");
        let files_ids = files_id.values().map(Clone::clone).collect::<Vec<Uuid>>();

        eprintln!("{:?}", &files_ids);

        let res = client
            .upload()
            .upload_session_id(upload_session_id)
            .commit()
            .post()
            .page_order(files_ids)
            .chapter(Some(String::from("144")))
            .translated_language(mangadex_api_types::Language::English)
            .send()
            .await?;

        println!("Commited! Congratz! :3");
        println!("Open https://mangadex.org/chapter/{}", res.data.id);
    } else {
        println!("some files upload files");
        println!("number = {}", res_upload.errors.len());
        println!("Canceling session...");
        abandon(upload_session_id, &client).await?;
        println!("Canceled!");
    }
    Ok(())
}
