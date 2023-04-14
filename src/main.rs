extern crate glutin_window;
extern crate piston;
extern crate graphics;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};

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
}

struct Snake{
    x_position: i32,
    y_position: i32,
}

impl Snake{
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs){
        use graphics;

        let BLACK: [f32; 4]= [0.0, 0.0, 0.0, 1.0];

        let square = graphics::rectangle::square(self.x_position
            as f64, self.y_position as f64, 20_f64);

        gl.draw(args.viewport(), |c, gl|{
            let transform = c.transform;
            graphics::rectangle(BLACK, square, transform, gl);
        });
    }
}

fn main() {
    let opengl = OpenGL::V3_2; // open window
    let mut window: GlutinWindow = WindowSettings::new(
        "Snake Game",
        [200, 200]
    ).opengl(opengl).exit_on_esc(true).build().unwrap();

    let mut game = Game{
        gl: GlGraphics::new(opengl),
        snake: Snake{x_position:50, y_position:100}, // initializing snake
    };
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window){
        if let Some(r) = e.render_args(){
            game.render(&r);
        }
    }
}
