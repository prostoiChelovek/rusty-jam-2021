use crate::{
    character::Character, character_body, character_body::CharacterBody, message::Message,
    request_model,
};
use rg3d::{
    core::{algebra::{Vector3, UnitQuaternion}, pool::Handle},
    engine::resource_manager::{MaterialSearchOptions, ResourceManager},
    scene::{
        base::BaseBuilder, camera::CameraBuilder, node::Node, transform::TransformBuilder, Scene,
    },
    event::{DeviceEvent, Event}
};
use std::{
    ops::{Deref, DerefMut},
    sync::mpsc::Sender,
};

pub struct Player {
    character: Character,
    camera: Handle<Node>,
    yaw: f32,
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
        let camera = CameraBuilder::new(
            BaseBuilder::new().with_local_transform(
                TransformBuilder::new()
                .with_local_position(Vector3::new(0.0, 0.25, 0.0))
                .build(),
                ),
                )
            .build(&mut scene.graph);

        let pivot = BaseBuilder::new()
            .with_children(&[camera])
            .build(&mut scene.graph);

        let body = character_body!(resource_manager, scene, player);
        let character = Character::new(scene, body, pivot);

        Self {
            character,
            camera,
            yaw: 0.0,
        }
    }

    pub fn process_input_event(&mut self, event: &Event<()>) -> bool {
        if let Event::DeviceEvent { event, .. } = event {
            match event {
                DeviceEvent::MouseMotion { delta } => {
                    self.yaw -= delta.0 as f32 * 0.3;
                },
                _ => (),
            };
        }
        true
    }

    pub fn update(&mut self, scene: &mut Scene) {
        let body = scene
            .physics
            .bodies
            .get_mut(&self.character.body.body)
            .unwrap();

        let mut position = *body.position();
        position.rotation =
            UnitQuaternion::from_axis_angle(&Vector3::y_axis(), self.yaw.to_radians());
        body.set_position(position, true);

    }
}
