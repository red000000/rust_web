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
    //username设为主键,唯一,从数据库获取用户信息
    pub fn from_database(username: String) -> Self {
        let row = postgres::Client::connect(DATABASE_CONNECT_BY_EASY_CONFIG, postgres::NoTls)
            .unwrap()
            .query_one(SELECT_USER_INFO_BY_USERNAME, &[&username])
            .unwrap();

        UserInfo {
            username: row.get("username"),
            password: row.get("password"),
            sex: row.get("sex"),
            birthday: row.get("birthday"),
            country: row.get("country"),
            province: row.get("province"),
            city: row.get("city"),
            profile_photo: row.get("profile_photo"),
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
    fn check_database_by_username(&self) -> bool {
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
    pub fn check_info(&self) -> warp::reply::Json {
        //检查是否能在数据库中找到用户并返回数据
        if self.check_database_by_username() {
            let user_info = UserInfo::from_database(self.get_username());
            warp::reply::json(&user_info)
        } else {
            let search_user_info_message =
                SearchUserInfoMessage::new("用户不存在".to_string(), false);
            warp::reply::json(&search_user_info_message)
        }
    }
}
impl SearchUserInfoMessage {
    pub fn new(message: String, flag: bool) -> Self {
        SearchUserInfoMessage { message, flag }
    }
}
