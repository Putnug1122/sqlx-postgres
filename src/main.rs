use futures::TryStreamExt;
use sqlx::postgres::PgPoolOptions;
use std::env;

#[derive(sqlx::FromRow)]
struct Todo {
    id: i32,
    title: String,
    done: bool,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Could not load the .env file!");
    let database_url =
        env::var("DATABASE_URL").expect("The environment variable DATABASE URL is missing!");

    let pool = PgPoolOptions::new()
        .connect(&database_url)
        .await
        .expect("Failed to connect to the database");

    let first_todo_title = "Learn SQLx";
    sqlx::query("INSERT INTO todos (title) VALUES ($1)")
        .bind(first_todo_title)
        .execute(&pool)
        .await
        .unwrap();

    let first_todo = sqlx::query_as::<_, Todo>("SELECT * FROM todos WHERE title=$1")
        .bind(first_todo_title)
        .fetch_one(&pool)
        .await
        .unwrap();

    println!(
        "query_as: id={}, title={}, done={:?}",
        first_todo.id, first_todo.title, first_todo.done
    );
}
