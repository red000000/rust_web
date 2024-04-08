use warp::Filter;
use web::data_struct::*;
pub fn init_get_user_info_router(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let cors = warp::cors().allow_any_origin();
    warp::path("api")
        .and(warp::path("get_user_info"))
        .and(warp::path::end())
        .and(warp::get())
        .and(warp::body::json())
        .map(|body: serde_json::Value| {
            let search_user_info: SearchUserInfo = serde_json::from_value(body).unwrap();
            search_user_info.check_info()
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
        .and(warp::multipart::form())
        .then(|form: warp::multipart::FormData| async {
            let mut form = form;
            while let Some(mut part) = form.try_next().await.unwrap() {
                if let Some(filename) = part.filename() {
                    // 提取文件名并保存文件到磁盘
                    let filepath = format!("写入测试/{}", filename);
                    let mut file = tokio::fs::File::create(filepath).await.unwrap();
                    while let Some(chunk) = part.data().await {
                        file.write_all_buf(&mut chunk.unwrap()).await.unwrap();
                    }
                }
            }
            warp::reply()
        })
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
