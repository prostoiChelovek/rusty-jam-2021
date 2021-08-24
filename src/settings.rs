use config::{ConfigError, Config, File};
use std::path::PathBuf;

pub type CharacterSize = (f32, f32);

#[derive(Debug, Deserialize)]
pub struct CharacterModel {
    pub model: String,
    pub scale: f32,
    pub size: CharacterSize,
}

#[derive(Debug, Deserialize)]
pub struct Models {
    pub player: CharacterModel
}

#[derive(Debug, Deserialize)]
pub struct Scene {
    pub model: String,
}

#[derive(Debug, Deserialize)]
pub struct Scenes {
    pub main: Scene,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub data_dir: String,
    pub models: Models,
    pub scenes: Scenes,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::default();
        s.merge(File::with_name("settings/settings"))?;
        s.merge(File::with_name("settings/models"))?;
        s.merge(File::with_name("settings/scenes"))?;

        s.try_into()
    }
}

impl Settings {
    pub fn get_materials_path(&self) -> PathBuf {
        PathBuf::from(&self.data_dir).join("textures")
    }

    pub fn get_model_path(&self, model: &String) -> PathBuf {
        PathBuf::from(&self.data_dir).join("models").join(model)
    }

    pub fn get_scene_path(&self, scene: &String) -> PathBuf {
        PathBuf::from(&self.data_dir).join("scenes").join(scene)
    }
}
