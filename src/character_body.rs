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
    scene::{Scene, node::Node, physics::Physics},
    resource::model::Model,
};
use crate::settings::CharacterSize;

#[derive(Default)]
pub struct CharacterBody {
    pub body: RigidBodyHandle,
    pub model: Handle<Node>,
    pub spine: Handle<Node>,
    pub collider: ColliderHandle,
    pub size: CharacterSize,
}

impl CharacterBody {
    pub fn new(scene: &mut Scene, 
                 model: Model,
                 spine: String,
                 size: CharacterSize, scale: f32) -> Self {
        let body = scene.physics.add_body(
            RigidBodyBuilder::new(RigidBodyType::Dynamic)
            .lock_rotations()
            .build(),
            );

        let model = model.instantiate_geometry(scene);

        scene.graph[model]
            .local_transform_mut()
            .set_scale(Vector3::new(scale, scale, scale));

        let collider = scene.physics.add_collider(
            ColliderBuilder::capsule_y(size.0 / 2.0, size.1)
            .translation(Vector3::new(0.0, 1.1, 0.0)) // TODO: should be done differently
            .friction(0.0)
            .build(),
            &body,
            );

        let spine = scene.graph.find_by_name(model, &spine);

        Self {
            body,
            model,
            spine,
            collider,
            size
        }
    }

    pub fn has_ground_contact(&self, physics: &Physics) -> bool {
        let body = physics.bodies.get(&self.body).unwrap();
        for contact in physics.narrow_phase.contacts_with(body.colliders()[0]) {
            for manifold in contact.manifolds.iter() {
                if manifold.local_n1.y > 0.7 {
                    return true;
                }
            }
        }
        false
    }
}

#[macro_export]
macro_rules! character_body {
    ($resource_manager:ident, $scene:expr, $($name:ident).+) => {
        {
            use crate::SETTINGS;
            let settings = &SETTINGS.read().unwrap();
            let models = &settings.models;
            let spine = models.$($name).+.spine.clone();
            let size = models.$($name).+.size;
            let scale = models.$($name).+.scale;

            let model = request_model!($resource_manager, $($name).+.model, settings);
            CharacterBody::new($scene, model, spine, size, scale)
        }
    };
}

