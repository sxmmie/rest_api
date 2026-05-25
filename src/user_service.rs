use sqlx::{Error, PgPool, postgres::PgPoolOptions};

pub struct UserService {
    pool: PgPool,
}

impl UserService {
    pub async fn new() -> Result<Self, Error> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect("postgresql://postgres@localhost/postgres")
            .await?;

        Ok(Self { pool })
    }
}
