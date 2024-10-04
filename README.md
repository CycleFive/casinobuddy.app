# casinobuddy.app
Casino Buddy is an application to track, summarize and aid in responsible online gaming.

## Features
- Track deposits and withdrawals.
- Daily bonus claim tracking.
- Summary of gains and losses over time.

## Dev Notes
- Remember to explicitly log errors in the rejection handler,
    otherwise they get masked by the custom types which hide the 
    underlying error from the user.

## Technologies
- TODO

## Installation
- TODO

## Usage
- TODO

## Building
### Prerequisites
- rustc / cargo
### On Linux
```bash
source ./.env.example
cargo build --release
```
./target/release/casinobuddy
```
