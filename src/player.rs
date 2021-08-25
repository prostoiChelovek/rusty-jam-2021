use crate::{
    SETTINGS,
    keyboard_input::{KeyMap, Action},
    attached_camera::AttachedCamera,
    character::Character, character_body, character_body::CharacterBody, message::Message,
    request_model,
};
use rg3d::{
    engine::resource_manager::ResourceManager,
    scene::{Scene, node::Node},
    core::algebra::Vector3,
    event::{Event, WindowEvent, ElementState},
};
use std::{
    ops::{Deref, DerefMut},
    sync::mpsc::Sender,
    collections::HashMap,
};

pub struct Player {
    keymap: KeyMap,

    character: Character,
    camera: AttachedCamera,

    actions: HashMap<Action, bool>,
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
        let body = character_body!(resource_manager, scene, player);

        let camera = AttachedCamera::new(scene, body.body.clone());

        let character = Character::new(scene, body);

        let keymap = SETTINGS.read().unwrap().keymap.clone();

        Self {
            character,
            camera,
            keymap,
            actions: Default::default(),
        }
    }

    pub fn process_input_event(&mut self, event: &Event<()>) {
        self.camera.process_input_event(event);

        self.update_input_actions(event);
    }

    pub fn update(&mut self, scene: &mut Scene) {
        self.camera.update(scene);

        self.process_input_actions(scene);
    }

    fn update_input_actions(&mut self, event: &Event<()>) {
        if let Event::WindowEvent { event, .. } = event {
            if let WindowEvent::KeyboardInput { input, .. } = event {
                let state = input.state == ElementState::Pressed;

                if let Some(code) = &input.virtual_keycode {
                    if self.keymap.contains_key(code) {
                        let action = self.keymap[code];
                        *self.actions.entry(action).or_insert(false) = state;
                    }
                }
            }
        }
    }

    fn process_input_actions(&mut self, scene: &mut Scene) {
        let pivot = &scene.graph[self.camera.camera.pivot];
        let look = pivot.look_vector();
        let side = pivot.side_vector();

        let mut velocity = Vector3::default();

        if self.action_state(Action::Left) { velocity += side; }
        if self.action_state(Action::Right) { velocity -= side; }
        if self.action_state(Action::Forward) { velocity += look; }
        if self.action_state(Action::Backward) { velocity -= look; }

        let body = scene
            .physics
            .bodies
            .get_mut(&self.character.body.body)
            .unwrap();
        body.set_angvel(Default::default(), true);
        if let Some(normalized_velocity) = velocity.try_normalize(std::f32::EPSILON) {
            body.set_linvel(
                Vector3::new(
                    normalized_velocity.x * 3.0,
                    body.linvel().y,
                    normalized_velocity.z * 3.0,
                    ),
                    true,
                    );
        }
    }

    fn action_state(&mut self, action: Action) -> bool {
        *self.actions.entry(action).or_insert(false)
    }
}
