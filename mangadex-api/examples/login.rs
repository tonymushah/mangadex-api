//! This will log in with the credentials provided upon compilation.
//!
//! Work only with the `oauth` feature and only personal client since public client isn't yet available
//!  
//! You must edit the [`login.rs`](login.rs) file with your credentials in the login builder pattern to use this.
//!
//! # Usage
//!
//! ```
//! login
//! ```

#[cfg(feature = "oauth")]
use mangadex_api::v5::MangaDexClient;
#[cfg(feature = "oauth")]
use mangadex_api_types::{Password, Username};

#[tokio::main]
async fn main() {
    #[cfg(feature = "oauth")]
    if let Err(e) = run().await {
        use std::process;
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
    #[cfg(not(feature = "oauth"))]
    eprintln!("You should enable the `legacy-auth` and `legacy-account` features")
}

#[cfg(feature = "oauth")]
async fn run() -> anyhow::Result<()> {
    // the old login system still work on api.mangadex.dev for now

    use mangadex_api_schema::v5::oauth::ClientInfo;
    let client = MangaDexClient::api_dev_client();

    client
        .set_client_info(&ClientInfo {
            client_secret: "YOUR CLIENT SECRET HERE".into(),
            client_id: "YOUR CLIENT ID HERE".into(),
        })
        .await?;

    let _login_res = client
        .oauth()
        .login()
        // You may also use an email address by replacing `username()` with `email()`.
        .username(Username::parse("<YOUR_USERNAME_HERE>")?)
        // The raw string prefix is used because one may have a password with characters
        // Rust may try to escape or format such as `{}` or `\`.
        .password(Password::parse(r#"<YOUR_PASSWORD_HERE>"#)?)
        .build()?
        .send()
        .await?;

    println!("{:#?}", _login_res);

    // We just clear the client info and the auth token and we're good to go.
    client.clear_client_info().await?;
    client.clear_auth_tokens().await?;

    // The login and logout succeeded if the program doesn't exit prematurely.
    Ok(())
}
