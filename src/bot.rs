use crate::{
    SETTINGS,
    GameTime,
    character::Character, 
    character_animation::{CharacterAnimations, CharacterAnimationController},
    character_body::CharacterBody,
    request_model, character_body, character_animations,
    message::Message,
};
use rg3d::{
    engine::resource_manager::ResourceManager,
    scene::Scene,
    core::algebra::Vector3,
};
use std::{
    ops::{Deref, DerefMut},
    sync::mpsc::Sender,
};

pub struct Bot {
    pub character: Character,
}

impl Deref for Bot {
    type Target = Character;

    fn deref(&self) -> &Self::Target {
        &self.character
    }
}

impl DerefMut for Bot {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.character
    }
}

impl Bot {
    pub async fn new(
        scene: &mut Scene,
        resource_manager: &ResourceManager,
        sender: Sender<Message>,
        position: Vector3<f32>
    ) -> Self {
        let body = character_body!(resource_manager, scene, bot, position);

        let settings = &SETTINGS.read().unwrap();

        let animations = character_animations!(scene, resource_manager, &body, bot, settings);
        let animation_controller = CharacterAnimationController::new(animations);

        let character = Character::new(scene, body, animation_controller);

        Self {
            character,
        }
    }

    pub fn update(&mut self, scene: &mut Scene, time: GameTime) {
        let animation_input = Default::default();
        self.character.update(scene, time, animation_input);
    }

}
