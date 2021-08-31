use std::env;
use chrono::{NaiveDateTime, NaiveDate};
use serde::{Serialize, Deserialize};
use sqlx::mysql::MySqlPool;

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
pub struct Poem {
    pub id: String,
    pub author: String,
    pub title: String,
    pub poem_type: i8,
    pub dynasty: i8,
    pub paragraphs: String,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime,
}

// Not neccessory
impl Default for Poem {
    fn default() -> Self {
        Poem {
            id: "".to_string(),
            author: "".to_string(),
            title: "".to_string(),
            poem_type: 100,
            dynasty: 100,
            paragraphs: "".to_string(),
            create_time: NaiveDate::from_ymd(1970, 1, 1).and_hms(0, 0, 0),
            update_time: NaiveDate::from_ymd(1970, 1, 1).and_hms(0, 0, 0),
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pool = MySqlPool::connect(&env::var("DATABASE_URL")?).await?;
    let poems = list_poems(&pool).await?;
    println!("{:#?}", poems);
    Ok(())
}

async fn list_poems(pool: &MySqlPool) -> Result<Vec<Poem>, sqlx::Error> {
    let recs = sqlx::query_as!(
        Poem,
        r#"
SELECT id, author, title, type as poem_type, dynasty, paragraphs, create_time, update_time
FROM poem
ORDER BY id
limit 10
        "#
    )
    .fetch_all(pool)
    .await?;

    // NOTE: Booleans in MySQL are stored as `TINYINT(1)` / `i8`
    //       0 = false, non-0 = true
    // for rec in recs {
    //     println!(
    //         "- {} by {}: {}",
    //         &rec.id,
    //         &rec.author,
    //         &rec.title,
    //     );
    // }

    Ok(recs)
}