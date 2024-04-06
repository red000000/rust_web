use crate::data_struct::*;
use warp::Filter;

pub fn _init_cert_key() -> (String, String) {
    let cert_path = "ssl/san_domain_com.crt";
    let key_path = "ssl/san_domain_com.key";
    (cert_path.to_string(), key_path.to_string())
}

pub fn init_sign_up_router() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    //cors
    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["Content-Type"])
        .allow_method("POST")
        .max_age(100); //100秒过期时间

    warp::path("sign_up")
        .and(warp::path::end())
        .and(warp::post())
        .and(warp::body::json())
        .map(|body: serde_json::Value| {
            //获取前端发送数据
            let sign_up_info: SignUpInfo = serde_json::from_value(body).unwrap();
            //打印测试
            let info = sign_up_info.get_info();
            print!("{}{}", info.0, info.1);
            //检查并返回数据
            if sign_up_info.check_user_username_and_password() {
                let success = SignUpMessage::new("登录成功".to_string(), true);
                warp::reply::json(&success)
            } else {
                let fail = SignUpMessage::new("登录失败".to_string(), false);
                warp::reply::json(&fail)
            }
        })
        .with(cors)
}
pub fn init_sign_in_router() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    //cors需要修改，等待数据类型确定
    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["Content-Type"])
        .allow_method("POST")
        .max_age(100); //100秒过期时间

    warp::path("sign_in")
        .and(warp::path::end())
        .and(warp::post())
        .and(warp::body::json())
        .map(|body: serde_json::Value| {
            //返回一个什么b东西
            let sign_in_info: SignInInfo = serde_json::from_value(body).unwrap();
            let info = sign_in_info.get_info();
            print!("{}{}", info.0, info.1);
            if sign_in_info.check_info() && sign_in_info.save_in_database() {
                let success = SignInMessage::new("注册成功".to_string(), true);
                warp::reply::json(&success)
            } else {
                let fail = SignInMessage::new("注册失败".to_string(), false);
                warp::reply::json(&fail)
            }
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
        .map(|body: serde_json::Value| {
            let search_user_info: SearchUserInfo = serde_json::from_value(body).unwrap();
            if search_user_info.check_database_by_username() {
                let user_info = UserInfo::from_database(search_user_info.get_username());
                warp::reply::json(&user_info)
            } else {
                let search_user_info_message =
                    SearchUserInfoMessage::new("用户不存在".to_string(), false);
                warp::reply::json(&search_user_info_message)
            }
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