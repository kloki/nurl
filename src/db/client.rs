use crate::{configuration::DatabaseSettings, nurls::Nurl};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use url::Url;
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
        INSERT INTO nurls(id) VALUES ($1);
                     "#,
            nurl.id
        )
        .execute(&mut transaction)
        .await?;

        for url in &nurl.urls {
            // sqlx doesn't really support multiple inserts yet?
            sqlx::query!(
                r#"
            INSERT INTO urls(url, nurl) VALUES ($1, $2);
                        "#,
                &url.to_string(),
                nurl.id
            )
            .execute(&mut transaction)
            .await?;
        }

        transaction.commit().await?;
        Ok(())
    }

    #[tracing::instrument(name = "Add nurl view")]
    pub async fn add_view(&mut self, nurl: &Nurl) -> Result<(), sqlx::Error> {
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
        SELECT views FROM nurls WHERE id=$1;
            "#,
            uuid,
        )
        .fetch_optional(&self.pool)
        .await?;
        match nurl_result {
            None => Ok(None),
            Some(nurl_result) => {
                let nurl = Nurl {
                    id: uuid,
                    views: nurl_result.views,
                    urls: self.get_url_set(uuid).await?,
                };
                Ok(Some(nurl))
            }
        }
    }
    #[tracing::instrument(name = "Get url set")]
    pub async fn get_url_set(&self, uuid: Uuid) -> Result<Vec<Url>, sqlx::Error> {
        let result = sqlx::query!(
            r#"
                SELECT url FROM urls WHERE nurl=$1;
            "#,
            uuid,
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(result
            .into_iter()
            .map(|r| r.url.parse::<Url>().unwrap())
            .collect())
    }
}
