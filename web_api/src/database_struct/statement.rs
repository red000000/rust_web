//简单与数据库建立连接语句
pub const CONNECT_DATABASE_BY_EASY_CONFIG: &str =
    "user=postgres password='password' dbname=postgres hostaddr=127.0.0.1 port=5432";

//创建用户表语句
pub const CREATE_USER_TABLE: &str = "CREATE TABLE IF NOT EXISTS users (
    username VARCHAR(255) PRIMARY KEY,
    password VARCHAR(255),
    sex VARCHAR(255),
    birthday VARCHAR(255),
    country VARCHAR(255),
    province VARCHAR(255),
    city VARCHAR(255),
    profile_photo VARCHAR(255)
)";

//插入单个用户信息语句
pub const INSERT_ONE_USER_INFO: &str = "INSERT INTO users 
(username, password, sex, birthday, country, province, city, profile_photo) 
VALUES 
($1, $2, $3, $4, $5, $6, $7, $8)";

//根据用户名更新用户信息语句
pub const UPDATE_USER_INFO_BY_USERNAME: &str = "UPDATE users
SET username = $2, password = $3, sex = $4, birthday = $5, country = $6, province = $7, city = $8, profile_photo = $9
WHERE username = $1";

//根据用户名删除用户信息语句
pub const DELETE_USER_INFO_BY_USERNAME: &str = "DELETE FROM users WHERE username = $1";

//根据用户名查询用户名语句
pub const SELECT_USER_USERNAME_BY_USERNAME: &str = "SELECT username FROM users WHERE username = $1";

//根据用户名查询用户名和密码语句
pub const SELECT_USER_USERNAME_AND_PASSWORD_BY_USERNAME: &str =
    "SELECT username, password FROM users WHERE username = $1";

//根据用户名查询用户信息语句
pub const SELECT_USER_INFO_BY_USERNAME: &str = "SELECT * FROM users WHERE username = $1";

//查询所有用户信息
pub const SELECT_ALL_USER: &str = "SELECT * FROM users";
