use super::sign_in_statement::*;
use crate::database_struct::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub struct SignIn {
    sign_in_info: SignInInfo,
    pool: Arc<DbPool>,
}

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
impl SignIn {
    pub fn new(sign_in_info: SignInInfo, pool: Arc<DbPool>) -> Self {
        SignIn { sign_in_info, pool }
    }
    //输入值检查,数字，英文字母大小写，下划线组成
    fn check_input(&self) -> bool {
        //检查用户名和密码是否符合要求
        for c in self.sign_in_info.username.chars() {
            if !c.is_ascii_alphanumeric() && c != '_' {
                return false;
            }
        }
        for c in self.sign_in_info.password.chars() {
            if !c.is_ascii_alphanumeric() && c != '_' {
                return false;
            }
        }
        true
    }
    //储存检查，检查是否有同名
    async fn check_save(&self, pool: &DbPool) -> bool {
        if let Ok(rows) = pool
            .query(
                SELECT_USER_USERNAME_BY_USERNAME,
                &[&self.sign_in_info.username],
            )
            .await
        {
            if rows.len() == 0 {
                true
            } else {
                false
            }
        } else {
            false
        }
    }
    //总检查并且向前端返回数据
    pub async fn check_and_return_info(&self) -> warp::reply::Json {
        if !self.check_input() {
            let fail = SignInMessage::new(
                "登录失败，用户名或密码不符合要求".to_string(),
                SIGN_IN_FAILED_BY_INPUT,
                false,
            );
            warp::reply::json(&fail)
        } else if !self.check_save(&self.pool).await {
            let fail = SignInMessage::new(
                "登录失败，用户名已存在".to_string(),
                SIGN_IN_FAILED_BY_SAVE_DATABASE,
                false,
            );
            warp::reply::json(&fail)
        } else {
            let success = SignInMessage::new("注册成功".to_string(), SIGN_IN_SUCCESS, true);
            warp::reply::json(&success)
            //先不储存
        }
    }
    //储存进数据库
    pub async fn save_in_database(&self) -> bool {
        match self
            .pool
            .execute(
                INSERT_ONE_USER_INFO,
                &[
                    &self.sign_in_info.username,
                    &self.sign_in_info.password,
                    &self.sign_in_info.sex,
                    &self.sign_in_info.birthday,
                    &self.sign_in_info.country,
                    &self.sign_in_info.province,
                    &self.sign_in_info.city,
                    &"".to_string(), //用户头像url之后根据用户上传再获取
                ],
            )
            .await
        {
            Ok(_) => true,
            Err(_) => false,
        }
    }
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
}
impl SignInMessage {
    pub fn new(message: String, message_type: u32, flag: bool) -> Self {
        SignInMessage {
            message,
            message_type,
            flag,
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
