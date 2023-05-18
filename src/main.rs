use std::net::TcpListener;
use zero2prod::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let address = TcpListener::bind("0.0.0.0:0")?;
    run(address)?.await

}
