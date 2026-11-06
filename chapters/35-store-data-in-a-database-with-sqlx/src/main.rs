use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = SqlitePool::connect("sqlite:harbor.db?mode=rwc").await?;
    sqlx::migrate!().run(&pool).await?;
    sqlx::query("INSERT INTO parcels (town, weight) VALUES (?, ?)")
        .bind("Oslo")
        .bind(1200)
        .execute(&pool)
        .await?;
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM parcels")
        .fetch_one(&pool)
        .await?;
    println!("Rows: {count}");
    Ok(())
}
