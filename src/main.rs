#![deny(unsafe_code)]

extern crate config;
extern crate serde;

#[macro_use]
extern crate serde_derive;

mod message;
mod keyboard_input;
mod character_body;
mod character_animation;
mod character;
mod attached_camera;
mod rotating_camera;
mod player;
mod movement_controller;
mod settings;
mod resource_helper;

use rg3d::{
    dpi::LogicalSize,
    core::{
        futures::executor::block_on,
        pool::Handle,
    },
    engine::Engine,
    scene::Scene,
    event::{DeviceEvent, Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    gui::{
        node::StubNode,
        UserInterface,
    },
};
use once_cell::sync::Lazy;
use crate::{
    settings::Settings,
    message::Message,
    player::Player,
};
use std::{
    fs::File,
    io::Write,
    sync::{
        mpsc::{self, Receiver, Sender},
        RwLock,
    },
    time::{self, Instant},
};

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
    scene: Handle<Scene>,
    last_tick_time: time::Instant,
    running: bool,
    time: GameTime,
    events_receiver: Receiver<Message>,
    events_sender: Sender<Message>,
    player: Player,
}

#[derive(Copy, Clone)]
pub struct GameTime {
    clock: time::Instant,
    elapsed: f64,
    delta: f32,
}

impl Game {
    pub async fn new(event_loop: &MyEventLoop, title: &'static str) -> Self {
        let (sender, receiver) = mpsc::channel();

        let inner_size = get_inner_size(event_loop);

        let window_builder = rg3d::window::WindowBuilder::new()
            .with_title(title)
            .with_inner_size(inner_size)
            .with_resizable(true);

        let mut engine = GameEngine::new(window_builder, event_loop, false).unwrap();

        let time = GameTime {
            clock: Instant::now(),
            elapsed: 0.0,
            delta: FIXED_TIMESTEP,
        };

        let window = engine.get_window();
        window.set_cursor_visible(false);
        window.set_cursor_grab(true).unwrap();

        let resource_manager = &engine.resource_manager;
        let mut scene = Scene::new();

        request_scene!(&engine.resource_manager, main.model)
            .instantiate_geometry(&mut scene);

        let player = Player::new(&mut scene, resource_manager, sender.clone()).await;

        let scene = engine.scenes.add(scene);

        Self {
            running: true,
            engine,
            scene,
            last_tick_time: time::Instant::now(),
            time,
            events_sender: sender,
            events_receiver: receiver,
            player,
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

                        game.engine.update(FIXED_TIMESTEP);

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
        let scene = &mut self.engine.scenes[self.scene];

        self.player.update(scene, time);
    }

    fn process_input_event(&mut self, event: &Event<()>) {
        self.player.process_input_event(event);

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
    let game = block_on(Game::new(&event_loop, "Jam"));
    Game::run(game, event_loop);
}
