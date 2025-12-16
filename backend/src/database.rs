use sea_orm::{Database, DatabaseConnection, DbErr};

pub async fn establish_connection(database_url: &str) -> Result<DatabaseConnection, DbErr> {
    tracing::info!("🔌 Connecting to database...");
    
    let db = Database::connect(database_url).await?;
    
    tracing::info!("✅ Database connection established");
    
    Ok(db)
}