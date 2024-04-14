use std::sync::Arc;

use crate::database_struct::*;
use serde::{Deserialize, Serialize};

pub struct User {
    user_info: UserInfo,
    pool: Arc<DbPool>,
}

pub struct SearchUser {
    search_user_info: SearchUserInfo,
    pool: Arc<DbPool>,
}

#[derive(Serialize, Deserialize)]
pub struct UserInfo {
    username: String,
    password: String,
    sex: String,
    birthday: String,
    country: String,
    province: String,
    city: String,
    profile_photo: String, //url
}
#[derive(Serialize, Deserialize)]
pub struct SearchUserInfo {
    username: String,
}
#[derive(Serialize, Deserialize)]
pub struct SearchUserInfoMessage {
    message: String,
    flag: bool,
}
impl User {
    pub fn new(user_info: UserInfo, pool: Arc<DbPool>) -> Self {
        User { user_info, pool }
    }
    //username设为主键,唯一,从数据库获取用户信息
    pub async fn from_database(&mut self, username: String) -> Result<&mut Self, ()> {
        if let Ok(row) = self
            .pool
            .query_one(SELECT_USER_INFO_BY_USERNAME, &[&username])
            .await
        {
            self.user_info.username = row.get("username");
            self.user_info.password = row.get("password");
            self.user_info.sex = row.get("sex");
            self.user_info.birthday = row.get("birthday");
            self.user_info.country = row.get("country");
            self.user_info.province = row.get("province");
            self.user_info.city = row.get("city");
            self.user_info.profile_photo = row.get("profile_photo");
            Ok(self)
        } else {
            Err(())
        }
    }
}
impl UserInfo {
    pub fn new() -> Self {
        UserInfo {
            username: String::new(),
            password: String::new(),
            sex: String::new(),
            birthday: String::new(),
            country: String::new(),
            province: String::new(),
            city: String::new(),
            profile_photo: String::new(),
        }
    }
}
impl SearchUser {
    pub fn new(search_user_info: SearchUserInfo, pool: Arc<DbPool>) -> Self {
        SearchUser {
            search_user_info,
            pool,
        }
    }
    //在数据库检查是否能找到用户
    async fn check_user_in_database_by_username(&self) -> bool {
        if let Ok(_) = self
            .pool
            .query_one(
                SELECT_USER_USERNAME_BY_USERNAME,
                &[&self.search_user_info.get_username()],
            )
            .await
        {
            true
        } else {
            false
        }
    }
    pub async fn check_info_and_return(&self) -> warp::reply::Json {
        //检查是否能在数据库中找到用户并返回数据,如果没有返回空userinfo
        if self.check_user_in_database_by_username().await {
            let mut user = User::new(UserInfo::new(), self.pool.clone());
            if let Ok(_) = user
                .from_database(self.search_user_info.username.clone())
                .await
            {
                warp::reply::json(&user.user_info)
            } else {
                let user_info = UserInfo::new();
                warp::reply::json(&user_info)
            }
        } else {
            let user_info = UserInfo::new();
            warp::reply::json(&user_info)
        }
    }
}
impl SearchUserInfo {
    //构建函数用于测试，实际是直接接收前端数据
    pub fn new() -> Self {
        SearchUserInfo {
            username: String::new(),
        }
    }
    pub fn get_username(&self) -> String {
        self.username.clone()
    }
    pub fn username(&mut self, username: String) -> &mut Self {
        self.username = username;
        self
    }
}
impl SearchUserInfoMessage {
    pub fn new() -> Self {
        SearchUserInfoMessage { message: String::new(), flag: false}
    }
    pub fn message(&mut self, message: String) -> &mut Self {
        self.message = message;
        self
    }
    pub fn flag(&mut self, flag: bool) -> &mut Self {
        self.flag = flag;
        self
    }
}
