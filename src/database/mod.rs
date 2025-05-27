pub mod auth_db;

use auth_db::create_users_table;

use crate::error::Result;

pub async fn create_tables() -> Result<()> {
    create_users_table().await?;
    Ok(())
}
