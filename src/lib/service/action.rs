use crate::data::{query, DatabasePool};
use crate::service::ask;
use crate::Clip;
use std::convert::TryInto;

use super::ServiceError;

pub async fn get_clip(req: ask::GetClip, pool: &DatabasePool) -> Result<Clip, ServiceError> {
    let user_password = req.password.clone();
    let clip: Clip = query::get_clip(req, pool).await?.try_into()?;

    if clip.password.has_password() {
        if clip.password == user_password {
            Ok(clip)
        } else {
            Err(ServiceError::PermissionError("Invalid password".to_owned()))
        }
    } else {
        Ok(clip)
    }
}

pub async fn new_clip(req: ask::NewClip, pool: &DatabasePool) -> Result<Clip, ServiceError> {
    // .await - wait for the db query to finish
    // try_into - try to convert from the data::Clip into the domain::Clip
    Ok(query::new_clip(req, pool).await?.try_into()?)
}

pub async fn update_clip(req: ask::UpdateClip, pool: &DatabasePool) -> Result<Clip, ServiceError> {
    // .await - wait for the db query to finish
    // try_into - try to convert from the data::Clip into the domain::Clip
    Ok(query::update_clip(req, pool).await?.try_into()?)
}
