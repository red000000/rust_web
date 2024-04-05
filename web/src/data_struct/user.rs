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
    profile_photo:String
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
            profile_photo:String::new(),
        }
    }
}
impl SearchUserInfo {
    pub fn get_username(&self) -> String {
        self.username.clone()
    }
    //在数据库检查是否能找到用户
    pub fn check_database(&self) -> bool {
        false
    }
}
impl SearchUserInfoMessage {
    pub fn new(message: String, flag: bool) -> Self {
        SearchUserInfoMessage { message, flag }
    }
}
