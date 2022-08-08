use dotenv;

use tide::{Server, Request};
use sqlx::PgPool;

#[async_std::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let db_url = std::env::var("DATABASE_URL")?;
    let db_pool = PgPool::new(&db_url).await?; 

    let mut app: Server<State> = Server::with_state(State { db_pool });

    app.at("/").get(|req: Request<State>| async move { 
        let db_pool = &req.state().db_pool;

        let rows = sqlx::query!("select 1 as one").fetch_one(db_pool).await?;
        
        Ok("Hello World")
    });
    app.listen("127.0.0.1:8080").await?;

    Ok(())
}

#[derive(Debug, Clone)]
struct State {
    db_pool: PgPool
}

#[derive(thiserror::Error, Debug)]
enum Error {
   #[error(transparent)]
   DbError(#[from] sqlx::Error),
   #[error(transparent)]
   IoError(#[from] std::io::Error),
   #[error(transparent)]
   VarError(#[from] std::env::VarError),
}