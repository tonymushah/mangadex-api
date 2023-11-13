use mangadex_api::MangaDexClient;
use mangadex_api_schema::v5::oauth::ClientInfo;
use mangadex_api_types::{Password, Username};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut client = MangaDexClient::default();
    client
        .set_client_info(&ClientInfo {
            client_id: String::from("<SET YOUR CLIENT ID HERE>"),
            client_secret: String::from("<SET YOUR CLIENT INFO HERE>"),
        })
        .await?;
    let response = client
        .oauth()
        .login()
        .username(Username::parse("<YOUR USERNAME HERE>")?)
        .password(Password::parse("<YOUR PASSWORD HERE>")?)
        .send()
        .await?;
    /*
       println!("Access Token: {}", response.access_token);
    */
    println!("Expires in {} minutes", response.expires_in / 60);
    Ok(())
}
