use log::{info};
use pretty_env_logger;
use dotenvy::{self, dotenv};
use sqlx::postgres::PgPoolOptions;
use axum::{
    routing::{delete, get, post},
    Router,
};

mod handlers;
mod models;

use handlers::*;


#[tokio::main]
async fn main() {

    // Initialize pretty_env_logger
    // Initialize dotenv
    let pretty_env_logger = pretty_env_logger::init();
    let dotenv = dotenv().ok();

    // create pg connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL must be set."))
        .await
        .expect("Failed to create Postgres connection pool!");

    
    // Using slqx, execute a SQL query that selects all questions from the questions table.
    // Use the `unwrap` or `expect` method to handle errors. This is just some test code to
    // make sure we can connect to the database.
    let recs = sqlx::query!("SELECT * FROM questions")
        .fetch_all(&pool)
        .await
        .unwrap();

    info!("********* Question Records *********");
    info!("{:?}", recs);

    let app = Router::new()
        .route("/question", post(create_question))
        .route("/questions", get(read_questions))
        .route("/question", delete(delete_question))
        .route("/answer", post(create_answer))
        .route("/answers", get(read_answers))
        .route("/answer", delete(delete_answer));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}