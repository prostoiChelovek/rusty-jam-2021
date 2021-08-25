use crate::{
    SETTINGS,
    attached_camera::AttachedCamera,
    character::Character, character_body, character_body::CharacterBody, message::Message,
    movement_controller::MovementControlelr,
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
    pub character: Character,
    pub camera: AttachedCamera,
    pub movement_controller: MovementControlelr,
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

        let settings = &SETTINGS.read().unwrap();
        let camera = AttachedCamera::new(scene, body.body.clone(), &settings.player.camera);

        let character = Character::new(scene, body);

        scene.graph.link_nodes(character.body.model, camera.camera.pivot);

        let movement_controller = MovementControlelr::new();

        Self {
            character,
            camera,
            movement_controller,
        }
    }

    pub fn process_input_event(&mut self, event: &Event<()>) {
        self.camera.process_input_event(event);

        self.movement_controller.process_input_event(event);
    }

    pub fn update(&mut self, scene: &mut Scene) {
        self.camera.update(scene);

        self.movement_controller.update(scene,
                                        self.camera.camera.pivot,
                                        &mut self.character.body);
    }


}
