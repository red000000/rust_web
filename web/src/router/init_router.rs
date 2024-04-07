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
            //检查并返回数据
            sign_up_info.check_and_return_info()
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
            //获取前端发送数据
            let sign_in_info: SignInInfo = serde_json::from_value(body).unwrap();
            //检查并返回数据
            sign_in_info.check_and_return_info()
        })
        .with(cors)
}

