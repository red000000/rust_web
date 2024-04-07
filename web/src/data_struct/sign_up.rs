use crate::database_statement::*;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct SignUpInfo {
    username: String,
    password: String,
}
#[derive(Serialize, Deserialize)]
pub struct SignUpMessage {
    message: String,
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
    fn check_user_username_and_password(&self) -> bool {
        let user_info = self.get_info();
        let vec_row = postgres::Client::connect(DATABASE_CONNECT_BY_EASY_CONFIG, postgres::NoTls)
            .unwrap()
            .query(
                SELECT_USER_USERNAME_AND_PASSWORD_BY_USERNAME,
                &[&self.get_info().0.as_str()],
            )
            .unwrap();
        //如果查询结果为空，则返回false
        if vec_row.len() == 0 {
            false
        } else {
            //此处如果查询结果为空，则迭代器无法实例化，会报错，所以需要判断一下
            let row = vec_row.iter().next().unwrap();
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
    pub fn check_and_return_info(&self) -> warp::reply::Json {
        if self.check_user_username_and_password() {
            let success = SignUpMessage::new("登录成功".to_string(), true);
            warp::reply::json(&success)
        } else {
            let fail = SignUpMessage::new("登录失败".to_string(), false);
            warp::reply::json(&fail)
        }
    }
}
impl SignUpMessage {
    pub fn new(message: String, flag: bool) -> Self {
        SignUpMessage { message, flag }
    }
}
