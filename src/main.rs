mod states;
mod assets;
mod map;
mod entities;
mod util;

use assets::Assets;
use glutin_window::{GlutinWindow, OpenGL};
use graphics::Context;
use opengl_graphics::GlGraphics;
use piston::{Button, EventSettings, Events, PressEvent, ReleaseEvent, RenderEvent, UpdateEvent, Window, WindowSettings};
use states::{game_state::GameState, state::State};

const WINDOW_WIDTH : u32 = 960;
const WINDOW_HEIGHT: u32 = 540;
const TILE_SIZE: usize = 32;

fn main() {
    let mut window: GlutinWindow = WindowSettings::new("Hello Piston!", [WINDOW_WIDTH, WINDOW_HEIGHT])
    .exit_on_esc(true)
    .build()
    .expect("Critical error while initializing display system");

    let assets = Assets::new();

    let mut states = Vec::<Box::<dyn State>>::new();
    let game_state = GameState::new(assets, window.size());

    states.push(Box::new(game_state));
    
    let mut events = Events::new(EventSettings::new());

    let opengl = OpenGL::V4_5;
    let mut gl = GlGraphics::new(opengl);

    //update input
    while let Some(event) = events.next(&mut window) {
        let top_state_index = states.len() - 1;
        
        // TODO fix this later
        // if {   
        //     states[top_state_index].mouse_release_event(b);
        // }
        // if {
        //     states[top_state_index].mouse_press_event(b);
        // }
        // if {
        //     states[top_state_index].mouse_position_event(Vector2f::new(x_move as f32, y_move as f32));
        // }
        if let Some(key) = event.press_args() {
            if let Button::Keyboard(key) = key {
                states[top_state_index].keypress_event(key);
            }
        }
        if let Some(key) = event.release_args() {
            if let Button::Keyboard(key) = key {
                states[top_state_index].keyrelease_event(key);
            }
        }

        //update game
        if let Some(_) = event.update_args() {
            states[top_state_index].update();
        }

        //render game
        if let Some(r) = event.render_args() {
            gl.draw(r.viewport(), |c: Context, g: &mut GlGraphics| {
                graphics::clear([0.0, 0.0, 0.0, 1.0], g);

                states[top_state_index].render(c, g);
            });
        }
    }
}
