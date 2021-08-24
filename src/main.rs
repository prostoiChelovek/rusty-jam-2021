#![deny(unsafe_code)]

extern crate config;
extern crate serde;

#[macro_use]
extern crate serde_derive;

mod settings;
mod resource_helper;

use rg3d::engine::resource_manager::TextureImportOptions;
use rg3d::gui::message::ProgressBarMessage;
use rg3d::gui::{HorizontalAlignment, VerticalAlignment};
use rg3d::resource::texture::CompressionOptions;
use rg3d::sound::context::SoundContext;
use rg3d::utils::log::{Log, MessageKind};
use rg3d::{
    core::{
        pool::Handle,
    },
    dpi::LogicalSize,
    engine::Engine,
    event::{DeviceEvent, Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    gui::{
        message::{UiMessage},
        node::{StubNode, UINode},
        UserInterface,
    },
};
use config::Config;
use once_cell::sync::Lazy;
use std::{
    fs::File,
    io::Write,
    path::Path,
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex, RwLock,
    },
    time::{self, Instant},
};
use settings::Settings;

const FIXED_FPS: f32 = 60.0;
const FIXED_TIMESTEP: f32 = 1.0 / FIXED_FPS;

static SETTINGS: Lazy<RwLock<Settings>> = Lazy::new(|| {
    RwLock::new(Settings::new().unwrap())
});

// Define type aliases for engine structs.
pub type MyEventLoop = EventLoop<()>;
pub type GameEngine = Engine<(), StubNode>;
pub type Gui = UserInterface<(), StubNode>;

pub struct Game {
    engine: GameEngine,
    last_tick_time: time::Instant,
    running: bool,
    time: GameTime,
}

#[derive(Copy, Clone)]
pub struct GameTime {
    clock: time::Instant,
    elapsed: f64,
    delta: f32,
}

impl Game {
    pub fn new(event_loop: &MyEventLoop, title: &'static str) -> Self {
        let inner_size = get_inner_size(event_loop);

        let window_builder = rg3d::window::WindowBuilder::new()
            .with_title(title)
            .with_inner_size(inner_size)
            .with_resizable(true);

        let engine = GameEngine::new(window_builder, event_loop, false).unwrap();

        let time = GameTime {
            clock: Instant::now(),
            elapsed: 0.0,
            delta: FIXED_TIMESTEP,
        };

        Self {
            running: true,
            engine,
            last_tick_time: time::Instant::now(),
            time,
        }
    }

    pub fn run(mut game: Self, event_loop: MyEventLoop) {
        event_loop.run(move |event, _, control_flow| {
            game.process_input_event(&event);

            match event {
                Event::MainEventsCleared => {
                    let mut dt = game.time.clock.elapsed().as_secs_f64() - game.time.elapsed;
                    while dt >= FIXED_TIMESTEP as f64 {
                        dt -= FIXED_TIMESTEP as f64;
                        game.time.elapsed += FIXED_TIMESTEP as f64;

                        game.update(game.time);

                        while let Some(ui_event) = game.engine.user_interface.poll_message() {
                        }
                    }
                    if !game.running {
                        *control_flow = ControlFlow::Exit;
                    }

                    game.engine.get_window().request_redraw();
                }
                Event::RedrawRequested(_) => {
                    // Render at max speed
                    game.engine.render().unwrap();
                }
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit
                    }
                    WindowEvent::Resized(new_size) => {
                        game.engine
                            .renderer
                            .set_frame_size(new_size.into())
                            .unwrap();
                    }
                    _ => (),
                },
                Event::LoopDestroyed => {
                    if let Ok(profiling_results) = rg3d::core::profiler::print() {
                        if let Ok(mut file) = File::create("profiling.log") {
                            let _ = writeln!(file, "{}", profiling_results);
                        }
                    }
                }
                _ => *control_flow = ControlFlow::Poll,
            }
        });
    }

    pub fn update(&mut self, time: GameTime) {
        let window = self.engine.get_window();
        window.set_cursor_visible(false);
        window.set_cursor_grab(true).unwrap();
    }

    fn process_input_event(&mut self, event: &Event<()>) {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(key_code) = input.virtual_keycode {
                        match key_code {
                            _ => (),
                        }
                    }
                }
                &WindowEvent::MouseInput { button, state, .. } => {}
                _ => (),
            },
            Event::DeviceEvent { event, .. } => {
                if let DeviceEvent::MouseMotion { delta } = event {}
            }
            _ => (),
        }
    }
}

fn get_inner_size(event_loop: &MyEventLoop) -> LogicalSize<f32> {
    let primary_monitor = event_loop.primary_monitor().unwrap();
    let mut monitor_dimensions = primary_monitor.size();
    monitor_dimensions.height = (monitor_dimensions.height as f32 * 0.7) as u32;
    monitor_dimensions.width = (monitor_dimensions.width as f32 * 0.7) as u32;
    monitor_dimensions.to_logical::<f32>(primary_monitor.scale_factor())
}


fn main() {
    let event_loop = MyEventLoop::new();
    let game = Game::new(&event_loop, "Jam");
    Game::run(game, event_loop);
}
