use axum::Router;
use dioxus::prelude::*;
use dioxus_liveview::LiveviewRouter;

fn app() -> Element {
    let mut num = use_signal(|| 0);

    rsx! {
        div {
            "hello axum! {num}"
            button { onclick: move |_| num += 1, "Increment" }
        }
    }
}

#[tokio::main]
async fn main() {
    let addr: std::net::SocketAddr = ([0, 0, 0 ,0], 8080).into();

    let app = Router::new().with_app("/", app);

    println!("Listening on http://{addr}");

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}