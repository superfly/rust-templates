use warp::Filter;

#[tokio::main]
async fn main() {
    let hello = warp::path::end().map(|| "Hello from fly.io");

    warp::serve(hello)
        .run(([0, 0, 0, 0], 8080))
        .await;
}