mod assets;
mod entities;
mod map;
mod states;
mod util;

use assets::Assets;
use glutin_window::{GlutinWindow, OpenGL};
use graphics::{Context, Viewport};
use opengl_graphics::GlGraphics;
use piston::{
    Button, EventSettings, Events, PressEvent, ReleaseEvent, RenderEvent, UpdateEvent, Window,
    WindowSettings,
};
use states::{game_state::GameState, state::State};
use winit::window::Fullscreen;

fn main() {
    let mut window: GlutinWindow = WindowSettings::new("Pixelypse 0.1", [1280, 720])
	        .samples(0)
            .graphics_api(OpenGL::V4_5)
            .exit_on_esc(true)
            .decorated(false)
            .build()
            .expect("Critical error while initializing display system");
    
    window.window.set_fullscreen(Some(Fullscreen::Borderless(None))); // I hate it too. I am lazy. You try and make a game in Rust, I'll wait. 

    let assets = Assets::new();
    let mut states: Vec<Box<dyn State>> = vec![Box::new(GameState::new(assets, window.size()))];
    let mut events = Events::new(EventSettings::new());
    let mut gl = GlGraphics::new(OpenGL::V4_5);

    //update input
    while let Some(event) = events.next(&mut window) {
        let top_state_index = states.len() - 1;

        // TODO impl this later
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
