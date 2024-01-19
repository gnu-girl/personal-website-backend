### Rust backend

#### This backend is wirtten in Rust and uses the [Rocket library](https://rocket.rs/) for an API.

#### To run:
1. Make sure you have `cargo` installed
2. Install all the required modules with `cargo build`
3. Run the serve & expose the api with `cargo run --bin gg_rust`

#### Persistence
This app uses a PostgreSQL database for persisting data. The code for interfacing with the database is written in Rust and also contained in this repo.
