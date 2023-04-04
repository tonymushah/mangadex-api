//! This will log in with the credentials provided upon compilation.
//!
//! You must edit the [`login.rs`](login.rs) file with your credentials in the login builder pattern to use this.
//!
//! # Usage
//!
//! ```
//! login
//! ```

use mangadex_api::types::{Password, Username};
use mangadex_api::v5::MangaDexClient;

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        use std::process;
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

async fn run() -> anyhow::Result<()> {
    // the old login system still work on api.mangadex.dev for now
    let client = MangaDexClient::api_dev_client();

    let _login_res = client
        .auth()
        .login()
        // You may also use an email address by replacing `username()` with `email()`.
        .username(Username::parse("<YOUR_USERNAME_HERE>")?)
        // The raw string prefix is used because one may have a password with characters
        // Rust may try to escape or format such as `{}` or `\`.
        .password(Password::parse(r#"<YOUR_PASSWORD_HERE>"#)?)
        .build()?
        .send()
        .await?;

    let _logout_res = client.auth().logout().build()?.send().await;

    // The login and logout succeeded if the program doesn't exit prematurely.
    Ok(())
}
