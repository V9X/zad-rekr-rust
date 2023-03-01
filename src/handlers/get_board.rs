use crate::snake::Snake;
use std::sync::Arc;


pub async fn get_board(
    axum::extract::State(snake): axum::extract::State<Arc<Snake>>
) -> String {
   snake.get_board()
}
