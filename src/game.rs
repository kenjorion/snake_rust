use piston_window::*;
use piston_window::types::Color;
use std::collections::LinkedList;

use rand::{thread_rng, Rng};

use crate::snake::{Direction, Snake};
use crate::draw::{draw_block, draw_rectangle};
/// Global constants for the Colors in RGBA

const FOOD_COLOR: Color = [0.50, 0.00, 0.50, 1.0];
const OBSTACLE_COLOR: Color = [0.50, 0.50, 0.50, 1.0];
const BORDER_COLOR: Color = [0.50, 0.50, 0.50, 1.0];
const GAMEOVER_COLOR: Color = [0.90, 0.00, 0.00, 0.5];
//0.80, 0.00, 0.00, 1.0
//color: [1.0, 0.5, 0.25, 1.0],
const MOVING_PERIOD: f64 = 0.1;
const RESTART_TIME: f64 = 2.0;
//#[derive(Copy, Clone, PartialEq)]
/// A Game struct is represented here
  /// A game struct 
    /// # items
    /// * `snake` - Snake 
    /// * `food_exists` - Bool
    /// * `food_x` - i32
    /// * `food_y` - i32 
    /// * `width` - i32
    /// * `height` - i32
    /// * `obst_exists` - Bool 
    /// * `obsts` - LinkedList<Obstacle>() (the number of elements of this list increases with the score )
    /// * `score` - i32 (number of food eaten by the snake during the game)
    /// * `game_over` - Bool
    /// * `waiting_time` - f64 (time that user must await before restart a new game)
    
pub struct Game {
    snake: Snake,
    food_exists: bool,
    food_x: i32,
    food_y: i32,
    obsts: LinkedList<Obstacle>,
    obst_exists: bool,
    score: i32,
    width: i32,
    height: i32,
    game_over: bool,
    waiting_time: f64,
}

impl Game {
    /// Returns a game with this arguments 
    ///
    /// # Arguments
    /// use doc::Snake;
    /// *  `snake` - Snake::new(2, 2);
    /// * `waiting-time` - f64 = 0.0
    /// * `food_exist` - Bool = True 
    /// * `food_x` -  i32 = 6 
    /// * `food_y` - i32 = 4 
    /// * `obst_exists` - A Bool = True
    /// * `obsts` - A LinkedList::new(); 
    /// * `score` - i32 = 0
    /// * `width` - i32
    /// * `height` - i32
    /// * `game_over` -  Bool = false 
    pub fn new(width: i32, height: i32) -> Game {
        let  obsts: LinkedList<Obstacle> = LinkedList::new();
        Game {
            snake: Snake::new(2, 2),
            waiting_time: 0.0,
            food_exists: true,
            food_x: 6,
            food_y: 4,
            obsts,
            obst_exists: true,   
            score: 0,
            width,
            height,
            game_over: false
        }
    }
 /// key_pressed : update the snake position depending on pressed keyboard key

    pub fn key_pressed(&mut self, key: Key) {
        if self.game_over {
            return;
        }

        let dir = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            _ => Some(self.snake.head_direction())
        };

        if dir.unwrap() == self.snake.head_direction().opposite() {
            return;
        }

        self.update_snake(dir);
    }
