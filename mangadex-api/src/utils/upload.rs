use uuid::Uuid;

use crate::MangaDexClient;

/// An Enum for handling [`check_session`] errors
#[derive(Debug, thiserror::Error)]
pub enum CheckSessionError {
    #[error("An upload session {0} already exists")]
    AlreadyExists(Uuid),
    #[error(transparent)]
    MangadexApiError(#[from] crate::error::Error),
}

/// Check if an upload session doesn't exist
/// If you want to automaticly abandon the existing session,
/// Please use [`check_and_abandon_session_if_exists`] instead.
pub async fn check_session(client: &MangaDexClient) -> Result<(), CheckSessionError> {
    match client.upload().get().send().await {
        Ok(i) => Err(CheckSessionError::AlreadyExists(i.body.data.id)),
        Err(e) => {
            if let crate::error::Error::Api(error) = &e {
                if error.errors.iter().any(|er| er.status == 404) {
                    return Ok(());
                }
            }
            Err(CheckSessionError::MangadexApiError(e))
        }
    }
}

/// Check if an upload session doesn't exist and if exists automaticly abandon it
/// If you don't want to automaticly abandon the existing session,
/// Please use [`check_session`] instead.
pub async fn check_and_abandon_session_if_exists(
    client: &MangaDexClient,
) -> Result<(), crate::error::Error> {
    if let Err(e) = check_session(client).await {
        match e {
            CheckSessionError::AlreadyExists(id) => abandon_session(id, client).await?,
            CheckSessionError::MangadexApiError(error) => return Err(error),
        };
    }
    Ok(())
}

/// A abadon a session
/// it calls `DELETE /upload/{upload_session_id}`
pub async fn abandon_session(
    session: Uuid,
    client: &MangaDexClient,
) -> Result<(), crate::error::Error> {
    client
        .upload()
        .upload_session_id(session)
        .delete()
        .send()
        .await?;
    Ok(())
}
