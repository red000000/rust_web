use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::types::ToSql;
use tokio_postgres::{Config, NoTls, Row};
pub struct DbPool {
    pub pool: Pool<PostgresConnectionManager<NoTls>>,
}
impl DbPool {
    pub async fn new(config: Config, min_connections: u32, max_connections: u32) -> Self {
        let manager = PostgresConnectionManager::new(config, NoTls);
        let pool = Pool::builder()
            .min_idle(min_connections)
            .max_size(max_connections)
            .build(manager)
            .await
            .unwrap();
        Self { pool }
    }
    pub async fn query(&self, query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<Vec<Row>, ()> {
        if let Ok(connetion) = self.pool.get().await {
            if let Ok(rows) = connetion.query(query, params).await {
                Ok(rows)
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }
    pub async fn query_one(&self, query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<Row, ()> {
        if let Ok(connetion) = self.pool.get().await {
            if let Ok(row) = connetion.query_one(query, params).await {
                Ok(row)
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }
    pub async fn execute(&self, query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<u64, ()> {
        if let Ok(connetion) = self.pool.get().await {
            if let Ok(u) = connetion.execute(query, params).await {
                Ok(u)
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }

}
