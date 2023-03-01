use std::sync::Arc;
mod snake;
mod handlers;


#[tokio::main]
async fn main() {
    let mut snake = snake::Snake::new(50, std::time::Duration::from_secs(1));
    snake.start_game_loop();

    let app = axum::Router::new()
        .route("/snake", axum::routing::get(handlers::get_board))
        .route("/snake/:direction", axum::routing::post(handlers::post_direction))
        .with_state(Arc::new(snake));

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap()
}