use crate::{
    message::Message,
    character::Character, character_body::CharacterBody, character_body,
    request_model,
};
use rg3d::{
    core::pool::Handle,
    scene::{node::Node, Scene},
    engine::resource_manager::{MaterialSearchOptions, ResourceManager}
};
use std::{
    ops::{Deref, DerefMut},
    sync::mpsc::Sender,
};

pub struct Player {
    character: Character,
    camera: Handle<Node>,
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
        let character = Character::new(scene, body);

        Self {
            character,
            camera: Default::default()
        }
    }
}
