use crate::database_statement::*;
use serde::{Deserialize, Serialize};
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

impl UserInfo {
    //username设为主键,唯一
    pub fn from_database(username: String) -> Self {
        //查询数据库获取,先这样，等待改动，2024.4.5
        UserInfo {
            username: username,
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
impl SearchUserInfo {
    //构建函数用于测试，实际是直接接收前端数据
    pub fn new(username: String) -> Self {
        SearchUserInfo { username }
    }
    pub fn get_username(&self) -> String {
        self.username.clone()
    }
    //在数据库检查是否能找到用户,此处在大量数据时可能会有性能问题，后面考虑在路由中传入postgres::Client类型，一直保持与数据库的连接
    pub fn check_database_by_username(&self) -> bool {
        let row = postgres::Client::connect(DATABASE_CONNECT_BY_EASY_CONFIG, postgres::NoTls)
            .unwrap()
            .query(SELECT_USER_USERNAME_BY_USERNAME, &[&self.get_username()])
            .unwrap();

        if row.len() == 0 {
            false
        } else {
            true
        }
    }
}
impl SearchUserInfoMessage {
    pub fn new(message: String, flag: bool) -> Self {
        SearchUserInfoMessage { message, flag }
    }
}
