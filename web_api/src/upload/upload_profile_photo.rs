
#[derive(serde::Deserialize,serde::Serialize)]
pub struct UploadUserProfilePhotoMessage{
    message:String,
    message_type:u32,
    flag:bool,
}
impl UploadUserProfilePhotoMessage{
    pub fn new(message:String,message_type:u32,flag:bool)->Self{
        UploadUserProfilePhotoMessage{
            message,
            message_type,
            flag,
        }
    }
}