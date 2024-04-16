use crate::upload_user_profile_photo::*;
use bytes::Buf;
use futures::TryFutureExt;
use warp::Filter;
pub fn _init_cert_key() -> (String, String) {
    let cert_path = "ssl/san_domain_com.crt";
    let key_path = "ssl/san_domain_com.key";
    (cert_path.to_string(), key_path.to_string())
}
//登录界面路由
pub fn init_sign_up_router(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let cors = warp::cors().allow_any_origin();
    warp::path("sign_up")
        .and(warp::path::end())
        .and(warp::get())
        .then(|| async { warp::reply() })
        .with(cors)
}
//注册界面路由
pub fn init_sign_in_router(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let cors = warp::cors().allow_any_origin();
    warp::path("sign_in")
        .and(warp::path::end())
        .and(warp::get())
        .map(|| {
            let sign_in_html = std::fs::read_to_string(
                "D:/project/rust_vscode/rust_web/face/luntan_files/html/登入注册.html",
            )
            .unwrap();
            warp::reply::html(sign_in_html)
        })
        .with(cors)
}
//上传页面路由
pub fn init_upload_user_profile_photo_router(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    use futures::TryStreamExt;
    use tokio::io::AsyncWriteExt;

    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("POST")
        .allow_headers(vec!["*"]);

    warp::path("upload_user_profile_photo")
        .and(warp::path::end())
        .and(warp::post())
        .and(warp::multipart::form())
        .then(|form: warp::multipart::FormData| async {
            let mut form = form;
            let mut filenames = Vec::new();
            //缓冲区，表单完整
            let mut json_vec = Vec::new();
            let mut jpg_vec = Vec::new();
            let filepath = "D:/project/rust_vscode/rust_web/web/写入测试".to_string();
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
                            if let Some(filename) = part.filename() {
                                filenames.push(filename.to_string());
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
                    }
                } else {
                    break;
                }
            }

            //先这样
            let filepath_and_file = format!("{}/{}",
                filepath,
                filenames[0]
            );

            let upload_user_profile_photo_info: UploadUserProfilePhotoInfo =
                serde_json::from_slice(&json_vec).unwrap();

            if let Err(e) = tokio::fs::write(&filepath_and_file, jpg_vec).await {
                eprintln!("写入文件失败{}", e);
            }

            let text = reqwest::Client::builder()
                .no_proxy()
                .build()
                .unwrap()
                .post("http://127.0.0.1:8081/api/upload_user_profile_photo_url")
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
//主页页面路由
pub fn init_index_router(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let cors = warp::cors().allow_any_origin();
    warp::path("index")
        .and(warp::path::end())
        .and(warp::get())
        .then(|| async { warp::reply() })
        .with(cors)
}
