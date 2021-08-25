use crate::{
    SETTINGS,
    keyboard_input::{KeyMap, Action}, character_body::CharacterBody,
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
}

impl MovementControlelr {
    pub fn new() -> Self {
        let keymap = SETTINGS.read().unwrap().keymap.clone();

        Self { 
            keymap,
            actions: Default::default(),
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
                             body: &mut CharacterBody) {
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
            }
            *self.actions.get_mut(&Action::Jump).unwrap() = false;
        }

        body.set_angvel(Default::default(), true);
        if let Some(normalized_velocity) = velocity.try_normalize(f32::EPSILON) {
            body.set_linvel(
                Vector3::new(
                    normalized_velocity.x * 3.0,
                    body.linvel().y + normalized_velocity.y * 3.0,
                    normalized_velocity.z * 3.0,
                    ),
                    true,
                    );
        }

        // Damping to prevent sliding
        // TODO: This is needed because Rapier does not have selection of friction
        // models yet.
        let mut vel = *body.linvel();
        if has_ground_contact {
            vel.x *= 0.9;
            vel.z *= 0.9;
        }

        // TODO: stupid hack that prevents 'bumpy' movement 
        // (for some reason, y oscillates close to 0 while moving)
        vel.y = if vel.y.abs() <= 10.0f32.powi(-4) {0.0} else {vel.y};
        body.set_linvel(vel, true);
    }

    fn action_state(&mut self, action: Action) -> bool {
        *self.actions.entry(action).or_insert(false)
    }
}
