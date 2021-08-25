use crate::rotating_camera::RotatingCamera;
use rg3d::{
    core::algebra::{Vector3, UnitQuaternion},
    engine::RigidBodyHandle,
    scene::Scene,
    event::{DeviceEvent, Event}
};

pub struct AttachedCamera {
    camera: RotatingCamera,
    body: RigidBodyHandle,
}

impl AttachedCamera {
    pub fn new(scene: &mut Scene, body: RigidBodyHandle) -> Self {
        let camera = RotatingCamera::new(scene);

        scene.physics_binder.bind(camera.pivot, body);

        Self { 
            camera,
            body,
        }
    }

    pub fn process_input_event(&mut self, event: &Event<()>) {
        if let Event::DeviceEvent { event, .. } = event {
            if let DeviceEvent::MouseMotion { delta } = event {
                self.camera.yaw -= delta.0 as f32 * 0.3;

                self.camera.pitch += delta.1 as f32 * 0.01;
                self.camera.pitch = self.camera.pitch
                    .clamp(-90.0f32.to_radians(), 90.0f32.to_radians());
            }
        }
    }

    pub fn update(&mut self, scene: &mut Scene) {
        let body = scene
            .physics
            .bodies
            .get_mut(&self.body)
            .unwrap();

        let mut position = *body.position();
        position.rotation =
            UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 
                                            self.camera.yaw.to_radians());
        body.set_position(position, true);

        scene.graph[self.camera.hinge]
            .local_transform_mut()
            .set_rotation(UnitQuaternion::from_axis_angle(
                    &Vector3::x_axis(),
                    self.camera.pitch,
                    ));
    }
}

