use crate::snake::Snake;
use crate::snake::Direction;
use std::sync::Arc;


pub async fn post_direction (
    axum::extract::Path(direction): axum::extract::Path<Direction>,
    axum::extract::State(snake): axum::extract::State<Arc<Snake>>
) {
    snake.add_direction(direction);
}