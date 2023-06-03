use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use nanorand::{Rng, WyRand};

const SIZE: (u32, u32) = (40, 30);
const TIME: (i32, i32) = (500, 200);

#[derive(PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Snake {
    head: (u32, u32),
    body: Vec<(u32, u32)>,
    food: (u32, u32),
    direction: Direction,
    wait_time: u64,
}

impl Snake {
    fn new() -> Snake {
        Snake {
            head: Snake::gen_pos(),
            body: Vec::new(),
            food: (Snake::gen_pos()),
            direction: Direction::Up,
            wait_time: TIME.0 as u64,
        }
    }
    fn gen_pos() -> (u32, u32) {
        let mut rng = WyRand::new();
        let x = rng.generate_range(0..SIZE.0);
        let y = rng.generate_range(0..SIZE.1);
        (x, y)
    }
    fn spawn_food(&mut self) {
        let mut food;
        loop {
            food = Snake::gen_pos();
            if !self.body.contains(&food) && food != self.head {
                break
            }
        }
        self.food = food;
    }
    fn update_time(&mut self) {
        let time  = ((TIME.1 - TIME.0) * self.body.len() as i32 / (SIZE.0 * SIZE.1) as i32) + TIME.0;
        self.wait_time = time as u64;
    }
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
        let mut direction = None;
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    if game.direction != Direction::Down {
                        direction = Some(Direction::Up);
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    if game.direction != Direction::Up {
                        direction = Some(Direction::Down);
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    if game.direction != Direction::Right {
                        direction = Some(Direction::Left);
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    if game.direction != Direction::Left {
                        direction = Some(Direction::Right);
                    }
                }
                Event::Quit { .. } => return,
                _ => {}
            }
        }
        
        if let Some(value) = direction {
            game.direction = value;
        }
        
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

        if game.body.contains(&game.head) {
            return
        }

        if game.head == game.food {
            game.body.push(game.head);
            game.update_time();
            game.spawn_food();
        }

        canvas.set_draw_color(Color::GRAY);
        canvas.clear();

        canvas.set_draw_color(Color::GREEN);
        canvas.fill_rect(rect_from_pos(game.head)).unwrap();

        for x in &game.body {
            canvas.fill_rect(rect_from_pos(*x)).unwrap();
        }

        canvas.set_draw_color(Color::RED);
        canvas.fill_rect(rect_from_pos(game.food)).unwrap();

        canvas.present();
        std::thread::sleep(Duration::from_millis(game.wait_time));
    }
}

fn rect_from_pos(x: (u32, u32)) -> Rect {
    Rect::new(x.0 as i32 * 20, x.1 as i32 * 20, 20, 20)
}
