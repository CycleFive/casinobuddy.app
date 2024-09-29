use sqlx::SqlitePool;
use std::{convert::Infallible, sync::Arc, fmt::Display};
use warp::{http::StatusCode, reject::Reject, reject::Rejection, reply, Filter, Reply};

/// Custom error type for unauthorized requests.
#[derive(Debug)]
pub struct Unauthorized;

/// Implement the [`Reject`] trait for [`Unauthorized`].
impl Reject for Unauthorized {}

/// Implement the [`Display`] trait for [`Unauthorized`].
impl Display for Unauthorized {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Unauthorized")
    }
}

/// Implement the [`std::error::Error`] trait for [`Unauthorized`].
impl std::error::Error for Unauthorized {}

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

/// DB struct for transactions.
#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize, serde::Deserialize, PartialEq)]
struct Transaction {
    id: i64,
    user_id: i64,
    casino_id: i64,
    cost: i64,
    benefit: i64,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
    notes: Option<String>,
}

/// DB struct for redemptions.
#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize, serde::Deserialize, PartialEq)]
struct Redemption {
    id: i64,
    user_id: i64,
    casino_id: i64,
    amount: i64,
    created_at: chrono::NaiveDateTime,
    received_at: Option<chrono::NaiveDateTime>,
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

/// Struct for the json response body for transactions.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
struct TransactionsReplyBody {
    body: Vec<Transaction>,
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
    // Do I really need to use Arc here? There is already an arc in the Pool...
    db: Arc<SqlitePool>,
}

/// Custom type for a user id.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CBUserId {
    id: i64,
}

/// Implementation for [CasinoContext]
impl CasinoContext {
    /// Create a new instance of `[CasinoContext]` given a database pool.
    fn new(db: SqlitePool) -> Self {
        Self { db: Arc::new(db) }
    }

    /// Get all transactions for a user.
    async fn get_transactions(&self, user_id: i64) -> Result<Vec<Transaction>, sqlx::Error> {
        let transactions = sqlx::query_as!(
            Transaction,
            r#"SELECT * FROM "transaction" WHERE user_id = $1"#,
            user_id
        )
        .fetch_all(&*self.db)
        .await?;
        Ok(transactions)
    }

    /// Get a user by their id.
    async fn get_user(&self, user_id: i64) -> Result<Vec<User>, sqlx::Error> {
        let user = sqlx::query_as!(User, "SELECT * FROM user WHERE id = $1", user_id)
            .fetch_all(&*self.db)
            .await?;
        Ok(user)
    }

    /// Check if a user and/or email already exists.
    async fn check_username_email(&self, email: &str, username: &str) -> Result<(), sqlx::Error> {
        // We are entirely using the Errors to communicate what has happened here.
        // I think this is the idiomatic way to do this in Rust, since they are also
        // error states.
        let matches = sqlx::query!("SELECT COUNT(*) as cnt FROM user WHERE email = $1", email)
            .fetch_one(&*self.db)
            .await?;
        if matches.cnt > 0 {
            return Err(sqlx::Error::RowNotFound);
        }

        let matches = sqlx::query!(
            "SELECT COUNT(*) as cnt FROM user WHERE username = $1",
            username
        )
        .fetch_one(&*self.db)
        .await?;
        if matches.cnt > 0 {
            return Err(sqlx::Error::RowNotFound);
        }
        Ok(())
    }

    /// Create a new user.
    async fn create_user(&self, email: &str, username: &str) -> Result<CBUserId, sqlx::Error> {
        // This shouldn't fail because we don't have any constraints on the email or username.
        // Do we want to constrain these in the database?
        // Should be checking for existing users with the same email or username?
        let user_id = sqlx::query_as!(
            CBUserId,
            "INSERT INTO user (email, username) VALUES ($1, $2) RETURNING id",
            email,
            username
        )
        .fetch_one(&*self.db)
        .await?;
        Ok(user_id)
    }

