use std::convert::Infallible;
use std::str::FromStr;
use warp::{Filter, Rejection, Reply};
use uuid::Uuid;
use bigdecimal::BigDecimal;

use crate::{BadRequest, CasinoContext};


#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct UserCreate {
    pub email: String,
    pub username: String,
}

/// Struct for the json query body for create adding a transaction.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct TransactionCreate {
    pub user_id:    String,
    pub casino_id:  String,
    pub cost:       BigDecimal,
    pub benefit:    BigDecimal,
    pub notes:      Option<String>,
}


/// Get a user by their id.
pub(crate) async fn get_user_filter(
    ctx: CasinoContext,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let context = warp::any().map(move || ctx.clone());

    warp::path!("user" / String)
        .and(warp::get())
        .and(context)
        .and_then( |user_id: String, inner_ctx: CasinoContext| async move {
            let user_id = Uuid::parse_str(&user_id).unwrap();
            tracing::info!("Getting user with id: {}", user_id);
            inner_ctx.process_get_user(user_id).await
        })
}

/// Filter to the transaction create params.
fn with_transaction_create_params(
    params: TransactionCreate,
) -> impl Filter<Extract = (TransactionCreate,), Error = Infallible> + Clone {
    warp::any().map(move || params.clone())
}

/// Filter to get the json body.
pub(crate) fn with_json_body<T: serde::de::DeserializeOwned + Send>(
) -> impl Filter<Extract = (T,), Error = Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

/// Post filter for transactions.
/// `/transaction/{user_id}/{casino_id}`
#[allow(clippy::unused_async)]
pub(crate) async fn transaction_post_filter(
    ctx: CasinoContext,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let context = warp::any().map(move || ctx.clone());

    // POST /transaction/{user_id}/{casino_id}
    warp::path!("transaction" / String / String)
        .and(warp::post())
        .and(with_json_body())
        .and(context)
        .and_then(
            |user_id: String, casino_id: String, params: TransactionCreate, inner_ctx: CasinoContext| async move {
                let user_id: Uuid = Uuid::from_str(&user_id).map_err(|_| BadRequest)?;
                let casino_id: Uuid = Uuid::from_str(&casino_id).map_err(|_| BadRequest)?;
                with_transaction_create_params(params.clone());
                inner_ctx
                    .clone()
                    .process_post_transaction(user_id, casino_id, params.cost, params.benefit, params.notes.as_ref())
                    .await
            },
        )
}

/// Get casino listing
/// `/casino`
pub(crate) async fn casino_list_filter(
    ctx: CasinoContext,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let context = warp::any().map(move || ctx.clone());

    warp::path("casino")
        .and(warp::get())
        .and(context)
        .and_then(|inner_ctx: CasinoContext| async move {
            tracing::info!("Requesting casino listing");
            inner_ctx.process_casino_listing().await
        })
}

/// Get all transactions for a user.
/// `/transaction/{user_id}`
#[allow(clippy::unused_async)]
pub(crate) async fn transaction_get_filter(
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
/// Filter to get the items db.
pub(crate) fn with_user_create_params(
    params: UserCreate,
) -> impl Filter<Extract = (UserCreate,), Error = Infallible> + Clone {
    warp::any().map(move || params.clone())
}

/// Post a new user.
/// `/user POST {'username': 'testuser', 'email': 'testemail'}`
#[allow(clippy::unused_async)]
pub(crate) async fn post_user_filter(
    ctx: CasinoContext,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let context = warp::any().map(move || ctx.clone());

    warp::path("user")
        .and(warp::post())
        .and(warp::path::end())
        .and(context)
        .and(with_json_body())
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

