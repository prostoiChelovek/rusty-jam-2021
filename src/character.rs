use crate::{
    GameTime,
    message::Message,
    character_body::CharacterBody,
    character_animation::{
        CharacterAnimationInput, CharacterAnimationController
    },
};
use rg3d::{
    core::algebra::Vector3,
    scene::{Scene, physics::Physics},
};
use std::sync::mpsc::Sender;

pub struct Character {
    pub body: CharacterBody,
    pub animation: CharacterAnimationController,

    pub health: f32,

    pub sender: Option<Sender<Message>>,
}

impl Default for Character {
    fn default() -> Self {
        Self {
            body: Default::default(),
            animation: Default::default(),
            health: 100.0,
            sender: None,
        }
    }
}

impl Character {
    pub fn new(scene: &mut Scene,
               body: CharacterBody,
               animation: CharacterAnimationController) -> Self {
        Self {
            body,
            animation,
            ..Default::default()
        }
    }

    pub fn update(&mut self, scene: &mut Scene, time: GameTime, animation_input: CharacterAnimationInput) {
        self.animation.apply(scene, time, animation_input);
    }

    pub fn set_position(&mut self, physics: &mut Physics, position: Vector3<f32>) {
        let body = physics.bodies.get_mut(&self.body.body).unwrap();
        let mut body_position = *body.position();
        body_position.translation.vector = position;
        body.set_position(body_position, true);
    }

    pub fn position(&self, physics: &Physics) -> Vector3<f32> {
        physics
            .bodies
            .get(&self.body.body)
            .unwrap()
            .position()
            .translation
            .vector
    }
}

