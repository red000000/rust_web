use crate::data_struct::*;
use std::vec;
use warp::Filter;

pub fn init_api_sign_up_router(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    //cors
    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["Content-Type"])
        .allow_method("POST")
        .max_age(100); //100秒过期时间

    warp::path("api")
        .and(warp::path("sign_up"))
        .and(warp::path::end())
        .and(warp::post())
        .and(warp::body::json())
        .then(|body: serde_json::Value| async {
            //获取前端发送数据
            let sign_up_info: SignUpInfo = serde_json::from_value(body).unwrap();
            //检查并返回数据
            sign_up_info.check_and_return_info().await
        })
        .with(cors)
}
pub fn init_api_sign_in_router(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    //cors需要修改，等待数据类型确定
    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["Content-Type"])
        .allow_method("POST");
    warp::path("api")
        .and(warp::path("sign_in"))
        .and(warp::path::end())
        .and(warp::post())
        .and(warp::body::json())
        .then(|body: serde_json::Value| async {
            //获取前端发送数据
            let sign_in_info: SignInInfo = serde_json::from_value(body).unwrap();
            //检查并返回数据
            sign_in_info.check_and_return_info().await
        })
        .with(cors)
}

pub fn init_get_user_info_router(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let cors = warp::cors().allow_any_origin();
    warp::path("api")
        .and(warp::path("get_user_info"))
        .and(warp::path::end())
        .and(warp::get())
        .and(warp::body::json())
        .then(|body: serde_json::Value| async {
            let search_user_info: SearchUserInfo = serde_json::from_value(body).unwrap();
            search_user_info.check_info().await
        })
        .with(cors)
}

pub fn init_upload_user_profile_photo_router(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    use futures::TryStreamExt;
    use tokio::io::AsyncWriteExt;

    let cors = warp::cors().allow_any_origin().allow_headers(vec!["*"]);
    warp::path("api")
        .and(warp::path("upload_user_profile_photo"))
        .and(warp::path::end())
        .and(warp::post())
        .and(warp::body::json())
        .and(warp::multipart::form())
        .then(
            |body: serde_json::Value, form: warp::multipart::FormData| async {
                let mut form = form;
                let mut filenames = Vec::new();
                let mut filepath = String::new();
                let upload_user_profile_photo: UploadUserProfilePhoto =
                    serde_json::from_value(body).unwrap();
                while let Some(mut part) = form.try_next().await.unwrap() {
                    if let Some(filename) = part.filename() {
                        filenames.push(filename.to_string());
                        // 提取文件名并保存文件到磁盘
                        filepath = format!("写入测试/{}", filename);
                        let mut file = tokio::fs::File::create(&filepath).await.unwrap();
                        while let Some(chunk) = part.data().await {
                            file.write_all_buf(&mut chunk.unwrap()).await.unwrap();
                        }
                    }
                }
                upload_user_profile_photo.check_and_return_info(filenames, filepath)
            },
        )
        .with(cors)
}

pub fn _video_test() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let cors = warp::cors().allow_any_origin();
    warp::path("mp4")
        .and(warp::path::end())
        .and(warp::get())
        .and(warp::fs::file("video.mp4"))
        .map(|reply: warp::filters::fs::File| {
            warp::reply::with_header(reply, "Content-Type", "video/mp4")
        })
        .with(cors)
}
