pub struct SignInInfo {
    username: String,
    password: String,
    sex: String,
    birthday: String,
    country: String,
    province: String,
    city: String,
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
    //储存进数据库
    pub fn save_in_database(&self){
        
    }
    //储存检查
    fn save_in_database_check(&self)->bool{
        false
    }
}
