use rg3d::{
    core::{
        algebra::Vector3,
        pool::Handle,
    },
    resource::model::Model,
    animation::{
        machine, machine::Machine as AnimationMachine,
        Animation
    },
    scene::{Scene, physics::Physics},
};
use std::sync::mpsc::Sender;
use crate::{
    GameTime,
    message::Message,
    character_body::CharacterBody,
};

pub struct CharacterAnimations {
    pub run: Handle<Animation>,
}

#[macro_export]
macro_rules! character_animations {
    ($scene:expr, $resource_manager:expr, $body:expr, $($name:ident).+, $settings:ident) => {
        {
            use crate::request_animation;
            let run = request_animation!($resource_manager, $($name).+.run, $settings);

            CharacterAnimations::new($scene, $body, run)
        }
    };
}

impl Default for CharacterAnimations {
    fn default() -> Self {
        Self {
            run: Default::default()
        }
    }
}

impl CharacterAnimations {
    pub fn new(scene: &mut Scene, body: &CharacterBody, run: Model) -> Self {
        let run = Self::prepare_animation(scene, body, run);

        Self { 
            run
        }
    }

    fn prepare_animation(scene: &mut Scene,
                         body: &CharacterBody,
                         animation: Model) -> Handle<Animation> {
        let animation = animation.retarget_animations(body.model, scene)[0];
        scene
            .animations
            .get_mut(animation)
            .set_node_track_enabled(body.spine, false);

        animation
    }
}

pub struct CharacterAnimationController {
    pub animations: CharacterAnimations,
    pub machine: AnimationMachine,
}

impl Default for CharacterAnimationController {
    fn default() -> Self {
        Self {
            animations: Default::default(),
            machine: Default::default(),
        }
    }
}

impl CharacterAnimationController {
    pub fn new(animations: CharacterAnimations) -> Self {
        let mut machine = AnimationMachine::new();

        let walk_node = machine.add_node(machine::PoseNode::make_play_animation(animations.run));
        let walk_state = machine.add_state(machine::State::new("Walk", walk_node));

        machine.set_entry_state(walk_state);

        Self {
            animations,
            machine,
        }
    }

    pub fn apply(&mut self, scene: &mut Scene, time: GameTime) {
        self.machine
            .evaluate_pose(&scene.animations, time.delta)
            .apply(&mut scene.graph);
    }
}

pub struct Character {
    pub body: CharacterBody,
    pub animation: CharacterAnimationController,

    pub health: f32,

    pub sender: Option<Sender<Message>>,
}

impl Default for Character {
    fn default() -> Self {
        Self {
            body: Default::default(),
            animation: Default::default(),
            health: 100.0,
            sender: None,
        }
    }
}

impl Character {
    pub fn new(scene: &mut Scene,
               body: CharacterBody,
               animation: CharacterAnimationController) -> Self {
        Self {
            body,
            animation,
            ..Default::default()
        }
    }

    pub fn update(&mut self, scene: &mut Scene, time: GameTime) {
        self.animation.apply(scene, time);
    }

    pub fn set_position(&mut self, physics: &mut Physics, position: Vector3<f32>) {
        let body = physics.bodies.get_mut(&self.body.body).unwrap();
        let mut body_position = *body.position();
        body_position.translation.vector = position;
        body.set_position(body_position, true);
    }

    pub fn position(&self, physics: &Physics) -> Vector3<f32> {
        physics
            .bodies
            .get(&self.body.body)
            .unwrap()
            .position()
            .translation
            .vector
    }
}

