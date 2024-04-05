use serde::{Deserialize, Serialize};

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
pub struct SignInOk {
    message: String,
}
#[derive(Serialize, Deserialize)]
pub struct SignInFail {
    message: String,
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
    //输入值检查
    fn check_input(&self) -> bool {
        false
    }
    //储存检查
    fn check_save(&self) -> bool {
        false
    }
    //总检查
    pub fn check_info(&self) -> bool {
        self.check_input() && self.check_save()
    }
    //储存进数据库
    pub fn save_in_database(&self) -> bool {
        false
    }
}
impl SignInOk {
    pub fn new(message: String) -> Self {
        SignInOk { message: message }
    }
}
impl SignInFail {
    pub fn new(message: String) -> Self {
        SignInFail { message: message }
    }
}
