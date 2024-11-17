use std::env;
use std::sync::Arc;
use tokio::sync::{OnceCell};
use sqlx::{SqlitePool};
use sqlx::sqlite::SqlitePoolOptions;

#[derive(sqlx::FromRow)]
pub struct Users {
    pub discord_id: u64,
    pub user_name: String,
    pub display_name: String,
    pub ban: bool,
    pub create_at: String,
    pub update_at: String,
}

#[derive(sqlx::FromRow)]
pub struct ReactionEmoji {
    pub id: i32,
    pub instance_num: i32,
    pub is_custom_emoji: bool,
    pub unicode_emoji: String,
    pub is_animated_custom_emoji: bool,
    pub custom_emoji_id: u64,
    pub custom_emoji_name: String,
}

#[derive(sqlx::FromRow)]
pub struct UserMissCount {
    pub discord_id: u64,
    pub count: i32,
    pub create_at: String,
    pub update_at: String,
}

#[derive(sqlx::FromRow)]
pub struct RoleAssignment {
    pub id: i32,
    pub instance_num: i32,
    pub role_id: u64,
}

#[derive(Clone)]
pub struct DbManager {
    pool: Arc<SqlitePool>,
}

static DB_MANAGER: OnceCell<DbManager> = OnceCell::const_new();

impl DbManager {
    pub async fn initialize() -> Result<(), Box<dyn std::error::Error>> {
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(db_url.as_str()).await?;

        let db_manager = DbManager { pool: Arc::new(pool) };

        DB_MANAGER.set(db_manager).map_err(|_| "Failed to set DB manager".into())
    }

    pub fn instance() -> &'static DbManager {
        DB_MANAGER.get().expect("DB manager is not initialized")
    }

    pub fn get_connection() -> Arc<SqlitePool> {
        let instance = Self::instance();
        Arc::clone(&instance.pool)
    }
}