use crate::configuration::DatabaseSettings;
use sqlx::postgres::PgPoolOptions;

use sqlx::PgPool;
pub struct DBClient {
    pool: PgPool,
}

impl DBClient {
    pub fn new(configuration: &DatabaseSettings) -> DBClient {
        DBClient {
            pool: PgPoolOptions::new()
                .acquire_timeout(std::time::Duration::from_secs(2))
                .connect_lazy_with(configuration.with_db()),
        }
    }
}
