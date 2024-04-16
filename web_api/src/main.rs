use std::sync::Arc;
use warp::Filter;
use web_api::{api_router::*, database_struct::*};
#[derive(serde::Deserialize, serde::Serialize)]
struct ProvinceList {
    message: String,
    list: Vec<String>,
}
#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "info");
    pretty_env_logger::try_init_timed().unwrap();
    let log = warp::log("WEB_API_LOG");

    let mut config = tokio_postgres::Config::new();
    config
        .user("postgres")
        .password("password")
        .dbname("postgres")
        .hostaddr(std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)))
        .port(5432);

    let db_pool = Arc::new(DbPool::new(config, 3, 10).await);

    let get_user_info_router = init_get_user_info_router(Arc::clone(&db_pool));
    let api_sign_in_router = init_api_sign_in_router(Arc::clone(&db_pool));
    let api_sign_up_router = init_api_sign_up_router(Arc::clone(&db_pool));
    let api_upload_user_profile_photo_url_router =
        init_upload_user_profile_photo_url_router(Arc::clone(&db_pool));
    let api_routers = get_user_info_router
        .with(log)
        .or(api_sign_in_router.with(log))
        .or(api_sign_up_router.with(log))
        .or(api_upload_user_profile_photo_url_router.with(log));
    warp::serve(api_routers).run(([127, 0, 0, 1], 8081)).await;
}

//上传文件测试
#[tokio::test]
async fn test() {
    use reqwest::multipart;
    use std::fs::File;
    use std::io::Read;
    // 读取图片文件
    let mut file = File::open("test.jpg").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    // 构建包含图片文W件的 multipart 请求
    let form = multipart::Form::new()
        // 添加一个文件字段
        .part(
            "image",
            multipart::Part::bytes(buffer)
                .file_name("test.jpg")
                .mime_str("image/jpg")
                .unwrap(),
        );

    // 创建一个客户端
    let client = reqwest::Client::builder().no_proxy().build().unwrap();

    // 发送 POST 请求
    client
        .post("http://127.0.0.1:8081/api/upload_user_profile_photo")
        // 设置请求体为 multipart
        .multipart(form)
        // 发送请求并等待响应
        .send()
        .await
        .unwrap();
}
//sign_in结束
#[tokio::test]
async fn test3() {
    let client = reqwest::Client::builder().no_proxy().build().unwrap();
    let test = client
        .get("http://127.0.0.1:8081/api/get_user_info")
        .json(&serde_json::json!({
            "username":"username",
        }))
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    println!("{:?}", test);
    /*     let mut i=0;
     while i<100 {
        let test = client
        .post("http://127.0.0.1:8081/api/sign_in")
        .json(&serde_json::json!({
            "username":"username",
            "password":"123",
            "sex":"123",
           "birthday":"123",
            "country":"123",
            "province":"123",
            "city":"123",
        }))
        .send()
        .await
        .unwrap().text().await.unwrap();
    println!("{:?}", test);
    i=i+1;
    } */
}
