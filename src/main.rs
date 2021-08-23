#![deny(unsafe_code)]

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
    engine::Engine,
    event::{DeviceEvent, Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    gui::{
        message::{UiMessage},
        node::{StubNode, UINode},
        UserInterface,
    },
};
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

const FIXED_FPS: f32 = 60.0;

// Define type aliases for engine structs.
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
    pub fn run() {
        let events_loop = EventLoop::<()>::new();

        let primary_monitor = events_loop.primary_monitor().unwrap();
        let mut monitor_dimensions = primary_monitor.size();
        monitor_dimensions.height = (monitor_dimensions.height as f32 * 0.7) as u32;
        monitor_dimensions.width = (monitor_dimensions.width as f32 * 0.7) as u32;
        let inner_size = monitor_dimensions.to_logical::<f32>(primary_monitor.scale_factor());

        let window_builder = rg3d::window::WindowBuilder::new()
            .with_title("Jam")
            .with_inner_size(inner_size)
            .with_resizable(true);

        let mut engine = GameEngine::new(window_builder, &events_loop, false).unwrap();

        const fixed_timestep: f32 = 1.0 / FIXED_FPS;

        let time = GameTime {
            clock: Instant::now(),
            elapsed: 0.0,
            delta: fixed_timestep,
        };

        let mut game = Game {
            running: true,
            engine,
            last_tick_time: time::Instant::now(),
            time,
        };

        events_loop.run(move |event, _, control_flow| {
            game.process_input_event(&event);

            match event {
                Event::MainEventsCleared => {
                    let mut dt = game.time.clock.elapsed().as_secs_f64() - game.time.elapsed;
                    while dt >= fixed_timestep as f64 {
                        dt -= fixed_timestep as f64;
                        game.time.elapsed += fixed_timestep as f64;

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
        let _ = window.set_cursor_grab(true);
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


fn main() {
    Game::run();
}
