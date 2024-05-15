use poem::{get, handler, listener::TcpListener, Route, Server};

#[handler]
fn hello() -> String {
    format!("hello from fly.io!")
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new().at("/", get(hello));
    Server::new(TcpListener::bind("0.0.0.0:8080"))
        .run(app)
        .await
}