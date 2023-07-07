use crate::{configuration::DatabaseSettings, nurls::Nurl};
use sqlx::postgres::PgPoolOptions;
use uuid::Uuid;

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

    pub fn save(&self, nurl: Nurl) -> Result<(), sqlx::Error> {
        Ok(())
    }

    pub fn get(&self, uuid: Uuid) -> Result<Option<Nurl>, sqlx::Error> {
        Ok(None)
    }
}
