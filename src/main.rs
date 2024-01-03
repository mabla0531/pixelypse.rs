mod display;
mod states;
mod assets;
mod map;
mod entities;

use std::time::Instant;

use assets::Assets;
use sfml::system::Vector2f;
use states::state::State;
use states::game_state::GameState;

use sfml::window::*;
use sfml::graphics::*;
use sfml::window::mouse::*;

struct System {
    window: RenderWindow,
    states: Vec<Box<dyn State>>,
}

impl System {
    fn new() -> Self {
        let assets = Assets::new();

        let window = RenderWindow::new(
            VideoMode::new(
                800, 
                600, 
                16
            ), 
            "Pixelypse", 
            Style::DEFAULT, 
            &ContextSettings::default()
        );

        let mut states = Vec::<Box::<dyn State>>::new();
        let game_state = GameState::new(&assets, window.size());

        states.push(Box::new(game_state));

        System {
            window,
            states,
        }
    }

    fn update(&mut self) {
        let top_state_index = self.states.len() - 1;
        while let Some(event) = self.window.poll_event() {
            match event {
                Event::Closed => self.window.close(),
                Event::MouseButtonPressed { button: b, x: _, y: _ } => self.states[top_state_index].mouse_press_event(b),
                Event::MouseButtonReleased { button: b, x: _, y: _ } => self.states[top_state_index].mouse_release_event(b),
                Event::MouseMoved { x: x_move, y: y_move } => self.states[top_state_index].mouse_position_event(Vector2f::new(x_move as f32, y_move as f32)),
                _ => {println!("{:?}", event)},
            }
        }

        self.states[top_state_index].update();
    }

    fn render(&mut self) {
        let top_state_index = self.states.len() - 1;

        self.window.clear(Color::BLACK);
        self.states[top_state_index].render(&mut self.window);
        self.window.display();
    }

    fn run(&mut self) {

        let mut tick_time = Instant::now();
        
        while self.window.is_open() {
            if tick_time.elapsed().as_millis() >= 10 {
                tick_time = Instant::now();

                self.update();
            }

            self.render();
        }
    }
}

fn main() {
    System::new().run()
}