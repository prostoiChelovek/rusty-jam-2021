use crate::{
    SETTINGS,
    GameTime,
    attached_camera::AttachedCamera,
    character::Character, 
    character_animation::{CharacterAnimations, CharacterAnimationController},
    character_body::CharacterBody,
    request_model, character_body, character_animations,
    message::Message,
    movement_controller::MovementControlelr,
    keyboard_input::Action,
};
use rg3d::{
    engine::resource_manager::ResourceManager,
    scene::Scene,
    core::algebra::Vector3,
    event::{Event, WindowEvent, ElementState, MouseButton},
};
use std::{
    ops::{Deref, DerefMut},
    sync::mpsc::Sender,
};

pub struct Player {
    pub character: Character,
    pub camera: AttachedCamera,
    pub movement_controller: MovementControlelr,
    is_attacking: bool,
    attack_duration: u128,
    attack_start_time: u128,
}

impl Deref for Player {
    type Target = Character;

    fn deref(&self) -> &Self::Target {
        &self.character
    }
}

impl DerefMut for Player {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.character
    }
}

impl Player {
    pub async fn new(
        scene: &mut Scene,
        resource_manager: &ResourceManager,
        sender: Sender<Message>,
    ) -> Self {
        let body = character_body!(resource_manager, scene, player, Vector3::new(0.0, 0.0, 0.0));

        let settings = &SETTINGS.read().unwrap();

        let animations = character_animations!(scene, resource_manager, &body, player, settings);
        let animation_controller = CharacterAnimationController::new(animations);

        let settings = &settings.player;

        let camera = AttachedCamera::new(scene,
                                         body.body.clone(),
                                         body.pivot.clone(),
                                         &settings.camera);

        let character = Character::new(scene, body, animation_controller);

        scene.graph.link_nodes(character.body.model, camera.camera.pivot);

        let movement_controller = MovementControlelr::new(settings.speed.clone());

        Self {
            character,
            camera,
            movement_controller,
            is_attacking: false,
            attack_duration: 1000,
            attack_start_time: 0,
        }
    }

    pub fn process_input_event(&mut self, event: &Event<()>) {
        self.camera.process_input_event(event);

        self.movement_controller.process_input_event(event);

        if let Event::WindowEvent { event, .. } = event {
            if let &WindowEvent::MouseInput { state, button, .. } = event {
                let state = state == ElementState::Pressed;
                if button == MouseButton::Left {
                    // TODO: generalize movement_controller
                    *self.movement_controller.actions.entry(Action::Attack).or_insert(false) = state;
                }
            }
        }
    }

    pub fn update(&mut self, scene: &mut Scene, time: GameTime) {
        self.camera.update(scene);

        let mut animation_input = self.movement_controller.update(scene,
                                                                  self.camera.camera.pivot,
                                                                  &mut self.character.body);

        let elapsed_time = time.clock.elapsed().as_millis();

        self.is_attacking = self.is_attacking && !(elapsed_time - self.attack_start_time >= self.attack_duration);

        if !self.is_attacking && self.movement_controller.action_state(Action::Attack) {
            self.is_attacking = true;
            self.attack_start_time = elapsed_time;
        }

        animation_input.attacking = self.is_attacking;

        self.character.update(scene, time, animation_input);
    }

}
