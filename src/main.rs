use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateEvent};
use piston::window::WindowSettings;
use piston::{Button, EventLoop, MouseCursorEvent, PressEvent, ReleaseEvent};

use std::time::Duration;

const WINDOW_SIZE: usize = 512;
const GRID_RES: usize = 32;

const RECT_SIZE: f64 = (WINDOW_SIZE / GRID_RES) as f64;

const NEIGHBOURS: [(i8, i8); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

pub struct App {
    gl: GlGraphics,
    paused: bool,
    grid: [[bool; GRID_RES]; GRID_RES],
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        self.gl.draw(args.viewport(), |c: Context, gl| {
            clear(BLACK, gl);

            for x in 0..GRID_RES {
                for y in 0..GRID_RES {
                    let rect_size_minus_border: f64 = RECT_SIZE - 1.0;

                    let transform: [[f64; 3]; 2] = c.transform.trans(
                        x as f64 * rect_size_minus_border,
                        y as f64 * rect_size_minus_border,
                    );

                    rectangle(
                        if self.grid[x][y] { WHITE } else { BLACK },
                        rectangle::square(x as f64, y as f64, RECT_SIZE as f64),
                        transform,
                        gl,
                    );
                }
            }
        });
    }

    fn update(&mut self) {
        let mut newgrid: [[bool; GRID_RES]; GRID_RES] = self.grid;

        for x in 0..GRID_RES {
            for y in 0..GRID_RES {
                if self.grid[x][y] == false {
                    if count_neighbours(self.grid, x, y) == 3 {
                        newgrid[x][y] = true;
                    }
                } else {
                    if count_neighbours(self.grid, x, y) < 2 {
                        newgrid[x][y] = false;
                    } else if count_neighbours(self.grid, x, y) > 3 {
                        newgrid[x][y] = false;
                    }
                }
            }
        }

        self.grid = newgrid;
    }
}

fn main() {
    let opengl: OpenGL = OpenGL::V4_5;

    let mut update_interval: f64 = 0.5;

    let mut draw: bool = false;
    let mut erase: bool = false;

    let mut elapsed_time: Duration = Duration::from_millis(0);

    let mut window: Window =
        WindowSettings::new("Game of Life", [WINDOW_SIZE as f64, WINDOW_SIZE as f64])
            .graphics_api(opengl)
            .exit_on_esc(true)
            .resizable(false)
            .build()
            .unwrap();

    let mut app: App = App {
        gl: GlGraphics::new(opengl),
        paused: false,
        grid: [[false; GRID_RES]; GRID_RES],
    };

    let mut events: Events = Events::new(EventSettings::new().max_fps(60));
    while let Some(e) = events.next(&mut window) {
        if let Some(button) = e.press_args() {
            match button {
                Button::Keyboard(piston::Key::Space) => app.paused = !app.paused,
                Button::Keyboard(piston::Key::C) => app.grid = [[false; GRID_RES]; GRID_RES],
                Button::Mouse(piston::MouseButton::Left) => {
                    if !erase {
                        draw = true
                    }
                }
                Button::Mouse(piston::MouseButton::Right) => {
                    if !draw {
                        erase = true
                    }
                }
                Button::Keyboard(piston::Key::LShift) => update_interval = 0.1,
                _ => (),
            }
        };

        if let Some(button) = e.release_args() {
            match button {
                Button::Mouse(piston::MouseButton::Left) => draw = false,
                Button::Mouse(piston::MouseButton::Right) => erase = false,
                Button::Keyboard(piston::Key::LShift) => update_interval = 0.5,
                _ => (),
            }
        }

        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if draw || erase {
            if let Some(pos) = e.mouse_cursor_args() {
                let (x, y) = (pos[0], pos[1]);

                if (x > 0.0) && (y > 0.0) && (x < WINDOW_SIZE as f64) && (y < WINDOW_SIZE as f64) {
                    let squarex: i32 = x as i32 / RECT_SIZE as i32;
                    let squarey: i32 = y as i32 / RECT_SIZE as i32;

                    app.grid[squarex as usize][squarey as usize] = draw;
                }
            }
        }

        if !app.paused {
            if let Some(args) = e.update_args() {
                elapsed_time += Duration::from_secs_f64(args.dt);
                if elapsed_time.as_secs_f64() >= update_interval {
                    app.update();
                    elapsed_time -= Duration::from_secs_f64(update_interval)
                }
            }
        }
    }
}

fn count_neighbours(grid: [[bool; GRID_RES]; GRID_RES], x: usize, y: usize) -> u8 {
    let mut count: u8 = 0;

    for &(dir_x, dir_y) in &NEIGHBOURS {
        let neighbour_x: usize = x + dir_x as usize;
        let neighbour_y: usize = y + dir_y as usize;

        if (neighbour_x < GRID_RES) && (neighbour_y < GRID_RES) && (grid[neighbour_x][neighbour_y])
        {
            count += 1;
        }
    }

    count
}
