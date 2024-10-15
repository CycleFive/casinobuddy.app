use rust_decimal::BigDecimal;
use tracing_subscriber::fmt::format::FmtSpan;
use uuid::Uuid;
use sqlx::PgPool;
use std::{convert::Infallible, str::FromStr, sync::Arc};
use warp::{http::StatusCode, reject::Rejection, reply, Filter, Reply};

pub mod error;
pub use error::*;

const DEFAULT_DATABASE_URL: &str = "postgresql://postgres:mysecretpassword@localhost:5432/casinobuddy";

/// Custom error type for sqlx errors.
#[derive(Debug)]
pub struct Sqlx(pub sqlx::Error);

/// Implement the [`warp::reject::Reject`] trait for [`Sqlx`].
impl warp::reject::Reject for Sqlx {}

/// Implement the [`std::fmt::Display`] trait for [`Sqlx`].
impl std::fmt::Display for Sqlx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.to_string())
    }
}

/// Implement the [`std::error::Error`] trait for [`Sqlx`].
impl std::error::Error for Sqlx {}

/// Custom rejection handler that maps rejections into responses.
async fn handle_rejection(err: Rejection) -> Result<impl Reply, Rejection> {
    tracing::warn!("handle_rejection");
    tracing::warn!("{:?}", err);
    if err.is_not_found() {
        tracing::error_span!("not found");
        Ok(reply::with_status(warp::reply::json( &error::NotFound), StatusCode::NOT_FOUND))
    } else if let Some(e) = err.find::<Sqlx>() {
        tracing::error!("sqlx error: {:?}", e);
        Ok(reply::with_status(warp::reply::json(&error::BadRequest), StatusCode::BAD_REQUEST))
    } else {
        eprintln!("unhandled rejection: {:?}", err);
        Ok(reply::with_status(
        warp::reply::json(&error::InternalServerError),
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    }
}

/// DB struct for transactions.
#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Transaction {
    pub id:         uuid::Uuid,
    pub user_id:    uuid::Uuid,
    pub casino_id:  uuid::Uuid,
    pub cost:       BigDecimal,
    pub benefit:    BigDecimal,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub notes:      Option<String>,
}

/// DB struct for redemptions.
#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Redemption {
    pub id: Uuid,
    pub user_id: Uuid,
    pub casino_id: Uuid,
    pub amount: BigDecimal,
    pub created_at: chrono::NaiveDateTime,
    pub received_at: Option<chrono::NaiveDateTime>,
}

/// DB struct for users.
#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct User {
    pub id: Uuid, // FIXME: Make this uuid4
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

/// Struct for the json response body for transactions.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct TransactionsReplyBody {
    pub body: Vec<Transaction>,
}

/// Struct for the json response body for users.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct UserReplyBody {
    pub body: Vec<User>,
}

/// Context object for casino buddy.
#[derive(Debug, Clone)]
struct CasinoContext {
    // We want to be able to run without a database and export a state file I think
    // Do I really need to use Arc here? There is already an arc in the Pool...
    db: Arc<PgPool>,
}

/// Custom type for a user id.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CBUserId {
    pub id: Uuid,
}

use serde_json::json;

/// Implementation for [CasinoContext]
impl CasinoContext {
    /// Create a new instance of `[CasinoContext]` given a database pool.
    fn new(db: PgPool) -> Self {
        Self { db: Arc::new(db) }
    }

    /// Get all transactions for a user.
    async fn get_transactions(&self, user_id: Uuid) -> Result<Vec<Transaction>, sqlx::Error> {
        let transactions = sqlx::query_as!(
                Transaction,
                r#"SELECT * FROM "transaction" WHERE user_id = $1"#,
                user_id
            )
            .fetch_all(&*self.db)
            .await?;
        Ok(transactions)
    }

