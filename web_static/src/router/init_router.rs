use warp::Filter;

use super::router_statement::*;
pub fn _init_cert_key() -> (String, String) {
    let cert_path = "ssl/san_domain_com.crt";
    let key_path = "ssl/san_domain_com.key";
    (cert_path.to_string(), key_path.to_string())
}
//注册和登录界面路由
pub fn init_sign_in_and_up_router(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let cors = warp::cors().allow_any_origin();
    warp::path("sign_in_and_up")
        .and(warp::path::end())
        .and(warp::get())
        .then(|| async {
            let sign_in_and_up_html = tokio::fs::read_to_string(SIGN_IN_AND_UP_PAGE_LOCAL_PATH)
                .await
                .unwrap();
            warp::reply::html(sign_in_and_up_html)
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
        .then(|| async {
            let sign_in_and_up_html = tokio::fs::read_to_string(INDEX_PAGE_LOCAL_PATH)
                .await
                .unwrap();
            warp::reply::html(sign_in_and_up_html)
        })
        .with(cors)
}
pub fn init_user_info_router(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let cors = warp::cors().allow_any_origin();
    warp::path("user_info")
        .and(warp::path::end())
        .and(warp::get())
        .then(|| async {
            let user_info_html = tokio::fs::read_to_string(USER_INFO_PAGE_LOCAL_PATH)
                .await
                .unwrap();
            warp::reply::html(user_info_html)
        })
        .with(cors)
}
pub fn init_upload_user_profile_photo_router(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let cors = warp::cors().allow_any_origin();
    warp::path("upload_user_profile_photo")
        .and(warp::path::end())
        .and(warp::get())
        .then(|| async { warp::reply() })
        .with(cors)
}
