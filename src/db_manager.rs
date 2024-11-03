use std::env;
use std::sync::Arc;
use tokio::sync::{Mutex, OnceCell};
use sqlx::{Database, Pool, Sqlite, SqlitePool};
use sqlx::sqlite::SqlitePoolOptions;

#[derive(sqlx::FromRow)]
pub struct Users {
    pub discordId: u64,
    pub userName: String,
    pub displayName: String,
    pub ban: bool,
    pub createAt: String,
    pub updateAt: String,
}

#[derive(sqlx::FromRow)]
pub struct ReactionEmoji {
    pub id: i32,
    pub instanceNum: i32,
    pub isCustomEmoji: bool,
    pub unicodeEmoji: String,
    pub isAnimatedCustomEmoji: bool,
    pub customEmojiId: u64,
    pub customEmojiName: String,
}

#[derive(sqlx::FromRow)]
pub struct UserMissCount {
    pub discordId: u64,
    pub count: i32,
    pub createAt: String,
    pub updateAt: String,
}

#[derive(sqlx::FromRow)]
pub struct RoleAssignment {
    pub id: i32,
    pub instanceNum: i32,
    pub roleId: u64,
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