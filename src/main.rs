mod display;
mod states;
mod assets;
mod map;
mod entities;

use std::time::Instant;

use assets::Assets;
use states::state::State;
use states::game_state::GameState;

use sfml::window::*;
use sfml::graphics::*;

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
                Event::KeyPressed {code: Key::W, scan: _, alt: _, ctrl: _, shift: _, system: _} => self.states[top_state_index].keypress_event(Key::W),
                Event::KeyPressed {code: Key::A, scan: _, alt: _, ctrl: _, shift: _, system: _} => self.states[top_state_index].keypress_event(Key::A),
                Event::KeyPressed {code: Key::S, scan: _, alt: _, ctrl: _, shift: _, system: _} => self.states[top_state_index].keypress_event(Key::S),
                Event::KeyPressed {code: Key::D, scan: _, alt: _, ctrl: _, shift: _, system: _} => self.states[top_state_index].keypress_event(Key::D),
                
                Event::KeyReleased {code: Key::W, alt: _, ctrl: _, shift: _, system: _} => self.states[top_state_index].keyrelease_event(Key::W),
                Event::KeyReleased {code: Key::A, alt: _, ctrl: _, shift: _, system: _} => self.states[top_state_index].keyrelease_event(Key::A),
                Event::KeyReleased {code: Key::S, alt: _, ctrl: _, shift: _, system: _} => self.states[top_state_index].keyrelease_event(Key::S),
                Event::KeyReleased {code: Key::D, alt: _, ctrl: _, shift: _, system: _} => self.states[top_state_index].keyrelease_event(Key::D),
                
                _ => println!("{:?}", event),
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