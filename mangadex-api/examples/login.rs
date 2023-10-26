//! This will log in with the credentials provided upon compilation.
//!
//! Work only with `legacy-auth` and `legacy-account` feature
//!  
//! You must edit the [`login.rs`](login.rs) file with your credentials in the login builder pattern to use this.
//!
//! # Usage
//!
//! ```
//! login
//! ```

#[cfg(all(feature = "legacy-auth", feature = "legacy-account"))]
use mangadex_api::v5::MangaDexClient;
#[cfg(all(feature = "legacy-auth", feature = "legacy-account"))]
use mangadex_api_types::{Password, Username};

#[tokio::main]
async fn main() {
    #[cfg(all(feature = "legacy-auth", feature = "legacy-account"))]
    if let Err(e) = run().await {
        use std::process;
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
    #[cfg(not(all(feature = "legacy-auth", feature = "legacy-account")))]
    eprintln!("You should enable the `legacy-auth` and `legacy-account` features")
}

#[cfg(all(feature = "legacy-auth", feature = "legacy-account"))]
async fn run() -> anyhow::Result<()> {
    // the old login system still work on api.mangadex.dev for now
    let client = MangaDexClient::api_dev_client();

    let _login_res = client
        .auth()
        .login()
        .post()
        // You may also use an email address by replacing `username()` with `email()`.
        .username(Username::parse("<YOUR_USERNAME_HERE>")?)
        // The raw string prefix is used because one may have a password with characters
        // Rust may try to escape or format such as `{}` or `\`.
        .password(Password::parse(r#"<YOUR_PASSWORD_HERE>"#)?)
        .build()?
        .send()
        .await?;

    let _logout_res = client.auth().logout().post().build()?.send().await;

    // The login and logout succeeded if the program doesn't exit prematurely.
    Ok(())
}
