use serde::{Serialize,Deserialize};
#[derive(Serialize, Deserialize)]
pub struct SignUpInfo {
    username: String,
    password: String,
}
struct SignUpOk {
    message: String,
}
struct SignUpFail {
    message: String,
}

impl SignUpInfo {
    //获取用户名和密码
    pub fn get_info(&self) -> (String, String) {
        (self.username.clone(), self.password.clone())
    }
    //之后连接数据库查询
    pub fn check_info(&self) -> bool {
        if self.username == "user".to_string() && self.password == "pass".to_string() {
            true
        } else {
            false
        }
    }
}
impl SignUpOk {
    pub fn new(message: String) -> Self {
        SignUpOk {
            message: message,
        }
    }
}
impl SignUpFail {
    pub fn new(message: String) -> Self {
        SignUpFail {
            message: message,
        }
    }
}
