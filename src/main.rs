extern crate glutin_window;
extern crate piston;
extern crate graphics;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};

use std::collections::LinkedList;
use std::iter::FromIterator;

#[derive(Clone, PartialEq)]

enum Direction{
    Right, Left, Up, Down
}
struct Game{
    gl: GlGraphics,
    snake: Snake,
}

impl Game {
    fn render(&mut self, arg: &RenderArgs){
        use graphics;

        let PINK: [f32; 4]= [1.0, 0.5, 0.7, 1.0];

        self.gl.draw(arg.viewport(), |c, gl|{
            graphics::clear(PINK, gl);
        });
        self.snake.render(&mut self.gl, arg);
    }

    fn update(&mut self){
        self.snake.update();
    }

    fn pressed(&mut self, btn: &Button){
        let last_direction = self.snake.dir.clone();

        self.snake.dir = match btn { // prevents snake going in reverse direction
            &Button::Keyboard(Key::Up)
                if last_direction != Direction::Down => Direction::Up,
            &Button::Keyboard(Key::Down)
                if last_direction != Direction::Up => Direction::Down,
            &Button::Keyboard(Key::Left)
                if last_direction != Direction::Right => Direction::Left,
            &Button::Keyboard(Key::Right)
                if last_direction != Direction::Left => Direction::Right,
            _ => last_direction
        };
    }
}

struct Snake{
    // old way of doing it
    // x_position: i32,
    // y_position: i32,

    //new way of doing it with linked list
    body: LinkedList<(i32, i32)>,
    dir: Direction, // moodify the snake so it sees the game as a grid
}

impl Snake{
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs){
        use graphics;

        let BLACK: [f32; 4]= [1.0, 0.0, 0.0, 1.0];

        let squares = Vec<graphics::types::Rectangle> = self.body
        .iter()
        .map(|&(x, y)|{
            graphics::rectangle::square(
                (self.x_position * 20) as f64, 
                (self.y_position * 20) as f64, 
                20_f64);
        })
        .collect();

        gl.draw(args.viewport(), |c, gl|{
            let transform = c.transform;
            squares.into_iter()
            .for_each(|square|  graphics::rectangle(BLACK, square, transform, gl));
           
        });
    }
    fn update(&mut self){
        let mut new_head = (*self.body.front().expect("Snake has no body")).clone();

        match self.dir {
            Direction::Down => new_head.1 +=1,
            Direction::Up => new_head.1 -=1,
            Direction::Left => new_head.0 -=1,
            Direction::Right => new_head.0 +=1,
        }
        self.body.pop_front(new_head);
        self.body.pop_back().unwrap();
    }
}

fn main() {
    let opengl = OpenGL::V3_2; // open window
    let mut window: GlutinWindow = WindowSettings::new(
        "Snake Game",
        [200, 200]
    ).opengl(opengl)
    .exit_on_esc(true)
    .build()
    .unwrap();

    let mut game = Game{
        gl: GlGraphics::new(opengl),
        snake: Snake{
            body: LinkedList::from_iter((vec![(0,0), (0,1)]).into_iter()),
            dir: Direction::Right
        }, // initializing snake
    };
    let mut events = Events::new(EventSettings::new()).ups(8);
    while let Some(e) = events.next(&mut window){
        if let Some(r) = e.render_args(){
            game.render(&r);
        }
        if let Some(u) = e.update_args(){
            game.update();
        }
        if let Some(k) = e.button_args(){
            if k.state == ButtonState::Press{
                game.pressed(&k.button);
            }
        }
    }
}
