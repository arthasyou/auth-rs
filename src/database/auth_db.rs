use service_utils_rs::services::db::get_db;

use crate::error::Result;
use crate::models::auth_model::{User, UserInput};

pub async fn create_users_table() -> Result<()> {
    let db = get_db();

    // 检查表是否存在
    let check_query = "SHOW TABLES";
    let result: Vec<String> = db.query(check_query).await?.take(0)?;

    // 检查返回的表名是否包含 "users"
    let tables: Vec<String> = result.iter().map(|row| row.to_string()).collect(); // 假设返回的是表名列表
    if !tables.contains(&"users".to_string()) {
        // 如果表不存在，则创建表
        let create_query = "CREATE TABLE users (
            id INT AUTO_INCREMENT PRIMARY KEY,
            name STRING,
            email STRING
        );";
        let r = db.query(create_query).await?;
        println!("Created users table: {:?}", r);
    } else {
        println!("Users table already exists");
    }

    Ok(())
}

pub async fn insert_user(user: UserInput) -> Result<()> {
    let db = get_db();
    let r: Option<User> = db.insert(("users", &user.username)).content(user).await?;
    println!("inserted user: {:?}", r);
    Ok(())
}

pub async fn get_user(username: &str) -> Result<Option<User>> {
    let db = get_db();
    let r: Option<User> = db.select(("users", username)).await?;
    println!("get user: {:?}", r);
    Ok(r)
}
