use casino_buddy::run;

/// Main function
#[tokio::main]
async fn main() {
    println!("Starting server");
    let _ = run().await;
}
