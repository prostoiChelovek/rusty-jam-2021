use crate::settings::CameraSettings;
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
    pub fn new(scene: &mut Scene, pivot: Handle<Node>, settings: &CameraSettings) -> Self {
        let camera = CameraBuilder::new(
            BaseBuilder::new().with_local_transform(
                TransformBuilder::new()
                .with_local_position(settings.get_offset())
                .build(),
                ),
                )
            .build(&mut scene.graph);

        let hinge = BaseBuilder::new()
            .with_local_transform(
                TransformBuilder::new()
                .with_local_position(settings.get_hinge_offset())
                .build(),
                )
            .with_children(&[camera])
            .build(&mut scene.graph);
        scene.graph.link_nodes(hinge, pivot);

        Self {
            camera,
            pivot,
            hinge,
            yaw: 0.0, pitch: 0.0,
        }
    }
}
