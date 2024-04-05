use serde::{Serialize,Deserialize};
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
impl SignUpMessage {
    pub fn new(message: String,flag:bool) -> Self {
        SignUpMessage {
            message,
            flag,
        }
    }
}

