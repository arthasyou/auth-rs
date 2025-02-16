pub mod auth_db;

use crate::error::Result;
use auth_db::create_users_table;

pub async fn create_tables() -> Result<()> {
    create_users_table().await?;
    Ok(())
}
