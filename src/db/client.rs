use crate::{
    configuration::DatabaseSettings,
    nurls::{Nurl, Nurlet},
};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug)]
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

    #[tracing::instrument(name = "Save nurl")]
    pub async fn save_nurl(&self, nurl: &Nurl) -> Result<(), sqlx::Error> {
        let mut transaction = self.pool.begin().await?;
        sqlx::query!(
            r#"
        INSERT INTO nurls(id, title) VALUES ($1, $2);
                     "#,
            nurl.id,
            nurl.title
        )
        .execute(&mut transaction)
        .await?;

        for url in &nurl.urls {
            // sqlx doesn't really support multiple inserts yet?
            let (payload, variant) = match url {
                Nurlet::Url(s) => (s, 0),
                Nurlet::Banner(s) => (s, 1),
            };
            sqlx::query!(
                r#"
            INSERT INTO urls(payload,variant, nurl) VALUES ($1, $2, $3);
                        "#,
                &payload,
                variant,
                nurl.id
            )
            .execute(&mut transaction)
            .await?;
        }

        transaction.commit().await?;
        Ok(())
    }

    #[tracing::instrument(name = "Add nurl view")]
    pub async fn add_view(&self, nurl: &Nurl) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
        UPDATE nurls SET views=views+1 WHERE id=$1;
            "#,
            nurl.id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
    #[tracing::instrument(name = "Get nurl")]
    pub async fn get_nurl(&self, uuid: Uuid) -> Result<Option<Nurl>, sqlx::Error> {
        let nurl_result = sqlx::query!(
            r#"
        SELECT title,views FROM nurls WHERE id=$1;
            "#,
            uuid,
        )
        .fetch_optional(&self.pool)
        .await?;
        match nurl_result {
            None => Ok(None),
            Some(nurl_result) => {
                let urls: Vec<Nurlet> = self.get_url_set(uuid).await?;
                let nurl = Nurl {
                    title: nurl_result.title,
                    id: uuid,
                    views: nurl_result.views,
                    urls: urls,
                };
                Ok(Some(nurl))
            }
        }
    }
    #[tracing::instrument(name = "Get url set")]
    pub async fn get_url_set(&self, uuid: Uuid) -> Result<Vec<Nurlet>, sqlx::Error> {
        let result = sqlx::query!(
            r#"
                SELECT payload,variant FROM urls WHERE nurl=$1;
            "#,
            uuid,
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(result
            .into_iter()
            .map(|r| match r.variant {
                0 => Nurlet::Url(r.payload),
                _ => Nurlet::Banner(r.payload),
            })
            .collect())
    }
}
