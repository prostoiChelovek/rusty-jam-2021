use config::{ConfigError, Config, File};
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct CharacterTexture {
    pub model: String,
}

#[derive(Debug, Deserialize)]
pub struct Textures {
    pub data_dir: String,
    pub player_texture: CharacterTexture
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub textures: Textures,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::default();
        s.merge(File::with_name("settings/textures"))?;

        s.try_into()
    }
}

impl Textures {
    pub fn get_data_dir_path(&self) -> PathBuf {
        PathBuf::from(&self.data_dir)
    }
}
