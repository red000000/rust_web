use super::upload_user_profile_photo_url_statement::*;
use crate::database_struct::{
    DbPool, SELECT_USER_USERNAME_BY_USERNAME, UPDATE_USER_PROFILE_PHOTO_URL_INFO,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
pub struct UpLoadUserProfilePhotoUrl {
    upload_user_profile_photo_info: UpLoadUserProfilePhotoUrlInfo,
    pool: Arc<DbPool>,
}
#[derive(Serialize, Deserialize)]
pub struct UpLoadUserProfilePhotoUrlInfo {
    username: String,
    profile_photo_url: String,
}
#[derive(Serialize, Deserialize)]
pub struct UpLoadUserProfilePhotoUrlMessage {
    message: String,
    message_type: u32,
    flag: bool,
}
impl UpLoadUserProfilePhotoUrl {
    pub fn new(
        upload_user_profile_photo_info: UpLoadUserProfilePhotoUrlInfo,
        pool: Arc<DbPool>,
    ) -> Self {
        UpLoadUserProfilePhotoUrl {
            upload_user_profile_photo_info,
            pool,
        }
    }
    //检查数据库是否有该用户
    pub async fn check_user_in_database_by_username(&self) -> bool {
        if let Ok(_) = self
            .pool
            .query_one(
                SELECT_USER_USERNAME_BY_USERNAME,
                &[&self.upload_user_profile_photo_info.username],
            )
            .await
        {
            true
        } else {
            false
        }
    }
    pub async fn check_and_return_info(&self) -> warp::reply::Json {
        if self.check_user_in_database_by_username().await {
            if let Ok(_) = self
                .pool
                .execute(
                    UPDATE_USER_PROFILE_PHOTO_URL_INFO,
                    &[
                        &self.upload_user_profile_photo_info.profile_photo_url,
                        &self.upload_user_profile_photo_info.username,
                    ],
                )
                .await
            {
                let mut success = UpLoadUserProfilePhotoUrlMessage::new();
                success
                    .message("更新成功".to_string())
                    .message_type(UPLOAD_USER_PROFILE_PHOTO_URL_SUCCESS)
                    .flag(true);
                warp::reply::json(&success)
            } else {
                let mut fail = UpLoadUserProfilePhotoUrlMessage::new();
                fail.message("更新数据库失败".to_string())
                    .message_type(UPLOAD_USER_PROFILE_PHOTO_URL_TO_DB_FAILED)
                    .flag(false);
                warp::reply::json(&fail)
            }
        } else {
            let mut fail = UpLoadUserProfilePhotoUrlMessage::new();
            fail.message("用户不存在".to_string())
                .message_type(UPLOAD_USER_PROFILE_PHOTO_URL_FAILED)
                .flag(false);
            warp::reply::json(&fail)
        }
    }
}
impl UpLoadUserProfilePhotoUrlInfo {
    pub fn new() -> Self {
        UpLoadUserProfilePhotoUrlInfo {
            username: "".to_string(),
            profile_photo_url: "".to_string(),
        }
    }
    pub fn username(&mut self, username: String) -> &mut Self {
        self.username = username;
        self
    }
    pub fn profile_photo_url(&mut self, profile_photo_url: String) -> &mut Self {
        self.profile_photo_url = profile_photo_url;
        self
    }
}
impl UpLoadUserProfilePhotoUrlMessage {
    pub fn new() -> Self {
        UpLoadUserProfilePhotoUrlMessage {
            message: "".to_string(),
            message_type: UPLOAD_USER_PROFILE_PHOTO_URL_DEFAULT_FAILED,
            flag: false,
        }
    }
    pub fn message(&mut self, message: String) -> &mut Self {
        self.message = message;
        self
    }
    pub fn message_type(&mut self, message_type: u32) -> &mut Self {
        self.message_type = message_type;
        self
    }
    pub fn flag(&mut self, flag: bool) -> &mut Self {
        self.flag = flag;
        self
    }
}
