use casino_buddy::run;
mod localization;  // Import the localization module

#[tokio::main]
async fn main() {
    // TODO: Add logging
    // TODO: Print some system/config info

    println!("{}", localization::STARTING_SERVER_MESSAGE);
    
    if let Err(e) = run().await {

        eprintln!("{} {:?}", localization::ERROR_MESSAGE, e);
    }
}
