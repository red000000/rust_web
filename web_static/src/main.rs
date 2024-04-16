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
    let upload_user_profile_photo_router = init_upload_user_profile_photo_router();

    let api_upload_user_profile_photo_router = init_api_upload_user_profile_photo_router();
    let routers = sign_up_router.with(log).or(sign_in_router
        .with(log)
        .or(upload_user_profile_photo_router.with(log))
        .or(api_upload_user_profile_photo_router.with(log)));

    warp::serve(routers)
        //.tls()
        //.cert_path(cert_path)
        //.key_path(key_path)
        .run(([127, 0, 0, 1], 8080))
        .await;
}
#[tokio::test]
async fn test() {
    use serde_json::json;
    let file_bytes = std::fs::read("1.jpg").unwrap();
    let json = json!(
        {
            "username":"username2",
            "profile_photo_url":"url",
        }
    );
    let json_bytes = serde_json::to_vec(&json).unwrap();
    let part1 = reqwest::multipart::Part::bytes(json_bytes)
        .file_name("upload_user_profile_photo.json") // 设置文件名
        .mime_str("application/json")
        .unwrap();
    let part2 = reqwest::multipart::Part::bytes(file_bytes)
        .file_name("1.jpg") // 设置文件名
        .mime_str("image/jpeg")
        .unwrap(); // 设置文件类型
    let form = reqwest::multipart::Form::new()
        .part("json_part", part1)
        .part("img_part", part2); // 将文件part添加到multipart表单中
    let text = reqwest::Client::builder()
        .no_proxy()
        .build()
        .unwrap()
        .post("http://127.0.0.1:8080/upload_user_profile_photo")
        .multipart(form)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    println!("{:?}", text);
}
