
pub const DATABASE_CONNECT_BY_EASY_CONFIG:&str="user=postgres password='password' dbname=postgres hostaddr=127.0.0.1 port=5432";
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

pub const INSERT_ONE_USER: &str = "INSERT INTO users 
(username, password, sex, birthday, country, province, city, profile_photo) 
VALUES 
($1, $2, $3, $4, $5, $6, $7, $8)";

pub const UPDATE_USER_INFO_BY_USERNAME: &str = "UPDATE users
SET username = $2, password = $3, sex = $4, birthday = $5, country = $6, province = $7, city = $8, profile_photo = $9
WHERE username = $1";

pub const DELETE_USER_BY_USERNAME: &str = "DELETE FROM users WHERE username = $1";

pub const SELECT_USER_USERNAME_BY_USERNAME: &str = "SELECT username FROM users WHERE username = $1";

pub const SELECT_USER_USERNAME_AND_PASSWORD_BY_USERNAME: &str = "SELECT username, password FROM users WHERE username = $1";

pub const SELECT_USER_INFO_BY_USERNAME: &str = "SELECT * FROM users WHERE username = $1";

pub const SELECT_ALL_USER: &str = "SELECT * FROM users";