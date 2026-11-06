# 35 - Store Data in a Database with SQLx

Why does your data vanish when the program stops? A notepad and a filing cabinet explain databases. With SQLx and SQLite we create a table, insert rows, read them into structs, handle a row that isn't there, see SQL injection happen and fix it with placeholders, catch a typo at compile time, group work in a transaction, and manage tables with migrations.

## Run it

```bash
cargo run
```

This folder holds the program as it stands at the end of the chapter.

## Commands used in the video

```bash
cargo add sqlx --features sqlite,runtime-tokio
cargo add tokio --features full
cargo run
ls
rm harbor.db
```

## Every version of `src/main.rs` from the video

Compiles and runs:

```rust
fn main() {
}
```

Compiles and runs:

```rust
use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = SqlitePool::connect("sqlite:harbor.db?mode=rwc").await?;
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS parcels (
            id INTEGER PRIMARY KEY,
            town TEXT NOT NULL,
            weight INTEGER NOT NULL
        )",
    )
    .execute(&pool)
    .await?;
    println!("Ready");
    Ok(())
}
```

Compiles and runs:

```rust
use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = SqlitePool::connect("sqlite:harbor.db?mode=rwc").await?;
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS parcels (
            id INTEGER PRIMARY KEY,
            town TEXT NOT NULL,
            weight INTEGER NOT NULL
        )",
    )
    .execute(&pool)
    .await?;
    let result = sqlx::query("INSERT INTO parcels (town, weight) VALUES (?, ?)")
        .bind("Oslo")
        .bind(1200)
        .execute(&pool)
        .await?;
    println!("New id: {}", result.last_insert_rowid());
    Ok(())
}
```

Compiles and runs:

```rust
use sqlx::sqlite::SqlitePool;

#[derive(sqlx::FromRow)]
struct Parcel {
    id: i64,
    town: String,
    weight: i64,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = SqlitePool::connect("sqlite:harbor.db?mode=rwc").await?;
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS parcels (
            id INTEGER PRIMARY KEY,
            town TEXT NOT NULL,
            weight INTEGER NOT NULL
        )",
    )
    .execute(&pool)
    .await?;
    let result = sqlx::query("INSERT INTO parcels (town, weight) VALUES (?, ?)")
        .bind("Bergen")
        .bind(1200)
        .execute(&pool)
        .await?;
    println!("New id: {}", result.last_insert_rowid());
    let parcels: Vec<Parcel> = sqlx::query_as("SELECT id, town, weight FROM parcels")
        .fetch_all(&pool)
        .await?;
    for p in parcels {
        println!("{} {} {}", p.id, p.town, p.weight);
    }
    Ok(())
}
```

Compiles and runs:

```rust
use sqlx::sqlite::SqlitePool;

#[derive(sqlx::FromRow)]
struct Parcel {
    id: i64,
    town: String,
    weight: i64,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = SqlitePool::connect("sqlite:harbor.db?mode=rwc").await?;
    let found: Option<Parcel> = sqlx::query_as("SELECT * FROM parcels WHERE id = ?")
        .bind(99)
        .fetch_optional(&pool)
        .await?;
    match found {
        Some(p) => println!("Found {} {} {}", p.id, p.town, p.weight),
        None => println!("No such parcel"),
    }
    Ok(())
}
```

Compiles and runs:

```rust
use sqlx::sqlite::SqlitePool;

#[derive(sqlx::FromRow)]
struct Parcel {
    id: i64,
    town: String,
    weight: i64,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = SqlitePool::connect("sqlite:harbor.db?mode=rwc").await?;
    let found: Option<Parcel> = sqlx::query_as("SELECT * FROM parcels WHERE id = ?")
        .bind(1)
        .fetch_optional(&pool)
        .await?;
    match found {
        Some(p) => println!("Found {} {} {}", p.id, p.town, p.weight),
        None => println!("No such parcel"),
    }
    Ok(())
}
```

Compiles and runs:

```rust
use sqlx::sqlite::SqlitePool;

#[derive(sqlx::FromRow)]
struct Parcel {
    id: i64,
    town: String,
    weight: i64,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = SqlitePool::connect("sqlite:harbor.db?mode=rwc").await?;
    let town = "x' OR '1'='1";
    let sql = format!("SELECT id, town, weight FROM parcels WHERE town = '{town}'");
    let rows: Vec<Parcel> = sqlx::query_as(sqlx::AssertSqlSafe(sql))
        .fetch_all(&pool)
        .await?;
    println!("{} rows", rows.len());
    for p in rows {
        println!("{} {} {}", p.id, p.town, p.weight);
    }
    Ok(())
}
```

Compiles and runs:

```rust
use sqlx::sqlite::SqlitePool;

#[derive(sqlx::FromRow)]
struct Parcel {
    id: i64,
    town: String,
    weight: i64,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = SqlitePool::connect("sqlite:harbor.db?mode=rwc").await?;
    let town = "x' OR '1'='1";
    let rows: Vec<Parcel> = sqlx::query_as("SELECT id, town, weight FROM parcels WHERE town = ?")
        .bind(town)
        .fetch_all(&pool)
        .await?;
    println!("{} rows", rows.len());
    for p in rows {
        println!("{} {} {}", p.id, p.town, p.weight);
    }
    Ok(())
}
```

Does not compile, on purpose, so we can read the error:

```rust
use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = SqlitePool::connect("sqlite:harbor.db?mode=rwc").await?;
    let rows = sqlx::query!("SELECT town FROM parcels")
        .fetch_all(&pool)
        .await?;
    for row in rows {
        println!("{}", row.town);
    }
    Ok(())
}
```

Does not compile, on purpose, so we can read the error:

```rust
use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = SqlitePool::connect("sqlite:harbor.db?mode=rwc").await?;
    let rows = sqlx::query!("SELECT towne FROM parcels")
        .fetch_all(&pool)
        .await?;
    for row in rows {
        println!("{}", row.town);
    }
    Ok(())
}
```

Compiles and runs:

```rust
use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = SqlitePool::connect("sqlite:harbor.db?mode=rwc").await?;
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM parcels")
        .fetch_one(&pool)
        .await?;
    println!("Before: {count}");
    let mut tx = pool.begin().await?;
    sqlx::query("INSERT INTO parcels (town, weight) VALUES (?, ?)")
        .bind("Tromso")
        .bind(900)
        .execute(&mut *tx)
        .await?;
    drop(tx);
    let after: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM parcels")
        .fetch_one(&pool)
        .await?;
    println!("After: {after}");
    Ok(())
}
```

Compiles and runs:

```rust
use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = SqlitePool::connect("sqlite:harbor.db?mode=rwc").await?;
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM parcels")
        .fetch_one(&pool)
        .await?;
    println!("Before: {count}");
    let mut tx = pool.begin().await?;
    sqlx::query("INSERT INTO parcels (town, weight) VALUES (?, ?)")
        .bind("Tromso")
        .bind(900)
        .execute(&mut *tx)
        .await?;
    tx.commit().await?;
    let after: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM parcels")
        .fetch_one(&pool)
        .await?;
    println!("After: {after}");
    Ok(())
}
```

Does not compile, on purpose, so we can read the error:

```rust
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
```

## Sources

- SQLx README and features: https://github.com/launchbadge/sqlx
- SqliteConnectOptions and mode=rwc: https://docs.rs/sqlx/latest/sqlx/sqlite/struct.SqliteConnectOptions.html
- query macro needs DATABASE_URL, offline mode: https://docs.rs/sqlx/latest/sqlx/macro.query.html
- migrate macro: https://docs.rs/sqlx/latest/sqlx/macro.migrate.html
- Transaction rolls back on drop: https://docs.rs/sqlx/latest/sqlx/struct.Transaction.html
- SQLite integers up to 8 bytes: https://www.sqlite.org/datatype3.html
- SQL injection, OWASP: https://owasp.org/www-community/attacks/SQL_Injection