    /// Get all transactions for a user.
    async fn _get_transactions_all(&self) -> Result<Vec<Transaction>, sqlx::Error> {
        let transactions = sqlx::query_as!(
                Transaction,
                r#"SELECT * FROM "transaction" ORDER BY created_at desc RETURNING *"#,
            )
            .fetch_all(&*self.db)
            .await?;
        Ok(transactions)
    }

    /// Get a user by their id.
    async fn get_user(&self, user_id: Uuid) -> Result<Vec<User>, sqlx::Error> {
        let user = sqlx::query_as!(User, r#"SELECT * FROM "user" WHERE id = $1"#, user_id)
            .fetch_all(&*self.db)
            .await?;
        Ok(user)
    }

    /// Check if a user and/or email already exists.
    async fn check_username_email(&self, _email: &str, _username: &str) -> Result<bool, sqlx::Error> {
        Ok(true)
    }
    // async fn check_username_email(&self, email: &str, username: &str) -> Result<bool, sqlx::Error> {
    //     // We are entirely using the Errors to communicate what has happened here.
    //     // I think this is the idiomatic way to do this in Rust, since they are also
    //     // error states.
    //     let matches = sqlx::query!(r#"SELECT email, COUNT(*) as cnt FROM user WHERE email = $1 GROUP BY email"#, email)
    //         .fetch_one(&*self.db)
    //         .await?;
    //     if matches.cnt > 0 {
    //         return Ok(false)
    //     }

    //     let matches = sqlx::query!(
    //         r#"SELECT COUNT(*) as cnt FROM user WHERE username = $1 GROUP BY username"#,
    //         username
    //     )
    //     .fetch_one(&*self.db)
    //     .await?;
    //     if matches.cnt > 0 {
    //         return Ok(false);
    //     }
    //     Ok(true)
    // }

    /// Create a new user.
    async fn create_user(&self, email: &str, username: &str) -> Result<CBUserId, sqlx::Error> {
        tracing::trace!("Creating user with email: {} and username: {}", email, username);
        // This shouldn't fail because we don't have any constraints on the email or username.
        // Do we want to constrain these in the database?
        // Should be checking for existing users with the same email or username?
        let discord_id = "0";
        let user_id = sqlx::query_as!(
            CBUserId,
            "INSERT INTO user created_at VALUES NOW() RETURNING id"
        )
        .fetch_one(&*self.db)
        .await?;
        Ok(user_id)
    }

    /// Create a new transaction, these are the purchases of coins from the casinos.
    async fn create_transaction(
        &self,
        user_id: Uuid,
        casino_id: Uuid,
        cost: BigDecimal,
        benefit: BigDecimal,
        notes: Option<String>,
    ) -> Result<Transaction, sqlx::Error> {
        let transaction = sqlx::query_as!(
            Transaction,
            r#"INSERT INTO "transaction" (user_id, casino_id, cost, benefit, notes) VALUES ($1, $2, $3, $4, $5) RETURNING *"#,
            user_id,
            casino_id,
            cost,
            benefit,
            notes
        )
        .fetch_one(&*self.db)
        .await?;
        Ok(transaction)
    }

    //TODO: Create a redemption entry.

    /// Process a request to get all transactions for a user.
    async fn process_get_transaction(&self, user_id: Uuid) -> Result<impl Reply, Rejection> {
        tracing::info!("Getting transactions with user_id: {}", user_id);
        let transactions = self.get_transactions(user_id).await.map_err(Sqlx)?;
        let res = if transactions.is_empty() {
            warp::reply::json(&json!({}))
        } else {
            warp::reply::json(&TransactionsReplyBody { body: transactions })
        };
        Ok(res)
    }

    /// Process a request to get a user by their id.
    async fn process_get_user(&self, user_id: Uuid) -> Result<impl Reply, Rejection> {
        tracing::info!("Getting user with id: {}", user_id);
        let user = self.get_user(user_id).await.map_err(Sqlx)?;
        Ok(warp::reply::json(&UserReplyBody { body: user }))
    }

    /// Process a request to create a new user.
    async fn process_post_user(
        &self,
        email: &str,
        username: &str,
    ) -> Result<impl Reply, Rejection> {
        let duplicate = self.check_username_email(email, username)
            .await
            .map_err(Sqlx)?;
        if duplicate {
            let user_id = self.create_user(email, username).await.map_err(Sqlx)?;
            Ok(warp::reply::json(&user_id))
        } else {
            Ok(warp::reply::json(&json!({"error": "User already exists"})))
        }
    }

    /// Process a request to create a new transaction.
    async fn process_post_transaction(
        &self,
        user_id: Uuid,
        casino_id: Uuid,
        cost: BigDecimal,
        benefit: BigDecimal,
        notes: &Option<String>,
    ) -> Result<impl Reply, Rejection> {
        let transaction = self.create_transaction(user_id, casino_id, cost, benefit, notes.clone()).await.map_err(Sqlx)?;
        Ok(warp::reply::with_status(warp::reply::json(&transaction), StatusCode::CREATED))
    }
}

/// Default object for `[CasinoContext]`
impl Default for CasinoContext {
    fn default() -> Self {
        // Try to get the database from the environment.
        let url = match std::env::var("DATABASE_URL") {
            Ok(val) => val,
            Err(_) => DEFAULT_DATABASE_URL.to_string(),
        };
        let db: sqlx::Pool<sqlx::Postgres> = sqlx::PgPool::connect_lazy(&url)
            .expect("Failed to connect to database");
        Self::new(db)
    }
}



/// Get a user by their id.
async fn get_user_filter(
    ctx: CasinoContext,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let context = warp::any().map(move || ctx.clone());

    warp::path!("user" / String)
        .and(warp::get())
        .and(context)
        .and_then(|user_id: String, inner_ctx: CasinoContext| async move {
            let user_id = Uuid::parse_str(&user_id).unwrap();
            tracing::info!("Getting user with id: {}", user_id);
            inner_ctx.process_get_user(user_id).await
        })
}


/// Struct for the json query body for create adding a transaction.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
struct TransactionCreate {
    // user_id: i64,
    // casino_id: i64,
    cost: BigDecimal,
    benefit: BigDecimal,
    notes: Option<String>,
}

/// Filter to the transaction create params.
fn with_transaction_create_params(
    params: TransactionCreate,
) -> impl Filter<Extract = (TransactionCreate,), Error = Infallible> + Clone {
    warp::any().map(move || params.clone())
}

/// Post filter for transactions.
async fn transaction_post_filter(
    ctx: CasinoContext,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let context = warp::any().map(move || ctx.clone());

    // POST /transaction/{user_id}/{casino_id}
    warp::path!("transaction" / String / String)
    .and(warp::post())
        .and(warp::body::json())
        .and(context)
        .and_then(
            |user_id: String, casino_id: String, params: TransactionCreate, inner_ctx: CasinoContext| async move {
                let user_id: Uuid = Uuid::from_str(&user_id).map_err(|_| BadRequest)?;
                let casino_id: Uuid = Uuid::from_str(&casino_id).map_err(|_| BadRequest)?;
                // params.user_id = user_id as i64;
                // params.casino_id = casino_id as i64;
                with_transaction_create_params(params.clone());
                inner_ctx
                    .clone()
                    .process_post_transaction(user_id, casino_id, params.cost, params.benefit, &params.notes)
                    .await
            },
        )
}

/// Get all transactions for a user.
async fn transaction_get_filter(
    ctx: CasinoContext,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let context = warp::any().map(move || ctx.clone());

    warp::path!("transaction" / String)
        .and(warp::get())
        .and(context)
        .and_then(|user_id: String, inner_ctx: CasinoContext| async move {
            let user_id = user_id.parse::<Uuid>().unwrap();
            tracing::info!("Getting transactions with user_id: {}", user_id);
            inner_ctx.process_get_transaction(user_id).await
        })
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
struct UserCreate {
    email: String,
    username: String,
}

/// Filter to get the items db.
fn with_user_create_params(
    params: UserCreate,
) -> impl Filter<Extract = (UserCreate,), Error = Infallible> + Clone {
//     warp::body::content_length_limit(1024 * 16).and(warp::body::json())
    warp::any().map(move || params.clone())
}

/// Post a new user.
async fn post_user_filter(
    ctx: CasinoContext,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let context = warp::any().map(move || ctx.clone());

    warp::path("user")
        .and(warp::post())
        .and(warp::path::end())
        .and(context)
        .and(warp::body::json())
        .and_then(
            |inner_ctx: CasinoContext, user_create: UserCreate| async move {
                tracing::info!("Creating user with email: {} and username: {}", user_create.email, user_create.username);
                with_user_create_params(user_create.clone());
                inner_ctx
                    .process_post_user(&user_create.email, &user_create.username)
                    .await
            },
        )
}

/// Get the routes for the server.
async fn get_app(
    ctx: &mut CasinoContext,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    tracing::info!("building filters!");
    let post_user_filter = post_user_filter(ctx.clone()).await;
    let get_user_filter = get_user_filter(ctx.clone()).await;
    let post_transaction_filter = transaction_post_filter(ctx.clone()).await;  
    let get_transaction_filter = transaction_get_filter(ctx.clone()).await;

    let health = warp::path!("health").map(|| "Hello, world!");

    health
        .or(post_transaction_filter)
        .or(get_user_filter)
        .or(get_transaction_filter)
        .or(post_user_filter)
        .recover(handle_rejection)
        .with(warp::trace::request())
}


/// Run the server.
pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let filter = std::env::var("RUST_LOG").unwrap_or_else(|_| "tracing=info,warp=debug".to_owned());

    // Configure the default `tracing` subscriber.
    // The `fmt` subscriber from the `tracing-subscriber` crate logs `tracing`
    // events to stdout. Other subscribers are available for integrating with
    // distributed tracing systems such as OpenTelemetry.
    tracing_subscriber::fmt()
        // Use the filter we built above to determine which traces to record.
        .with_env_filter(filter)
        // Record an event when each span closes. This can be used to time our
        // routes' durations!
        .with_span_events(FmtSpan::CLOSE)
        .init();
    // We leak the box here on purpose so we can have a reference to it that
    // lives for the lifetime of the program.
    let ctx = Box::leak(Box::new(CasinoContext::default()));
    let app = get_app(ctx).await;
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 3030));

    tracing::info!("Starting server on {:?}", addr);
    warp::serve(app).run(addr).await;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./test_migrations");

    #[sqlx::test(migrator = "MIGRATOR")]
    async fn test_get_transactions(pool: PgPool) -> sqlx::Result<()> {
        let test_uuid = Uuid::nil();
        let ctx = CasinoContext::new(pool.clone());
        let result = ctx.get_transactions(test_uuid).await;
        match result {
            Ok(transactions) => {
                assert_eq!(1, transactions.len());
                assert_eq!(test_uuid, transactions[0].id);
            },
            Err(e) => {
                eprintln!("Error: {:?}", e);
                assert!(false);
            }
        }
        Ok(())
    }

    #[sqlx::test(migrator = "MIGRATOR")]
    async fn test_get_user(pool: PgPool) -> sqlx::Result<()> {
        let test_uuid = Uuid::nil();
        let ctx = CasinoContext::new(pool.clone());
        let result = ctx.get_user(test_uuid).await;
        match result {
            Ok(user) => {
                assert_eq!(1, user.len());
                assert_eq!(test_uuid, user[0].id);
            },
            Err(e) => {
                eprintln!("Error: {:?}", e);
                assert!(false);
            }
        }
        Ok(())
    }

    #[sqlx::test(migrator = "MIGRATOR")]
    async fn test_check_username_email(pool: PgPool) -> sqlx::Result<()> {
        let ctx = CasinoContext::new(pool.clone());
        let result = ctx.check_username_email("testuser", "testemail@test.test").await;
        match result {
            Ok(_) => {
                assert!(true);
            },
            Err(e) => {
                eprintln!("Error: {:?}", e);
                assert!(false);
            }
        }
        Ok(())
    }

    #[sqlx::test(migrator = "MIGRATOR")]
    async fn test_create_user(pool: PgPool) -> sqlx::Result<()> {
        let ctx = CasinoContext::new(pool.clone());
        let result = ctx.create_user("email", "username").await;
        match result {
            Ok(user_id) => {
                assert_eq!(2, user_id.id);
            },
            Err(e) => {
                eprintln!("Error: {:?}", e);
                assert!(false);
            }
        }
        Ok(())
    }

    #[sqlx::test(migrator = "MIGRATOR")]
    async fn test_create_transaction(pool: PgPool) -> sqlx::Result<()> {
        let user_id = Uuid::nil();
        let casino_id = Uuid::nil();
        let transaction_id = Uuid::nil();
        let ctx = CasinoContext::new(pool.clone());
        let result = ctx.create_transaction(user_id, casino_id, 1, 1, None).await;
        match result {
            Ok(transaction) => {
                assert_eq!(transaction_id, transaction.id);
            },
            Err(e) => {
                eprintln!("Error: {:?}", e);
                assert!(false);
            }
        }
        Ok(())
    }

    #[sqlx::test(migrator = "MIGRATOR")]
    async fn test_req_get_user(pool: PgPool) -> sqlx::Result<()> {
        let user_id = Uuid::nil();
        let ctx = CasinoContext::new(pool.clone());
        let req = warp::test::request().method("GET").path(&format!("/user/{}", user_id));
        let res = req.reply(&get_user_filter(ctx).await).await;
        assert_eq!(res.status(), StatusCode::OK);
        Ok(())
    }   

    #[sqlx::test(migrator = "MIGRATOR")]
    async fn test_req_get_transactions(pool: PgPool) -> sqlx::Result<()> {
        let ctx = CasinoContext::new(pool.clone());
        let req = warp::test::request().method("GET").path("/transaction/1");
        let res = req.reply(&transaction_get_filter(ctx).await).await;
        assert_eq!(res.status(), StatusCode::OK);
        Ok(())
    }

    #[sqlx::test(migrator = "MIGRATOR")]
    async fn test_req_post_user(pool: PgPool) -> sqlx::Result<()> {
        let ctx = CasinoContext::new(pool.clone());
        let req = warp::test::request()
            .method("POST")
            .path("/user")
            .json(&UserCreate {
                email: "testemail".to_string(),
                username: "testusername".to_string(),
            });
        let res = req.reply(&post_user_filter(ctx).await).await;
        assert_eq!(res.status(), StatusCode::OK);
        Ok(())
    }

    #[sqlx::test(migrator = "MIGRATOR")]
    async fn test_req_post_transaction(pool: PgPool) -> sqlx::Result<()> {
        let ctx = CasinoContext::new(pool.clone());
        let req = warp::test::request()
            .method("POST")
            .path("/transaction/1/1")
            .json(&TransactionCreate {
                cost: 1,
                benefit: 1,
                notes: None,
            });
        let res = req.reply(&transaction_post_filter(ctx).await).await;
        assert_eq!(res.status(), StatusCode::CREATED);
        Ok(())
    }

    #[sqlx::test(migrator = "MIGRATOR")]
    async fn test_req_health(pool: PgPool) -> sqlx::Result<()> {
        let ctx = Box::leak(Box::new(CasinoContext::new(pool.clone())));
        let req = warp::test::request().method("GET").path("/health");
        let res = req.reply(&get_app(ctx).await).await;
        assert_eq!(res.status(), StatusCode::OK);
        Ok(())
    }
}