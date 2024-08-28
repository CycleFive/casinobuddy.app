use tracing::trace;
use warp::Filter;

#[tokio::main]
async fn main() {
    // GET /health => 200 OK with body "Ok"
    let index = warp::path!("/").and(warp::get()).map(|| "Hello, World!");
    let and = warp::path!("health").and(warp::get());
    let health = index
        .and_then(and.map(|| {
            trace!("healthy");
            "Ok".to_string()
        }))
        .with(warp::trace::named("health"));

    warp::serve(health).run(([127, 0, 0, 1], 3030)).await;
}
