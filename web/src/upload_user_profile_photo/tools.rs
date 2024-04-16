use super::upload_user_profile_photo::*;
pub async fn get_upload_user_profile_photo_info_json(part: &mut warp::multipart::Part)->Result<UploadUserProfilePhotoInfo,()>{
    use bytes::Buf;

    if let Some(chunk) = part.data().await {
        if let Ok(json_data) = chunk {
            let upload_user_profile_photo_info: UploadUserProfilePhotoInfo =
                serde_json::from_slice(&json_data.chunk()).unwrap();
            reqwest::Client::builder()
                .no_proxy()
                .build()
                .unwrap()
                .post("http://127.0.0.1:8081/api/upload_user_profile_photo_url")
                .json(&upload_user_profile_photo_info)
                .send()
                .await
                .unwrap();
            Ok(upload_user_profile_photo_info)
        }else {
            Err(())
        }
    }else {
        Err(())
    }
}