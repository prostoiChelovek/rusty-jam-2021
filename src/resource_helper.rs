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
            let models = &$settings.models;
            let model_file = &models.$($name).+;

            $resource_manager.request_model(
                models.get_model_path(model_file),
                MaterialSearchOptions::MaterialsDirectory(models.get_materials_path()),
                )
                .await
                .unwrap()
        }
    };
}

