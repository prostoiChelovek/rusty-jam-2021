use rg3d::{
    core::{
        algebra::Vector3,
        pool::Handle,
    },
    physics::{
        dynamics::{RigidBodyBuilder, RigidBodyType},
        geometry::ColliderBuilder,
    },
    engine::{RigidBodyHandle, ColliderHandle},
    scene::{Scene, node::Node},
    resource::model::Model,
};
use crate::settings::CharacterSize;

#[derive(Default)]
pub struct CharacterBody {
    pub body: RigidBodyHandle,
    pub model: Handle<Node>,
    pub collider: ColliderHandle,
    pub size: CharacterSize,
}

impl CharacterBody {
    pub fn new(scene: &mut Scene, 
                 model: Model,
                 size: CharacterSize, scale: f32) -> Self {
        let body = scene.physics.add_body(
            RigidBodyBuilder::new(RigidBodyType::Dynamic)
            .lock_rotations()
            .build(),
            );

        let model = model.instantiate_geometry(scene);

        scene.graph[model]
            .local_transform_mut()
            .set_position(Vector3::new(0.0, -size.0 / 2.0, 0.0))
            .set_scale(Vector3::new(scale, scale, scale));

        let collider = scene.physics.add_collider(
            ColliderBuilder::capsule_y(size.0 / 2.0, size.1)
            .friction(0.0)
            .build(),
            &body,
            );

        Self {
            body,
            model,
            collider,
            size
        }
    }
}

#[macro_export]
macro_rules! character_body {
    ($resource_manager:ident, $scene:expr, $($name:ident).+) => {
        {
            use crate::SETTINGS;
            let settings = &SETTINGS.read().unwrap();
            let models = &settings.models;
            let size = models.$($name).+.size;
            let scale = models.$($name).+.scale;

            let model = request_model!($resource_manager, $($name).+.model, settings);
            CharacterBody::new($scene, model, size, scale)
        }
    };
}

