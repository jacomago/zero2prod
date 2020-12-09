use std::net::TcpListener;

use zero2prod::run;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("localhost:8000").expect("Error binding address");
    run(listener)?.await
}
