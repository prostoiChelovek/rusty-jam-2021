use crate::keyboard_input::KeyMap;
use rg3d::core::algebra::Vector3;
use config::{ConfigError, Config, File};
use std::path::PathBuf;

pub type CharacterSize = (f32, f32);

#[derive(Debug, Deserialize)]
pub struct CharacterModel {
    pub model: String,
    pub spine: String,
    pub scale: f32,
    pub size: CharacterSize,
}

#[derive(Debug, Deserialize)]
pub struct WeaponModel { // TODO: duplication
    pub model: String,
    pub scale: f32,
}

#[derive(Debug, Deserialize)]
pub struct Models {
    pub player: CharacterModel,
    pub bot: CharacterModel,
    pub weapon: WeaponModel,
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
pub struct CameraSettings {
    pub offset: (f32, f32, f32),
    pub hinge_offset: (f32, f32, f32),
}


#[derive(Debug, Deserialize, Clone)]
pub struct CharacterSpeedSettings {
    pub run: f32,
    pub jump: f32,
}

#[derive(Debug, Deserialize)]
pub struct AnimationSettings {
    pub idle: String,
    pub run: String,
    pub jump: String,
    pub attack: String,
}

#[derive(Debug, Deserialize)]
pub struct Animations {
    pub player: AnimationSettings,
    pub bot: AnimationSettings,
}

#[derive(Debug, Deserialize)]
pub struct PlayerSettings {
    pub camera: CameraSettings,
    pub speed: CharacterSpeedSettings,
    pub hand_node: String,
}

#[derive(Debug, Deserialize)]
pub struct BotSettings {
    pub speed: CharacterSpeedSettings,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub data_dir: String,
    pub models: Models,
    pub scenes: Scenes,
    pub animations: Animations,
    pub keymap: KeyMap,
    pub player: PlayerSettings,
    pub bot: BotSettings,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::default();
        s.merge(File::with_name("settings/settings"))?;
        s.merge(File::with_name("settings/models"))?;
        s.merge(File::with_name("settings/animations"))?;
        s.merge(File::with_name("settings/scenes"))?;
        s.merge(File::with_name("settings/keymap"))?;
        s.merge(File::with_name("settings/player"))?;
        s.merge(File::with_name("settings/bot"))?;

        s.try_into()
    }
}

impl Settings {
    pub fn get_materials_path(&self) -> PathBuf {
        PathBuf::from(&self.data_dir).join("textures")
    }
}

impl CameraSettings {
    pub fn get_offset(&self) -> Vector3<f32> {
        CameraSettings::tuple_to_vector(&self.offset)
    }

    pub fn get_hinge_offset(&self) -> Vector3<f32> {
        CameraSettings::tuple_to_vector(&self.hinge_offset)
    }

    fn tuple_to_vector(tuple: &(f32, f32, f32)) -> Vector3<f32> {
        Vector3::new(tuple.0, tuple.1, tuple.2)
    }
}
