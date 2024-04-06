use std::env;
use warp::Filter;
use web::router::*;
#[tokio::main]
async fn main() {
    //log初始化
    env::set_var("RUST_LOG", "info");
    pretty_env_logger::try_init().expect("Failed to init logger");
    let log = warp::log("RUST_LOG");
    //初始化证书和私钥
    //let (cert_path, key_path) = init_cert_key();
    let sign_up_router = init_sign_up_router();
    let sign_in_router = init_sign_in_router();
    let get_user_info_router = init_get_user_info_router();
    let routers = sign_up_router
        .with(log)
        .or(sign_in_router.with(log))
        .or(get_user_info_router.with(log));

    warp::serve(routers)
        //.tls()
        //.cert_path(cert_path)
        //.key_path(key_path)
        .run(([127, 0, 0, 1], 8080))
        .await;
}
#[test]
fn p_test() {
    /*  use postgres::Client;
    use postgres::NoTls;
    use web::data_struct::*;
    use web::database_statement::*;
         client
    .execute(
        INSERT_ONE_USER,
        &[
            &"username",
            &"password",
            &"sex",
            &"birthday",
            &"country",
            &"province",
            &"city",
            &"profile_photo",
        ],
    )
    .unwrap(); */
    /*     client
    .execute(
        UPDATE_USER_INFO_BY_USERNAME,
        &[
            &"username",
            &"username2",
            &"password2",
            &"sex2",
            &"birthday2",
            &"country2",
            &"province2",
            &"city2",
            &"profile_photo2",
        ],
    )
    .unwrap(); */
    /*     let s_info = SearchUserInfo::new("username2".to_string());
    println!("{}", s_info.check_database_by_username()) */
    /*     let s_u_info = SignUpInfo::new("username".to_string(),"password2".to_string());
    println!("{}",s_u_info.check_user_username_and_password()); */
}
