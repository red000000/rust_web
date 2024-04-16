use crate::database_struct::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use super::sign_up_statements::*;
pub struct SignUp {
    sign_up_info: SignUpInfo,
    pool: Arc<DbPool>,
}
#[derive(Serialize, Deserialize)]
pub struct SignUpInfo {
    username: String,
    password: String,
}
#[derive(Serialize, Deserialize)]
pub struct SignUpMessage {
    message: String,
    message_type: u32,
    flag: bool,
}
impl SignUp {
    pub fn new(sign_up_info: SignUpInfo, pool: Arc<DbPool>) -> Self {
        SignUp { sign_up_info, pool }
    }
    //之后连接数据库查询
    async fn check_user_username_and_password(&self) -> bool {
        let user_info = self.sign_up_info.get_info();

        if let Ok(rows) = self
            .pool
            .query(
                SELECT_USER_USERNAME_AND_PASSWORD_BY_USERNAME,
                &[&user_info.0.as_str()],
            )
            .await
        {
            if rows.len() == 0 {
                false
            } else {
                //此处如果查询结果为空，则迭代器无法实例化，会报错，所以需要判断一下
                if let Some(row) = rows.iter().next() {
                    let database_username: String = row.get("username");
                    let database_password: String = row.get("password");
                    //判断是否相等
                    if database_username == user_info.0.as_str()
                        && database_password == user_info.1.as_str()
                    {
                        true
                    } else {
                        false
                    }
                } else {
                    println!("rows迭代器无法实例化");
                    false
                }
            }
        } else {
            println!("查询用户信息失败");
            false
        }
    }
    //总检查并向前端返回数据
    pub async fn check_and_return_info(&self) -> warp::reply::Json {
        if self.check_user_username_and_password().await {
            let mut success = SignUpMessage::new();
            success
                .message("登录成功".to_string())
                .message_type(SIGN_UP_SUCCESS)
                .flag(true);
            warp::reply::json(&success)
        } else {
            let mut fail = SignUpMessage::new();
            fail.message("用户名或密码错误".to_string()).message_type(SIGN_UP_FAILED).flag(false);
            warp::reply::json(&fail)
        }
    }
}

impl SignUpInfo {
    //此处构造函数用于测试，实际接收前端传来的数据
    pub fn new(username: String, password: String) -> Self {
        SignUpInfo { username, password }
    }
    //获取用户名和密码
    pub fn get_info(&self) -> (String, String) {
        (self.username.clone(), self.password.clone())
    }
}
impl SignUpMessage {
    pub fn new() -> Self {
        SignUpMessage {
            message: String::new(),
            message_type: SIGN_UP_DEFAULT_FAILED,
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
