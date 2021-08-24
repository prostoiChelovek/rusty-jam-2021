#[macro_export]
macro_rules! request_model {
    ($resource_manager:ident, $($name:ident).+) => {
        {
            use crate::SETTINGS;
            let settings = &SETTINGS.read().unwrap();
            request_model!($resource_manager, $($name).+, settings)
        }
    };
    ($resource_manager:ident, $($name:ident).+, $settings:ident) => {
        {
            let textures = &$settings.textures;

            $resource_manager.request_model(
                Path::new(&textures.$($name).+),
                MaterialSearchOptions::MaterialsDirectory(textures.get_data_dir_path()),
                )
                .await
                .unwrap()
        }
    };
}

