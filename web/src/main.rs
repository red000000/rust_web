use warp::Filter;
use web::router::*;
#[tokio::main]
async fn main() {
    //log初始化
    std::env::set_var("RUST_LOG", "info");
    pretty_env_logger::try_init_timed().expect("Failed to init logger");
    let log = warp::log("RUST_LOG");
    //初始化证书和私钥
    //let (cert_path, key_path) = init_cert_key();
    let sign_up_router = init_sign_up_router();
    let sign_in_router = init_sign_in_router();
    let routers = sign_up_router.with(log).or(sign_in_router.with(log));

    warp::serve(routers)
        //.tls()
        //.cert_path(cert_path)
        //.key_path(key_path)
        .run(([127, 0, 0, 1], 8080))
        .await;
}
#[test]
fn test() {
    let sign_in_html = std::fs::read_to_string("D:/project/rust_vscode/rust_web/face/luntan_files/html/登入注册.html")
                .unwrap();
            println!("{}", sign_in_html);
}
