use std::collections::LinkedList;
use piston_window::{Context, G2d};
use piston_window::types::Color;
use crate::draw::draw_block;
use crate::direction::Direction;

const SNAKE_COLOR: Color = [0.0, 0.8, 0.0, 1.0];
const SNAKE_NB_BLOCKS: i32 = 3;

#[derive(Debug, Clone)]
struct Block {
    x: i32,
    y: i32
}

pub struct Snake {
    direction: Direction,
    body: LinkedList<Block>,
    tail: Option<Block>
}

impl Snake {
    pub fn create(x: i32, y:i32) -> Snake {

        let mut body = LinkedList::new();

        for i in 0..SNAKE_NB_BLOCKS {
            body.push_front(Block{x: x + i, y});
        }

        Snake {
            direction: Direction::RIGHT,
            body,
            tail: None
        }

    }

    pub fn draw(&self, ctx: &Context, g: &mut G2d) {
        for bloc in &self.body {
            draw_block(SNAKE_COLOR, bloc.x, bloc.y, ctx, g);
        }
    }

    pub fn head_position(&self) -> (i32, i32) {
        let head = self.body.front().unwrap();
        (head.x, head.y)
    }

    pub fn head_direction(&self) -> Direction {
        self.direction
    }

    pub fn move_forward(&mut self, direction: Option<Direction>) {
        match direction {
            None => {}
            Some(d) => {
                self.direction = d;
            }
        }

        let (last_x, last_y) = self.head_position();

        let new_bloc = match self.direction {
            Direction::UP => Block {
                x: last_x,
                y: last_y - 1
            },
            Direction::DOWN => Block{
                x: last_x,
                y: last_y + 1
            },
            Direction::LEFT => Block{
                x: last_x -1,
                y: last_y
            },
            Direction::RIGHT => Block{
                x: last_x + 1,
                y: last_y
            }
        };

        self.body.push_front(new_bloc);

        let removed_block = self.body.pop_back().unwrap();

        self.tail = Some(removed_block);
    }

    pub fn next_head(&self, direction: Option<Direction>) -> (i32, i32) {
        let (head_x, head_y) = self.head_position();

        let mut moving_dir = self.direction;

        match direction {
            None => {}
            Some(d) => {
                moving_dir = d;
            }
        };

        match moving_dir {
            Direction::UP => (head_x, head_y - 1),
            Direction::DOWN => (head_x, head_y + 1),
            Direction::LEFT => (head_x - 1 , head_y),
            Direction::RIGHT => (head_x + 1, head_y)
        }
    }

    pub fn restore_tail(&mut self){
        let tail = self.tail.clone().unwrap();
        self.body.push_back(tail);
    }

    pub fn overlap_tail(&self, x: i32, y: i32 ) -> bool {
        let mut ch = 0;

        for block in &self.body {
            if x == block.x && y == block.y {
                return true;
            }
            ch +=1;

            if ch == (self.body.len() -1) {
                break;
            }
        }

        false
    }
}
