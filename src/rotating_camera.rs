use rg3d::{
    core::{algebra::{Vector3, UnitQuaternion}, pool::Handle},
    engine::resource_manager::{MaterialSearchOptions, ResourceManager},
    scene::{
        base::BaseBuilder, camera::CameraBuilder, node::Node, transform::TransformBuilder, Scene,
    },
};

pub struct RotatingCamera {
    pub camera: Handle<Node>,

    pub pivot: Handle<Node>,
    pub hinge: Handle<Node>,

    pub yaw: f32,
    pub pitch: f32,
}

impl RotatingCamera {
    pub fn new(scene: &mut Scene) -> Self {
        let camera = CameraBuilder::new(
            BaseBuilder::new().with_local_transform(
                TransformBuilder::new()
                .with_local_position(Vector3::new(0.0, 0.25, 0.0))
                .build(),
                ),
                )
            .build(&mut scene.graph);

        let hinge = BaseBuilder::new()
            .with_local_transform(
                TransformBuilder::new()
                .with_local_position(Vector3::new(0.0, 0.55, 0.0))
                .build(),
                )
            .with_children(&[camera])
            .build(&mut scene.graph);

        let pivot = BaseBuilder::new()
            .with_children(&[hinge])
            .build(&mut scene.graph);

        Self {
            camera,
            pivot,
            hinge,
            yaw: 0.0, pitch: 0.0,
        }
    }
}
