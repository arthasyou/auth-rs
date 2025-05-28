pub mod user;

use user::create_users_table;

use crate::error::Result;

pub async fn create_tables() -> Result<()> {
    create_users_table().await?;
    Ok(())
}
