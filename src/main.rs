mod assets;
mod entities;
mod map;
mod states;
mod util;

use std::time::Instant;

use sfml::{graphics::{Color, RenderTarget, RenderWindow}, system::Vector2f, window::{ContextSettings, Style, VideoMode}};

use assets::Assets;
use states::{game_state::GameState, state::State};

const WINDOW_WIDTH: u32 = 1200;
const WINDOW_HEIGHT: u32 = 800;

pub fn main() {

    let mut window = RenderWindow::new(
        VideoMode::new(WINDOW_WIDTH,  WINDOW_HEIGHT,  16),
        "Pixelypse", 
        Style::DEFAULT, 
        &ContextSettings::default()
    );
    
    let assets = Assets::new();

    let mut states: Vec<Box<dyn State>> = vec![Box::new(GameState::new(
        assets.clone(), 
        Vector2f::new(
            WINDOW_WIDTH as f32, 
            WINDOW_HEIGHT as f32
        )
    ))];

    let mut tick_time = Instant::now();

    while window.is_open() {
        if let Some(event) = window.poll_event() {
            match event {
                sfml::window::Event::Closed => window.close(),
                sfml::window::Event::Resized { width, height } => {},
                sfml::window::Event::LostFocus => {},
                sfml::window::Event::GainedFocus => {},
                sfml::window::Event::KeyPressed { code, .. } => states.first_mut().unwrap().keypress_event(code),
                sfml::window::Event::KeyReleased { code, .. } => states.first_mut().unwrap().keyrelease_event(code),
                sfml::window::Event::MouseWheelScrolled { wheel, delta, x, y } => {},
                sfml::window::Event::MouseButtonPressed { button, x, y } => {},
                sfml::window::Event::MouseButtonReleased { button, x, y } => {},
                sfml::window::Event::MouseMoved { x, y } => {},
                sfml::window::Event::MouseEntered => {},
                sfml::window::Event::MouseLeft => {},
                _ => {},
            }
        }
        
        // update
        if tick_time.elapsed().as_millis() >= 10 {
            tick_time = Instant::now();
            states
                .first_mut()
                .expect("No states!")
                .update();
        }

        // render
        window.clear(Color::BLACK);
        states.first_mut().expect("No states!").render(&mut window);
        window.display();

    }
}
