use crate::{
    SETTINGS,
    settings::CharacterSpeedSettings,
    keyboard_input::{KeyMap, Action},
    character_body::CharacterBody,
    character::CharacterAnimationInput,
};
use rg3d::{
    scene::{Scene, node::Node},
    core::{algebra::Vector3, pool::Handle},
    event::{Event, WindowEvent, ElementState},
};
use std::collections::HashMap;

pub struct MovementControlelr {
    pub keymap: KeyMap,
    pub actions: HashMap<Action, bool>,
    speed: CharacterSpeedSettings,
}

impl MovementControlelr {
    pub fn new(speed: CharacterSpeedSettings) -> Self {
        let keymap = SETTINGS.read().unwrap().keymap.clone();

        Self { 
            keymap,
            actions: Default::default(),
            speed,
        }
    }

    pub fn process_input_event(&mut self, event: &Event<()>) {
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

    pub fn update(&mut self,
                             scene: &mut Scene, 
                             camera_pivot: Handle<Node>,
                             body: &mut CharacterBody) -> CharacterAnimationInput {
        let mut animation_input = CharacterAnimationInput::default();

        let pivot = &scene.graph[camera_pivot];
        let look = pivot.look_vector();
        let side = pivot.side_vector();

        let physics = &mut scene.physics;
        let has_ground_contact = body.has_ground_contact(physics);
        let body = physics
            .bodies
            .get_mut(&body.body)
            .unwrap();

        let mut velocity = Vector3::default();

        if self.action_state(Action::Left) { velocity += side; }
        if self.action_state(Action::Right) { velocity -= side; }
        if self.action_state(Action::Forward) { velocity += look; }
        if self.action_state(Action::Backward) { velocity -= look; }
        if self.action_state(Action::Jump) {
            if has_ground_contact {
                velocity += Vector3::new(0.0, 1.0, 0.0);

                animation_input.just_started_jumping = !animation_input.jumping;
            }
            *self.actions.get_mut(&Action::Jump).unwrap() = false;
        }

        body.set_angvel(Default::default(), true);
        if let Some(normalized_velocity) = velocity.try_normalize(f32::EPSILON) {
            // velocity is not zero and not jumping
            animation_input.running = normalized_velocity.x > 0.0 || normalized_velocity.z > 0.0;

            let speed = &self.speed;
            body.set_linvel(
                Vector3::new(
                    normalized_velocity.x * speed.run,
                    body.linvel().y + normalized_velocity.y * speed.jump,
                    normalized_velocity.z * speed.run,
                    ),
                    true,
                    );
        }
        animation_input.jumping = !has_ground_contact;

        // Damping to prevent sliding
        // TODO: This is needed because Rapier does not have selection of friction
        // models yet.
        if has_ground_contact {
            let mut vel = *body.linvel();
            vel.x *= 0.9;
            vel.z *= 0.9;
            body.set_linvel(vel, true);
        }

        animation_input
    }

    fn action_state(&mut self, action: Action) -> bool {
        *self.actions.entry(action).or_insert(false)
    }
}
