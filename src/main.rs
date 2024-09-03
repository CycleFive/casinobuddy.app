use casino_buddy::run;

#[tokio::main]
async fn main() {
    // TODO: Add logging
    // TODO: Print some system / config info
    println!("Starting Casino Buddy server...");
    if let Err(e) = run().await {
        eprintln!("Error: {:?}", e);
    }
}
