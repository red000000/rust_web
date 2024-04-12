use crate::database_statement::*;
use serde::{Deserialize, Serialize};

use super::{SIGN_UP_FAILED, SIGN_UP_SUCCESS};
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

impl SignUpInfo {
    //此处构造函数用于测试，实际接收前端传来的数据
    pub fn new(username: String, password: String) -> Self {
        SignUpInfo { username, password }
    }
    //获取用户名和密码
    pub fn get_info(&self) -> (String, String) {
        (self.username.clone(), self.password.clone())
    }
    //之后连接数据库查询
    async fn check_user_username_and_password(&self) -> bool {
        let user_info = self.get_info();
        let (client, connection) =
            tokio_postgres::connect(DATABASE_CONNECT_BY_EASY_CONFIG, tokio_postgres::NoTls)
                .await
                .unwrap();

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("连接错误: {}", e);
            }
        });

        let rows = client
            .query(
                SELECT_USER_USERNAME_AND_PASSWORD_BY_USERNAME,
                &[&self.get_info().0.as_str()],
            )
            .await
            .unwrap();
        //如果查询结果为空，则返回false
        if rows.len() == 0 {
            false
        } else {
            //此处如果查询结果为空，则迭代器无法实例化，会报错，所以需要判断一下
            let row = rows.iter().next().unwrap();
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
        }
    }
    //总检查并向前端返回数据
    pub async fn check_and_return_info(&self) -> warp::reply::Json {
        if self.check_user_username_and_password().await {
            let success = SignUpMessage::new("登录成功".to_string(), SIGN_UP_SUCCESS, true);
            warp::reply::json(&success)
        } else {
            let fail = SignUpMessage::new("登录失败".to_string(), SIGN_UP_FAILED, false);
            warp::reply::json(&fail)
        }
    }
}
impl SignUpMessage {
    pub fn new(message: String, message_type: u32, flag: bool) -> Self {
        SignUpMessage {
            message,
            message_type,
            flag,
        }
    }
}
