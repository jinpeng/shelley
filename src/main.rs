use std::env;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use sqlx::mysql::MySqlPool;

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
pub struct Poem {
    pub id: uuid::Uuid,
    pub author: String,
    pub title: String,
    pub poem_type: i32,
    pub dynasty: i32,
    pub paragraphs: String,
    pub create_time: DateTime<Utc>,
    pub update_time: DateTime<Utc>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Hello, world!");
    let pool = MySqlPool::connect(&env::var("DATABASE_URL")?).await?;
    list_poems(&pool).await?;
    Ok(())
}

async fn list_poems(pool: &MySqlPool) -> anyhow::Result<()> {
    let recs = sqlx::query!(
        r#"
SELECT id, author, title
FROM poem
ORDER BY id
limit 10
        "#
    )
    .fetch_all(pool)
    .await?;

    // NOTE: Booleans in MySQL are stored as `TINYINT(1)` / `i8`
    //       0 = false, non-0 = true
    for rec in recs {
        println!(
            "- {} by {}: {}",
            &rec.id,
            &rec.author,
            &rec.title,
        );
    }

    Ok(())
}