    /// Create a new transaction, these are the purchases of coins from the casinos.
    async fn create_transaction(
        &self,
        user_id: i64,
        casino_id: i64,
        cost: i64,
        benefit: i64,
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

    /// Create a redemption entry.
    

    /// Process a request to get all transactions for a user.
    async fn process_get_transactions(&self, user_id: i64) -> Result<impl Reply, Rejection> {
        let transactions = self.get_transactions(user_id).await.map_err(Sqlx)?;
        Ok(warp::reply::json(&TransactionsReplyBody { body: transactions }))
    }

    /// Process a request to get a user by their id.
    async fn process_get_user(&self, user_id: i64) -> Result<impl Reply, Rejection> {
        let user = self.get_user(user_id).await.map_err(Sqlx)?;
        Ok(warp::reply::json(&UserReplyBody { body: user }))
    }

    /// Process a request to create a new user.
    async fn process_post_user(
        &self,
        email: &str,
        username: &str,
    ) -> Result<impl Reply, Rejection> {
        self.check_username_email(email, username)
            .await
            .map_err(Sqlx)?;
        let user_id = self.create_user(email, username).await.map_err(Sqlx)?;
        Ok(warp::reply::json(&user_id))
    }

    /// Process a request to create a new transaction.
    async fn process_put_transaction(
        &self,
        user_id: i64,
        casino_id: i64,
        cost: i64,
        benefit: i64,
        notes: &Option<String>,
    ) -> Result<impl Reply, Rejection> {
        let transaction = self.create_transaction(user_id, casino_id, cost, benefit, notes.clone()).await.map_err(Sqlx)?;
        Ok(warp::reply::json(&transaction))
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

    warp::path!("transactions" / i64)
        .and(warp::get())
        .and(context)
        .and_then(|user_id: i64, inner_ctx: CasinoContext| async move {
            inner_ctx.clone().process_get_transactions(user_id).await
        })
        .recover(handle_rejection)
}

/// Get a user by their id.
async fn get_user_filter(
    ctx: CasinoContext,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let context = warp::any().map(move || ctx.clone());

    warp::path!("user" / i64)
        .and(warp::get())
        .and(context)
        .and_then(|user_id: i64, inner_ctx: CasinoContext| async move {
            inner_ctx.clone().process_get_user(user_id).await
        })
        .recover(handle_rejection)
}


/// Struct for the json query body for create adding a transaction.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
struct TransactionCreate {
    user_id: i64,
    casino_id: i64,
    cost: i64,
    benefit: i64,
    notes: Option<String>,
}

/// Filter to the transaction create params.
fn with_transaction_create_params(
    params: TransactionCreate,
) -> impl Filter<Extract = (TransactionCreate,), Error = Infallible> + Clone {
    warp::any().map(move || params.clone())
}

/// Get all transactions for a user.
async fn transaction_put_filter(
    ctx: CasinoContext,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let context = warp::any().map(move || ctx.clone());

    warp::path!("transactions")
        .and(warp::put())
        .and(context)
        .and(warp::body::json())
        .and_then(
            |inner_ctx: CasinoContext, params: TransactionCreate| async move {
                with_transaction_create_params(params.clone());
                inner_ctx
                    .clone()
                    .process_put_transaction(params.user_id, params.casino_id, params.cost, params.benefit, &params.notes)
                    .await
            },
        )
        .recover(handle_rejection)
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
    warp::any().map(move || params.clone())
}

/// Post a new user.
async fn post_user_filter(
    ctx: CasinoContext,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let context = warp::any().map(move || ctx.clone());

    warp::path!("user")
        .and(warp::post())
        .and(context)
        .and(warp::body::json())
        .and_then(
            |inner_ctx: CasinoContext, user_create: UserCreate| async move {
                with_user_create_params(user_create.clone());
                inner_ctx
                    .clone()
                    .process_post_user(&user_create.email, &user_create.username)
                    .await
            },
        )
        .recover(handle_rejection)
}

/// Get the routes for the server.
async fn get_app(
    ctx: CasinoContext,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    println!("building filters!");
    let transaction_filter = transaction_filter(ctx.clone()).await;
    let put_transaction_filter = transaction_put_filter(ctx.clone()).await;
    let post_user_filter = post_user_filter(ctx.clone()).await;
    let get_user_filter = get_user_filter(ctx).await;
    let health = warp::path!("health").map(|| "Hello, world!");
    let log = warp::log("casino-buddy");
    health
        .or(transaction_filter)
        .or(put_transaction_filter)
        .or(post_user_filter)
        .or(get_user_filter)
        .with(log)
}

/// Run the server.
pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let ctx = CasinoContext::default(); //Box::leak(Box::new(VotingContext::new().await));
    let app = get_app(ctx).await;

    println!("Starting server on");
    warp::serve(app).run(([0, 0, 0, 0], 3030)).await;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./test_migrations");

    #[sqlx::test(migrator = "MIGRATOR")]
    async fn test_get_transactions(pool: SqlitePool) -> sqlx::Result<()> {
        let ctx = CasinoContext::new(pool.clone());
        let result = ctx.get_transactions(1).await;
        match result {
            Ok(transactions) => {
                assert_eq!(1, transactions.len());
                assert_eq!(1, transactions[0].id);
            },
            Err(e) => {
                eprintln!("Error: {:?}", e);
                assert!(false);
            }
        }
        Ok(())
    }

    #[sqlx::test(migrator = "MIGRATOR")]
    async fn test_get_user(pool: SqlitePool) -> sqlx::Result<()> {
        let ctx = CasinoContext::new(pool.clone());
        let result = ctx.get_user(1).await;
        match result {
            Ok(user) => {
                assert_eq!(1, user.len());
                assert_eq!(1, user[0].id);
            },
            Err(e) => {
                eprintln!("Error: {:?}", e);
                assert!(false);
            }
        }
        Ok(())
    }

    #[sqlx::test(migrator = "MIGRATOR")]
    async fn test_check_username_email(pool: SqlitePool) -> sqlx::Result<()> {
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
    async fn test_create_user(pool: SqlitePool) -> sqlx::Result<()> {
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
    async fn test_create_transaction(pool: SqlitePool) -> sqlx::Result<()> {
        let ctx = CasinoContext::new(pool.clone());
        let result = ctx.create_transaction(1, 1, 1, 1, None).await;
        match result {
            Ok(transaction) => {
                assert_eq!(2, transaction.id);
            },
            Err(e) => {
                eprintln!("Error: {:?}", e);
                assert!(false);
            }
        }
        Ok(())
    }

}