/// draw : draw the snake, food, Obstacle
    ///  use doc::Snake;
    ///  draw(con, g);
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        self.snake.draw(con, g);

        if self.food_exists {
            draw_block(FOOD_COLOR, self.food_x, self.food_y, con, g);
        }
        
        if self.obst_exists{
            for e in &self.obsts {
                 draw_block(OBSTACLE_COLOR, e.x , e.y , con, g)
            }   
            
        }
        draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, self.height - 1, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, con, g);
        draw_rectangle(BORDER_COLOR, self.width - 1, 0, 1, self.height, con, g);

        if self.game_over {
            draw_rectangle(GAMEOVER_COLOR, 0, 0, self.width, self.height, con, g);
           
        } 
    }
    /// update : update the game by:
    /// * `restart` if game-over
    /// * `add_food` if it doesn't existe 
    /// * `update_snake` if `waiting_time` >   `MOVING_PERIOD`
    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;

        if self.game_over {
            if self.waiting_time > RESTART_TIME {
                self.restart();
            }
            return;
        }

        if !self.food_exists {
            self.add_food();
        }


        if self.waiting_time > MOVING_PERIOD {
            self.update_snake(None);
        }
    }

    /// add_obst it  add an obstacle to obsts: LinkedList<Obstacle>

    pub fn add_obst(&mut self){
       
            let mut rng = thread_rng();
            let mut new_x = rng.gen_range(1, self.width - 1);
            let mut new_y = rng.gen_range(1, self.height - 1);
            while self.snake.overlap_tail(new_x, new_y) {
                new_x = rng.gen_range(1, self.width - 1);
                new_y = rng.gen_range(1, self.height - 1);}
            self.obsts.push_back(Obstacle{
                x: new_x,
                y: new_y
            });
       
    }


    
    /// check_eating change the variable `food_exist` to false and let snake eat this bloc
    ///  use doc::Snake;
    ///  restore_tail();

    fn check_eating(&mut self) {
        let (head_x, head_y): (i32, i32) = self.snake.head_position();
        if self.food_exists && self.food_x == head_x && self.food_y == head_y {
            self.food_exists = false;
            self.score += 1;
            self.snake.restore_tail();
              
            self.add_obst( );
        }
    }

    /// check_obst check if the the head of snake is in the obstacle
    pub fn check_obst(&self, x: i32, y: i32)-> bool{
         for e in &self.obsts {
                 if e.x == x && e.y == y {
                     return true;
                 }
            }
            return false;   
    }
 /// check if snake is alive and retourne a Bool
    fn check_if_snake_alive(&mut self, dir: Option<Direction>) -> bool {
        let (next_x, next_y) = self.snake.next_head(dir);

        if self.snake.overlap_tail(next_x, next_y) {
            return false;
        }

        if self.check_obst(next_x, next_y){
            return false;
        }

        next_x > 0 && next_y > 0 && next_x < self.width - 1 && next_y < self.height - 1
    }
     /// add a new bloc of food in the gamethread::spawn(move || {
    /// coordiante of food are generated with randum value
    /// change the variable `food_exists` to true
    fn add_food(&mut self) {
        let mut rng = thread_rng();

        let mut new_x = rng.gen_range(1, self.width - 1);
        let mut new_y = rng.gen_range(1, self.height - 1);
        while self.snake.overlap_tail(new_x, new_y) {
            new_x = rng.gen_range(1, self.width - 1);
            new_y = rng.gen_range(1, self.height - 1);
        }

        self.food_x = new_x;
        self.food_y = new_y;
        self.food_exists = true;
    }

   

     /// update snake  
    /// check if snake is alive

    fn update_snake(&mut self, dir: Option<Direction>) {
        if self.check_if_snake_alive(dir) {
            self.snake.move_forward(dir);
            self.check_eating();
        } else {
            self.game_over = true;
             println!("Congratulations, your score was: {}", self.score);
        }
        self.waiting_time = 0.0;
    }

 /// restart the game with 
    ///  /// Returns a game with this arguments 
    ///
    /// # Arguments
    /// use doc::Person;
    /// * let snake = Snake::new(2, 2);
    /// * `waiting-time` - A f64 = 0.0
    /// * `food_exist` - A Bool = True 
    /// * `obst_exists` - A Bool = True
    /// * `obsts` - A LinkedList::new(); 
    /// * `food_x` - A i32 = 6 
    /// * `food_y` - A i32 = 4 
    /// * `game_over` - A Bool = false 
    fn restart(&mut self) {
        self.snake = Snake::new(2, 2);
        self.waiting_time = 0.0;
        self.food_exists = true;
        self.food_x = 6;
        self.food_y = 4;
        self.obsts = LinkedList::new();
        self.obst_exists = true;
        self.score = 0;
        self.game_over = false;
    }
}
#[derive(Debug)]
/// Obstacle struct its the block Obstacle whitch meet the snake on the game
/// # items
/// * x - i32
/// * y - i32

pub struct Obstacle {
    x: i32,
    y: i32
}