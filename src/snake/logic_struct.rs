use crate::snake::Direction;
use rand::Rng;
use tokio::sync::oneshot::{Sender, self};
use std::{collections::{ HashSet, HashMap, VecDeque}, sync::{Arc, RwLock}};


struct Inner {
    size: usize,
    interval_duration: std::time::Duration,
    direction_map: HashMap<Direction, i32>,
    snake_position: (VecDeque<(usize, usize)>, Direction),
    food_position: HashSet<(usize, usize)>,
    empty_position: HashSet<(usize, usize)>,
}

impl Inner {
    fn update(&mut self) {
        self.direction_map.insert(-self.snake_position.1, 0);

        let val_sum: i32 = self.direction_map.values().sum();
        if val_sum != 0 {
            let rand_from_sum: f64 = rand::thread_rng().gen_range(0f64..f64::from(val_sum));
            let mut cnt = 0;
            for (key, value) in self.direction_map.iter() {
                if f64::from(cnt + value) >= rand_from_sum {
                    self.snake_position.1 = *key;
                    break;
                }
                cnt += value;
            }
        }
        for value in self.direction_map.values_mut() {
            *value = 0
        }

        let new_head_position: (usize, usize) = match (&self.snake_position.1, self.snake_position.0[0]) {
            (Direction::up, (x, 0)) => (x, self.size - 1),
            (Direction::down, (x, y)) if y == self.size - 1 => (x, 0),
            (Direction::right, (x, y)) if x == self.size - 1 => (0, y),
            (Direction::left, (0, y)) => (self.size - 1, y),

            (Direction::up, (x, y)) => (x, y - 1),
            (Direction::down, (x, y)) => (x, y + 1),
            (Direction::right, (x, y)) => (x + 1, y),
            (Direction::left, (x, y)) => (x - 1, y),
        };

        if self.snake_position.0.contains(&new_head_position) {
            self.reset();
            return;
        }
        if self.food_position.contains(&new_head_position) {
            self.food_position.remove(&new_head_position);
            self.snake_position.0.push_front(new_head_position);

        } else {
            self.snake_position.0.rotate_right(1);
            self.empty_position.insert(self.snake_position.0[0]);
            self.snake_position.0[0] = new_head_position;
        }

        self.empty_position.remove(&new_head_position);

        if rand::thread_rng().gen_range(0..5) == 2 {
            let new_position = *Vec::from_iter(
                self.empty_position.iter()
            )[rand::thread_rng().gen_range(0..self.empty_position.len())];

            self.empty_position.remove(&new_position);
            self.food_position.insert(new_position);
        }
    }

    fn reset(&mut self) {
        self.food_position.clear();
        self.snake_position.0.clear();

        for position in Snake::STARTING_POSITION {
            self.snake_position.0.push_front(*position);
        }

        for i in 0..self.size { 
            for j in 0..self.size { 
                self.empty_position.insert((i, j)); 
            } 
        }

        for position in Snake::STARTING_POSITION {
            self.empty_position.remove(position);
        }

        for value in self.direction_map.values_mut() {
            *value = 0
        }

        self.snake_position.1 = Direction::right;
    }
}

pub struct Snake {
    inner: Arc<RwLock<Inner>>,
    game_loop_killer: Option<Sender<()>>,
    board_layout: String,
}

impl Snake {
    const STARTING_POSITION: &'static [(usize, usize)] = &[(0, 0), (1, 0), (2, 0)];

    pub fn new(size: usize, interval_duration: std::time::Duration) -> Snake {
        let mut snake_position_vec = VecDeque::<(usize, usize)>::with_capacity(size * size);
        for position in Snake::STARTING_POSITION {
            snake_position_vec.push_front(*position);
        }
        
        let mut empty_position = HashSet::<(usize, usize)>::with_capacity(size * size);
        for i in 0..size { 
            for j in 0..size { 
                empty_position.insert((i, j)); 
            } 
        };
        for position in Snake::STARTING_POSITION {
            empty_position.remove(position);
        }

        let mut direction_map = HashMap::with_capacity(4);
        direction_map.insert(Direction::down, 0);
        direction_map.insert(Direction::up, 0);
        direction_map.insert(Direction::left, 0);
        direction_map.insert(Direction::right, 0);

        let mut board_layout: String = "-".repeat((size + 1) * size);
        for ind in 0..size {
            let str_ind = (size + 1) * ind + size;
            board_layout.replace_range(str_ind..str_ind + 1, "\n");
        }

        Snake { 
            inner: Arc::new(RwLock::new(
                Inner {
                    size,
                    interval_duration,
                    snake_position: (snake_position_vec, Direction::right),
                    food_position: std::collections::HashSet::<(usize, usize)>::new(),
                    empty_position,
                    direction_map,
                },
            )),
            game_loop_killer: None,
            board_layout,
        }
    }

    pub fn add_direction(&self, direction: Direction) {
        self.inner.write().unwrap()
            .direction_map.entry(direction)
            .and_modify(|val| *val += 1);
    }

    pub fn get_board(&self) -> String {
        let this = self.inner.read().unwrap();

        let mut board = self.board_layout.clone();

        for (x, y) in &this.snake_position.0 {
            let str_ind = y * (this.size + 1) + x;
            board.replace_range(str_ind..=str_ind, "O");
        }

        for (x, y) in &this.food_position {
            let str_ind = y * (this.size + 1) + x;
            board.replace_range(str_ind..=str_ind, "&");
        }
        board
    }

    pub fn start_game_loop(&mut self) {
        let (sender, mut receiver) = oneshot::channel::<()>();
        self.game_loop_killer = Some(sender);

        let this = self.inner.clone();

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(this.read().unwrap().interval_duration);
            loop {
                tokio::select! {
                    _ = interval.tick() => this.write().unwrap().update(),
                    _ = &mut receiver => break,
                }
            }
        });
    }
}