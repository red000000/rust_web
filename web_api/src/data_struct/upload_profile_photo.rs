use super::upload_profile_photo_statement::*;
#[derive(serde::Deserialize,serde::Serialize)]
pub struct UploadUserProfilePhoto{
    username:String,
    profile_photo_url:String,
}
impl UploadUserProfilePhoto {
    //检查下载文件是否成功并返回数据
    pub fn check_and_return_info(&self,filenames:Vec<String>,profile_photo_url:String)->warp::reply::Json{
        let mut flag = false;
            let folder_path = profile_photo_url;
            if let Ok(entries) = std::fs::read_dir(folder_path) {
                let mut found_files = vec![false; filenames.len()];
                for entry in entries {
                    if let Ok(entry) = entry {
                        if let Some(file_name) = entry.file_name().to_str() {
                            // 检查文件名是否在列表中，并更新标记
                            if let Some(index) = filenames.iter().position(|name| name == file_name)
                            {
                                found_files[index] = true;
                            }
                        }
                    }
                }
                flag = found_files.iter().all(|&found| found);
            }
            if flag {
                let success = UploadUserProfilePhotoMessage::new(
                    "上传成功".to_string(),
                    UPLOAD_PROFILE_PHOTO_SUCCESS,
                    true,
                );
                warp::reply::json(&success)
            } else {
                let fail = UploadUserProfilePhotoMessage::new(
                    "上传失败".to_string(),
                    UPLOAD_PROFILE_PHOTO_FAILED,
                    false,
                );
                warp::reply::json(&fail)
            }
    }
}
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