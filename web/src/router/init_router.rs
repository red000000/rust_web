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
            let sign_in_html = std::fs::read_to_string("D:/project/rust_vscode/rust_web/face/luntan_files/html/登入注册.html")
            .unwrap();
            warp::reply::html(sign_in_html)
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
