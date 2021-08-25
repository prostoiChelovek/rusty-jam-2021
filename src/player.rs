use crate::{
    attached_camera::AttachedCamera,
    character::Character, character_body, character_body::CharacterBody, message::Message,
    request_model,
};
use rg3d::{
    engine::resource_manager::ResourceManager,
    scene::Scene,
    event::Event,
};
use std::{
    ops::{Deref, DerefMut},
    sync::mpsc::Sender,
};

pub struct Player {
    character: Character,
    camera: AttachedCamera,
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

        Self {
            character,
            camera,
        }
    }

    pub fn process_input_event(&mut self, event: &Event<()>) {
        self.camera.process_input_event(event);
    }

    pub fn update(&mut self, scene: &mut Scene) {
        self.camera.update(scene);
    }
}
