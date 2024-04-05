mod data_struct;

use std::env;
use warp::Filter;
use data_struct::sign_up::*;

#[tokio::main]
async fn main() {
    //log初始化
    env::set_var("RUST_LOG", "info");
    pretty_env_logger::try_init().expect("Failed to init logger");
    let log = warp::log("RUST_LOG");
    //初始化证书和私钥
    let (cert_path, key_path) = init_cert_key();
    let sign_up_router=init_sign_up_router();

    let routers=sign_up_router.with(log);
    warp::serve(routers)
        .tls()
        .cert_path(cert_path)
        .key_path(key_path)
        .run(([127, 0, 0, 1], 8080))
        .await;
}
fn init_cert_key() -> (String, String) {
    let cert_path = "ssl/san_domain_com.crt";
    let key_path = "ssl/san_domain_com.key";
    (cert_path.to_string(), key_path.to_string())
}

fn init_sign_up_router() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    //cors需要修改，等待数据类型确定
    let cors=warp::cors().allow_any_origin();
    warp::path("sign_up").and(warp::post()).and(warp::body::json()).map(|body:serde_json::Value| {
        //对body一系列处理
        let sign_up_info:SignUpInfo=serde_json::from_value(body).unwrap();
        let info=sign_up_info.get_info();
        print!("{}{}",info.0,info.1);
        if sign_up_info.check_info(){
            "ok"
        }else{
            "错误"
        }
    }).with(cors)
}
fn init_sign_in_router() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    //cors需要修改，等待数据类型确定
    let cors=warp::cors().allow_any_origin();
    warp::path("sign_in").and(warp::post()).map(|| {
        //返回一个什么b东西
        warp::reply::reply()
    }).with(cors)
}