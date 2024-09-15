use sqlx::SqlitePool;
use std::sync::Arc;
use warp::{http::StatusCode, reject::Rejection, reply, Filter, Reply};

/// Custom error type for unauthorized requests.
#[derive(Debug)]
pub struct Unauthorized;

/// Implement the [`Reject`] trait for [`Unauthorized`].
impl warp::reject::Reject for Unauthorized {}

/// Implement the [`Display`] trait for [`Unauthorized`].
impl std::fmt::Display for Unauthorized {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Unauthorized")
    }
}

/// Implement the [`Error`] trait for [`Unauthorized`].
impl std::error::Error for Unauthorized {}

/// Custom error type for sqlx errors.
#[derive(Debug)]
pub struct Sqlx(pub sqlx::Error);

/// Implement the [`Reject`] trait for [`Sqlx`].
impl warp::reject::Reject for Sqlx {}

/// Implement the [`Display`] trait for [`Sqlx`].
impl std::fmt::Display for Sqlx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.to_string())
    }
}

/// Implement the [`Error`] trait for [`Sqlx`].
impl std::error::Error for Sqlx {}

/// Custom rejection handler that maps rejections into responses.
async fn handle_rejection(err: Rejection) -> Result<impl Reply, Rejection> {
    println!("handle_rejection");
    if err.is_not_found() {
        Ok(reply::with_status("NOT_FOUND", StatusCode::NOT_FOUND))
    } else if let Some(_e) = err.find::<Sqlx>() {
        Ok(reply::with_status("BAD_REQUEST", StatusCode::BAD_REQUEST))
    } else {
        eprintln!("unhandled rejection: {:?}", err);
        Ok(reply::with_status(
            "INTERNAL_SERVER_ERROR",
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    }
}

/// DB struct for purchases.
#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize, serde::Deserialize, PartialEq)]
struct Purchase {
    id: i64,
    user_id: i64,
    casino_id: i64,
    cost: i64,
    benefit: i64,
    created_at: chrono::NaiveDateTime,
    notes: Option<String>,
}

/// DB struct for users.
#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize, serde::Deserialize, PartialEq)]
struct User {
    id: i64,
    email: String,
    username: String,
    avatar: Option<String>,
    discord_id: Option<String>,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
}

/// Struct for the json response body for purchases.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
struct PurchasesReplyBody {
    body: Vec<Purchase>,
}

/// Struct for the json response body for users.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
struct UserReplyBody {
    body: Vec<User>,
}

/// Context object for casino buddy.
#[derive(Debug, Clone)]
struct CasinoContext {
    // We want to be able to run without a database and export a state file I think
    db: Arc<SqlitePool>,
}

/// Implementation for `[CasinoContext]
impl CasinoContext {
    fn new(db: SqlitePool) -> Self {
        Self { db: Arc::new(db) }
    }

    async fn get_purchases(self: &Self, user_id: i64) -> Result<Vec<Purchase>, sqlx::Error> {
        let purchases = sqlx::query_as!(
            Purchase,
            "SELECT * FROM purchase WHERE user_id = $1",
            user_id
        )
        .fetch_all(&*self.db)
        .await?;
        Ok(purchases)
    }

    async fn get_user(self: &Self, user_id: i64) -> Result<Vec<User>, sqlx::Error> {
        let user = sqlx::query_as!(User, "SELECT * FROM user WHERE id = $1", user_id)
            .fetch_all(&*self.db)
            .await?;
        Ok(user)
    }

    async fn process_get_purchases(self: &Self, user_id: i64) -> Result<impl Reply, Rejection> {
        let purchases = self.get_purchases(user_id).await.map_err(Sqlx)?;
        Ok(warp::reply::json(&PurchasesReplyBody { body: purchases }))
    }

    async fn process_get_user(self: &Self, user_id: i64) -> Result<impl Reply, Rejection> {
        let user = self.get_user(user_id).await.map_err(Sqlx)?;
        Ok(warp::reply::json(&UserReplyBody { body: user }))
    }
}

/// Default object for `[CasinoContext]`
impl Default for CasinoContext {
    fn default() -> Self {
        let db: SqlitePool = sqlx::SqlitePool::connect_lazy("sqlite::memory")
            .expect("Failed to connect to database");
        Self::new(db)
    }
}

/// Get all transactions for a user.
async fn transaction_filter(
    ctx: CasinoContext,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let context = warp::any().map(move || ctx.clone());
    //.and(log_headers())
    //.and(log_body())

    warp::path!("purchases" / i64)
        .and(warp::get())
        .and(context)
        .and_then(|user_id: i64, inner_ctx: CasinoContext| async move {
            inner_ctx.clone().process_get_purchases(user_id).await
        })
        .recover(handle_rejection)
}

/// Get all transactions for a user.
async fn filter_get_user_by_id(
    ctx: CasinoContext,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let context = warp::any().map(move || ctx.clone());
    //.and(log_headers())
    //.and(log_body())

    warp::path!("user" / i64)
        .and(warp::get())
        .and(context)
        .and_then(|user_id: i64, inner_ctx: CasinoContext| async move {
            inner_ctx.clone().process_get_user(user_id).await
        })
        .recover(handle_rejection)
}

/// Get the routes for the server.
async fn get_app(
    ctx: CasinoContext,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    println!("get_app");
    let transaction_filter = transaction_filter(ctx.clone()).await;
    let user_filter = filter_get_user_by_id(ctx).await;
    let health = warp::path!("health").map(|| "Hello, world!");
    let log = warp::log("crack-voting");
    health.or(transaction_filter).or(user_filter).with(log)
}

/// Run the server.
pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let ctx = CasinoContext::default(); //Box::leak(Box::new(VotingContext::new().await));
    let app = get_app(ctx).await;

    warp::serve(app).run(([0, 0, 0, 0], 3030)).await;

    Ok(())
}
