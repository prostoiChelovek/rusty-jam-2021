use rg3d::engine::ColliderHandle;
use rg3d::resource::model::Model;
use rg3d::engine::resource_manager::MaterialSearchOptions;
use std::path::Path;
use rg3d::engine::resource_manager::ResourceManager;
use rg3d::scene::base::BaseBuilder;
use crate::message::Message;
use rg3d::{
    core::{
        algebra::Vector3,
        pool::Handle,
    },
    physics::{
        dynamics::{RigidBodyBuilder, RigidBodyType},
        geometry::ColliderBuilder,
    },
    engine::RigidBodyHandle,
    scene::{Scene, node::Node, physics::Physics},
};
use std::sync::mpsc::Sender;
use crate::request_model;
use crate::settings::CharacterSize;
use crate::character_body::CharacterBody;

pub struct Character {
    pub name: String,

    pub pivot: Handle<Node>,
    pub body: CharacterBody,

    pub health: f32,

    pub sender: Option<Sender<Message>>,
}

impl Default for Character {
    fn default() -> Self {
        Self {
            name: Default::default(),
            pivot: Handle::NONE,
            body: Default::default(),
            health: 100.0,
            sender: None,
        }
    }
}

impl Character {
    pub fn new(scene: &mut Scene, body: CharacterBody) -> Self {
        let pivot = BaseBuilder::new()
            .with_children(&[body.model])
            .build(&mut scene.graph);

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
