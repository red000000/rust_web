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
