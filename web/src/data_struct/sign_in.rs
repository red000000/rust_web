use serde::{Deserialize, Serialize};
use super::sign_in_statement::*;
use crate::database_statement::*;

#[derive(Serialize, Deserialize)]
pub struct SignInInfo {
    username: String,
    password: String,
    sex: String,
    birthday: String,
    country: String,
    province: String,
    city: String,
}

#[derive(Serialize, Deserialize)]
pub struct SignInMessage {
    message: String,
    message_type: u32,
    flag: bool,
}

impl SignInInfo {
    pub fn get_info(&self) -> (String, String, String, String, String, String, String) {
        (
            self.username.clone(),
            self.password.clone(),
            self.sex.clone(),
            self.birthday.clone(),
            self.country.clone(),
            self.province.clone(),
            self.city.clone(),
        )
    }
    //输入值检查,数字，英文字母大小写，下划线组成
    fn check_input(&self) -> bool {
        //检查用户名和密码是否符合要求
        for c in self.username.chars() {
            if !c.is_ascii_alphanumeric() && c != '_' {
                return false;
            }
        }
        for c in self.password.chars() {
            if !c.is_ascii_alphanumeric() && c != '_' {
                return false;
            }
        }
        true
    }
    //储存检查，检查是否有同名
    fn check_save(&self) -> bool {
        let vec_row = postgres::Client::connect(DATABASE_CONNECT_BY_EASY_CONFIG, postgres::NoTls)
            .unwrap()
            .query(SELECT_USER_USERNAME_BY_USERNAME, &[&self.username])
            .unwrap();
        if vec_row.len() == 0 {
            true
        } else {
            false
        }
    }
    //总检查
    pub fn check_info(&self) -> warp::reply::Json {
        if !self.check_input() {
            let fail=SignInMessage::new("登录失败，用户名或密码不符合要求".to_string(),SIGN_IN_FAILED_BY_INPUT, false);
            warp::reply::json(&fail)
        }else if !self.check_save(){
            let fail=SignInMessage::new("登录失败，用户名已存在".to_string(),SIGN_IN_FAILED_BY_SAVE_DATABASE, false);
            warp::reply::json(&fail)
        }else{
            let success=SignInMessage::new("登录成功".to_string(),SIGN_IN_SUCCESS, true);
            warp::reply::json(&success)
        }
    }
    //储存进数据库
    pub fn save_in_database(&self) -> bool {
        postgres::Client::connect(DATABASE_CONNECT_BY_EASY_CONFIG, postgres::NoTls)
            .unwrap()
            .execute(
                INSERT_ONE_USER_INFO,
                &[
                    &self.username,
                    &self.password,
                    &self.sex,
                    &self.birthday,
                    &self.country,
                    &self.province,
                    &self.city,
                    &"",//用户头像url之后根据用户上传再获取
                ],
            )
            .is_ok()
    }
}
impl SignInMessage {
    pub fn new(message: String, message_type: u32, flag: bool) -> Self {
        SignInMessage {
            message,
            message_type,
            flag,
        }
    }
}
