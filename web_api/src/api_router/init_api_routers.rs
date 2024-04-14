use crate::{data_struct::*, database_struct::DbPool};
use std::{sync::Arc, vec};
use warp::Filter;

pub fn init_api_sign_up_router(
    pool: Arc<DbPool>,
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
        .then(move |body: serde_json::Value| {
            let pool = Arc::clone(&pool);
            async move {
                //获取前端发送数据
                let sign_up_info: SignUpInfo = serde_json::from_value(body).unwrap();
                let sign_up = SignUp::new(sign_up_info, pool); //创建 SignUp 结构体实例
                                                               //检查并返回数据
                sign_up.check_and_return_info().await
            }
        })
        .with(cors)
}
pub fn init_api_sign_in_router(
    pool: Arc<DbPool>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let cors = warp::cors() // 配置 CORS
        .allow_any_origin()
        .allow_headers(vec!["Content-Type"])
        .allow_methods(vec!["POST"]);

    warp::path("api")
        .and(warp::path("sign_in"))
        .and(warp::path::end())
        .and(warp::post())
        .and(warp::body::json())
        .then(move |body: serde_json::Value| {
            let pool = Arc::clone(&pool);
            // 使用 move 来确保 pool 被捕获
            // 再次克隆 pool，确保每个请求都有自己的 pool 引用
            async move {
                let sign_in_info: SignInInfo = serde_json::from_value(body).unwrap(); // 解析登录信息
                let sign_in = SignIn::new(sign_in_info, pool); // 创建 SignIn 结构体实例
                sign_in.check_and_return_info().await // 检查登录信息并返回
            }
        })
        .with(cors)
}

pub fn init_get_user_info_router(
    pool: Arc<DbPool>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let cors = warp::cors().allow_any_origin();
    warp::path("api")
        .and(warp::path("get_user_info"))
        .and(warp::path::end())
        .and(warp::get())
        .and(warp::body::json())
        .then(move |body: serde_json::Value| {
            let pool = Arc::clone(&pool);
            async move {
                let search_user_info: SearchUserInfo = serde_json::from_value(body).unwrap();
                let search_user = SearchUser::new(search_user_info, pool);
                search_user.check_info_and_return().await
            }
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
