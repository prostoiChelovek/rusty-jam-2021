use crate::{
    SETTINGS,
    settings::CharacterSpeedSettings,
    keyboard_input::{KeyMap, Action},
    character_body::CharacterBody,
    character_animation::CharacterAnimationInput,
};
use rg3d::{
    scene::{Scene, node::Node},
    core::{algebra::{Vector3, UnitQuaternion}, pool::Handle},
    event::{Event, WindowEvent, ElementState},
};
use std::collections::HashMap;

pub struct BotAi {
    speed: CharacterSpeedSettings,
}

impl BotAi {
    pub fn new(speed: CharacterSpeedSettings) -> Self {
        Self { 
            speed,
        }
    }

    pub fn update(&mut self,
                             scene: &mut Scene, 
                             body: &mut CharacterBody) -> CharacterAnimationInput {

        let mut animation_input = CharacterAnimationInput::default();

        let pivot = &scene.graph[body.pivot];
        let self_position = pivot.global_position();

        let target = Vector3::new(1.0,self_position .y, 1.0);
        let direction = target - self_position;
        let distance = direction.norm();

        let rigid_body = scene.physics.bodies.get_mut(&body.body).unwrap();

        // Make sure bot is facing towards the target.
        let mut position = *rigid_body.position();
        position.rotation = UnitQuaternion::face_towards(
            &Vector3::new(direction.x, 0.0, direction.z),
            &Vector3::y_axis(),
            );
        rigid_body.set_position(position, true);

        // Move only if we're far enough from the target.
        if distance > 0.1 {
            // Normalize direction vector and scale it by movement speed.
            let xz_velocity = direction.scale(1.0 / distance).scale(self.speed.run);

            let new_velocity =
                Vector3::new(xz_velocity.x, rigid_body.linvel().y, xz_velocity.z);

            rigid_body.set_linvel(new_velocity, true);

            animation_input.running = true;
        } else {
            animation_input.running = false;
        }

        animation_input
    }
}
