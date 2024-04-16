use crate::upload_user_profile_photo::*;
use warp::Filter;
//上传头像api
pub fn init_api_upload_user_profile_photo_router(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    use bytes::Buf;
    use futures::TryStreamExt;

    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("POST")
        .allow_headers(vec!["*"]);

    warp::path("static")
        .and(warp::path("api"))
        .and(warp::path("upload_user_profile_photo"))
        .and(warp::path::end())
        .and(warp::post())
        .and(warp::multipart::form().max_length(2 * 1024 * 1024))//默认2mb
        .then(|form: warp::multipart::FormData| async {
            let mut form = form;
            let mut filenames = Vec::new();
            //缓冲区，表单完整
            let mut json_vec = Vec::new();
            let mut jpg_vec = Vec::new();
            let filepath = "D:/project/rust_vscode/rust_web/web_static/写入测试".to_string();
            //使用try_next按顺序异步读取
            while let Ok(part_option) = form.try_next().await {
                if let Some(mut part) = part_option {
                    if let Some(content_type) = part.content_type() {
                        if content_type == "application/json" {
                            //持续获取json数据直至异步流结束
                            while let Some(chunk) = part.data().await {
                                if let Ok(json_data) = chunk {
                                    json_vec.extend_from_slice(json_data.chunk());
                                } else {
                                    break;
                                }
                            }
                        } else {
                            filenames.push(part.name().to_string());
                            //持续获取jpg数据直至异步流结束
                            while let Some(chunk) = part.data().await {
                                if let Ok(jpg) = chunk {
                                    jpg_vec.extend_from_slice(jpg.chunk())
                                } else {
                                    break;
                                }
                            }
                        }
                    }
                } else {
                    break;
                }
            }

            let upload_user_profile_photo_info: UploadUserProfilePhotoInfo =
                serde_json::from_slice(&json_vec).unwrap();

            //先这样
            let filepath_and_file = format!("{}/{}", filepath, filenames[0]);
            if let Err(e) = tokio::fs::write(&filepath_and_file, jpg_vec).await {
                eprintln!("写入文件失败{}", e);
            }

            let text = reqwest::Client::builder()
                .no_proxy()
                .build()
                .unwrap()
                .post("http://127.0.0.1:8081/dynamic/api/upload_user_profile_photo_url")
                .json(&upload_user_profile_photo_info)
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
            println!("{}", text);
            upload_user_profile_photo_info.check_and_return_info(filenames, filepath)
        })
        .with(cors)
}
