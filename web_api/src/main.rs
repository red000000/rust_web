mod api_router;
use api_router::*;
use warp::Filter;
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
    let get_user_info_router = init_get_user_info_router();
    let api_routers = get_user_info_router.with(log);
    warp::serve(api_routers).run(([127, 0, 0, 1], 8081)).await;
}
/* #[tokio::test]
async fn test() {
    use reqwest::Client;
    let get_province = Client::new()
        .get("https://hmajax.itheima.net/api/province")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let province_list: ProvinceList = serde_json::from_str(&get_province).unwrap();
    for province in province_list.list {
        println!("{}", province);
    }
} */
