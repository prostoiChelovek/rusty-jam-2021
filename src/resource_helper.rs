#[macro_export]
macro_rules! request_resource {
    ($resource_manager:expr, $type:ident, $($name:ident).+) => {
        {
            use crate::SETTINGS;

            let settings = &SETTINGS.read().unwrap();
            request_resource!($resource_manager, $type, $($name).+, settings)
        }
    };
    ($resource_manager:expr, $type:ident, $($name:ident).+, $settings:ident) => {
        {
            use std::path::PathBuf;
            use rg3d::engine::resource_manager::MaterialSearchOptions;

            let file = &$settings.$type.$($name).+;
            let path = PathBuf::from(&$settings.data_dir).join(stringify!($type)).join(file);
            $resource_manager
                .request_model(
                    path,
                    MaterialSearchOptions::MaterialsDirectory($settings.get_materials_path()),
                    )
                .await
                .unwrap()
        }
    };
}

#[macro_export]
macro_rules! request_model {
    ($resource_manager:expr, $($name:ident).+ $(, $settings:ident)?) => {
            crate::request_resource!($resource_manager, models, $($name).+ $(, $settings)?)
    };
}

#[macro_export]
macro_rules! request_scene {
    ($resource_manager:expr, $($name:ident).+ $(, $settings:ident)?) => {
            crate::request_resource!($resource_manager, scenes, $($name).+ $(, $settings)?)
    };
}
