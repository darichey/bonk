use futures_util::{StreamExt, TryStreamExt};
use sqlx::{Connection, SqliteConnection};

#[derive(sqlx::FromRow, Debug)]
struct Foo {
    id: i64,
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let mut conn = SqliteConnection::connect("sqlite::memory:").await?;

    sqlx::query("create table foo ( id int primary key, name string )")
        .execute(&mut conn)
        .await?;

    sqlx::query("insert into foo values (1, \"foo\")")
        .execute(&mut conn)
        .await?;

    sqlx::query("insert into foo values (2, \"bar\")")
        .execute(&mut conn)
        .await?;

    let stream = sqlx::query_as::<_, Foo>("select * from foo").fetch(&mut conn);

    println!("{:?}", stream.try_collect::<Vec<Foo>>().await?);

    Ok(())
}
