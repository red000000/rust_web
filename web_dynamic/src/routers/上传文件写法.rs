/* pub fn init_upload_user_profile_photo_router(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let cors = warp::cors().allow_any_origin().allow_headers(vec!["*"]);
    warp::path("api")
        .and(warp::path("upload_user_profile_photo"))
        .and(warp::path::end())
        .and(warp::post())
        .and(warp::multipart::form())
        .then(handle_form)
        .with(cors)
}

async fn handle_form(
    mut form: warp::multipart::FormData,
) -> impl warp::Reply{
    use futures::TryStreamExt;
    use tokio::io::AsyncWriteExt;
    // 迭代所有的字段
    while let Some(mut part) = form.try_next().await.unwrap() {
        // 检查字段是否是文件字段
        if let Some(filename) = part.filename() {
            // 提取文件名并保存文件到磁盘
            let filepath = format!("写入测试/{}", filename);
            let mut file = tokio::fs::File::create(filepath).await.unwrap();
            while let Some(chunk) = part.data().await {
                file.write_all_buf(&mut chunk.unwrap()).await.unwrap();
            }
        }
    }
    // 返回成功响应
    warp::reply()
} */

/* pub fn init_upload_user_profile_photo_router(
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
} */
