use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use rand::prelude::{thread_rng, Rng};

const SIZE: (u32, u32) = (40, 30);
const TIME: (i32, i32) = (500, 200);

struct Snake {
    head: (u32, u32),
    body: Vec<(u32, u32)>,
    food: (u32, u32),
    direction: Direction,
    previous_direction: Direction,
}

impl Snake {
    fn new() -> Snake {
        Snake {
            head: Snake::gen_pos(),
            body: Vec::new(),
            food: (Snake::gen_pos()),
            direction: Direction::Up,
            previous_direction: Direction::Down,
        }
    }
    fn gen_pos() -> (u32, u32) {
        let mut rng = thread_rng();
        let x = rng.gen_range(0..SIZE.0 - 1);
        let y = rng.gen_range(0..SIZE.1 - 1);
        (x, y)
    }
    fn spawn_food(&mut self) {
        let mut food;
        loop {
            food = Snake::gen_pos();
            if !self.body.contains(&food) && &food != &self.head {
                break
            }
        }
        self.food = food;
    }
}

#[derive(PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();

    let window = video
        .window("minhoca", SIZE.0 * 20, SIZE.1 * 20)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut game = Snake::new();

    loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    if game.previous_direction != Direction::Down {
                        game.direction = Direction::Up;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    if game.previous_direction != Direction::Up {
                        game.direction = Direction::Down;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    if game.previous_direction != Direction::Right {
                        game.direction = Direction::Left;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    if game.previous_direction != Direction::Left {
                        game.direction = Direction::Right;
                    }
                }
                Event::Quit { .. } => return,
                _ => {}
            }
        }

        game.previous_direction = game.direction;

        if game.body.last().is_some() {
            game.body.pop();
            game.body.insert(0, game.head)
        }

        match game.direction {
            Direction::Up => {
                if game.head.1 == 0 {
                    game.head = (game.head.0, SIZE.1 - 1)
                } else {
                    game.head = (game.head.0, game.head.1 - 1)
                };
            }
            Direction::Down => {
                if game.head.1 == SIZE.1 - 1 {
                    game.head = (game.head.0, 0)
                } else {
                    game.head = (game.head.0, game.head.1 + 1)
                };
            }
            Direction::Left => {
                if game.head.0 == 0 {
                    game.head = (SIZE.0 - 1, game.head.1)
                } else {
                    game.head = (game.head.0 - 1, game.head.1)
                };
            }
            Direction::Right => {
                if game.head.0 == SIZE.0 - 1 {
                    game.head = (0, game.head.1)
                } else {
                    game.head = (game.head.0 + 1, game.head.1)
                };
            }
        }

        for x in &game.body {
            if x == &game.head {
                return;
            }
        }

        if game.head == game.food {
            game.body.push(game.head);
            game.spawn_food()
        }

        canvas.set_draw_color(Color::GRAY);
        canvas.clear();

        canvas.set_draw_color(Color::GREEN);
        canvas.fill_rect(rect_from_pos(game.head)).unwrap();

        canvas.set_draw_color(Color::RED);
        canvas.fill_rect(rect_from_pos(game.food)).unwrap();

        for x in &game.body {
            canvas.set_draw_color(Color::GREEN);
            canvas.fill_rect(rect_from_pos(*x)).unwrap();
        }

        canvas.present();
        let time = ((TIME.1 - TIME.0) * game.body.len() as i32 / (SIZE.0 * SIZE.1) as i32) + TIME.0 ;
        std::thread::sleep(Duration::from_millis(time as u64));
    }
}

fn rect_from_pos(x: (u32, u32)) -> Rect {
    Rect::new(x.0 as i32 * 20, x.1 as i32 * 20, 20, 20)
}
