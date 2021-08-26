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

#[derive(Default)]
pub struct CharacterAnimations {
    pub idle: Handle<Animation>,
    pub run: Handle<Animation>,
    pub jump: Handle<Animation>,
}

#[macro_export]
macro_rules! character_animations {
    ($scene:expr, $resource_manager:expr, $body:expr, $($name:ident).+, $settings:ident) => {
        {
            use crate::request_animation;
            // TODO: do this concurrently
            let idle = request_animation!($resource_manager, $($name).+.idle, $settings);
            let run = request_animation!($resource_manager, $($name).+.run, $settings);
            let jump = request_animation!($resource_manager, $($name).+.jump, $settings);

            CharacterAnimations::new($scene, $body,
                                     idle, run, jump)
        }
    };
}

impl CharacterAnimations {
    pub fn new(scene: &mut Scene,
               body: &CharacterBody,
               idle: Model,
               run: Model,
               jump: Model) -> Self {
        let idle = Self::prepare_animation(scene, body, idle);
        let run = Self::prepare_animation(scene, body, run);
        let jump = Self::prepare_animation(scene, body, jump);

        Self { 
            idle,
            run,
            jump,
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

#[derive(Default)]
pub struct CharacterAnimationInput {
    pub is_running: bool,
}

#[derive(Default)]
pub struct CharacterAnimationController {
    pub animations: CharacterAnimations,
    pub machine: AnimationMachine,
}

impl CharacterAnimationController {
    const WALK_TO_IDLE_PARAM: &'static str = "WalkToIdle";
    const WALK_TO_JUMP_PARAM: &'static str = "WalkToJump";
    const IDLE_TO_WALK_PARAM: &'static str = "IdleToWalk";
    const IDLE_TO_JUMP_PARAM: &'static str = "IdleToJump";

    pub fn new(animations: CharacterAnimations) -> Self {
        let mut machine = AnimationMachine::new();

        let idle_node = machine.add_node(machine::PoseNode::make_play_animation(animations.idle));
        let idle_state = machine.add_state(machine::State::new("idle", idle_node));

        let run_node = machine.add_node(machine::PoseNode::make_play_animation(animations.run));
        let run_state = machine.add_state(machine::State::new("run", run_node));

        let jump_node = machine.add_node(machine::PoseNode::make_play_animation(animations.jump));
        let jump_state = machine.add_state(machine::State::new("jump", jump_node));

        machine.add_transition(machine::Transition::new(
            "run->idle",
            run_state,
            idle_state,
            0.5,
            Self::WALK_TO_IDLE_PARAM,
        ));
        machine.add_transition(machine::Transition::new(
            "run->jump",
            run_state,
            jump_state,
            0.5,
            Self::WALK_TO_JUMP_PARAM,
        ));
        machine.add_transition(machine::Transition::new(
            "idle->run",
            idle_state,
            run_state,
            0.5,
            Self::IDLE_TO_WALK_PARAM,
        ));
        machine.add_transition(machine::Transition::new(
            "idle->jump",
            idle_state,
            jump_state,
            0.5,
            Self::IDLE_TO_JUMP_PARAM,
        ));

        machine.set_entry_state(idle_state);

        Self {
            animations,
            machine,
        }
    }

    pub fn apply(&mut self, scene: &mut Scene, time: GameTime, input: CharacterAnimationInput) {
        self.machine
            .set_parameter(
                Self::IDLE_TO_WALK_PARAM,
                machine::Parameter::Rule(input.is_running),
            )
            .set_parameter(
                Self::WALK_TO_IDLE_PARAM,
                machine::Parameter::Rule(!input.is_running),
            )
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

    pub fn update(&mut self, scene: &mut Scene, time: GameTime, animation_input: CharacterAnimationInput) {
        self.animation.apply(scene, time, animation_input);
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

