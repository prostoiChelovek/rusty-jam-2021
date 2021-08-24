use rg3d::{
    core::{
        algebra::Vector3,
        pool::Handle,
    },
    scene::{Scene, node::Node, physics::Physics, base::BaseBuilder},
};
use std::sync::mpsc::Sender;
use crate::{message::Message, character_body::CharacterBody,};

pub struct Character {
    pub pivot: Handle<Node>,
    pub body: CharacterBody,

    pub health: f32,

    pub sender: Option<Sender<Message>>,
}

impl Default for Character {
    fn default() -> Self {
        Self {
            pivot: Handle::NONE,
            body: Default::default(),
            health: 100.0,
            sender: None,
        }
    }
}

impl Character {
    pub fn new(scene: &mut Scene, body: CharacterBody, pivot: Handle<Node>) -> Self {
        scene.physics_binder.bind(pivot, body.body);

        Self {
            body,
            pivot,
            ..Default::default()
        }
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

