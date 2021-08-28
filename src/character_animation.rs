use crate::{
    GameTime,
    character_body::CharacterBody,
};
use rg3d::{
    core::pool::Handle,
    resource::model::Model,
    animation::{
        machine, machine::Machine as AnimationMachine,
        Animation
    },
    scene::Scene,
};

#[derive(Default)]
pub struct CharacterAnimations {
    pub idle: Handle<Animation>,
    pub run: Handle<Animation>,
    pub jump: Handle<Animation>,
    pub attack: Handle<Animation>,
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
            let attack = request_animation!($resource_manager, $($name).+.attack, $settings);

            CharacterAnimations::new($scene, $body,
                                     idle, run, jump, attack)
        }
    };
}

impl CharacterAnimations {
    pub fn new(scene: &mut Scene,
               body: &CharacterBody,
               idle: Model,
               run: Model,
               jump: Model,
               attack: Model) -> Self {
        let idle = Self::prepare_animation(scene, body, idle);
        let run = Self::prepare_animation(scene, body, run);
        let jump = Self::prepare_animation(scene, body, jump);
        let attack = Self::prepare_animation(scene, body, attack);

        scene.animations.get_mut(jump).set_loop(false);

        Self { 
            idle,
            run,
            jump,
            attack,
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

#[derive(Debug, Default)]
pub struct CharacterAnimationInput {
    pub running: bool,

    pub jumping: bool,
    pub just_started_jumping: bool,

    pub attacking: bool,
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
    const JUMP_TO_WALK_PARAM: &'static str = "JumpToIdle";
    const JUMP_TO_IDLE_PARAM: &'static str = "JumpToWalk";

    const WALK_TO_ATTACK_PARAM: &'static str = "WalkToAttack";
    const IDLE_TO_ATTACK_PARAM: &'static str = "IdleToAttack";
    const JUMP_TO_ATTACK_PARAM: &'static str = "JumpToAttack";
    const ATTACK_TO_WALK_PARAM: &'static str = "AttackToWalk";
    const ATTACK_TO_IDLE_PARAM: &'static str = "AttackToIdle";
    const ATTACK_TO_JUMP_PARAM: &'static str = "AttackToJump";

    pub fn new(animations: CharacterAnimations) -> Self {
        let mut machine = AnimationMachine::new();

        // TODO: get rid of code duplication for every animation in several places
        let idle_node = machine.add_node(machine::PoseNode::make_play_animation(animations.idle));
        let idle_state = machine.add_state(machine::State::new("idle", idle_node));

        let run_node = machine.add_node(machine::PoseNode::make_play_animation(animations.run));
        let run_state = machine.add_state(machine::State::new("run", run_node));

        let jump_node = machine.add_node(machine::PoseNode::make_play_animation(animations.jump));
        let jump_state = machine.add_state(machine::State::new("jump", jump_node));

        let attack_node = machine.add_node(machine::PoseNode::make_play_animation(animations.attack));
        let attack_state = machine.add_state(machine::State::new("attack", attack_node));

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
            0.25,
            Self::WALK_TO_JUMP_PARAM,
        ));
        machine.add_transition(machine::Transition::new(
            "idle->run",
            idle_state,
            run_state,
            0.1,
            Self::IDLE_TO_WALK_PARAM,
        ));
        machine.add_transition(machine::Transition::new(
            "idle->jump",
            idle_state,
            jump_state,
            0.25,
            Self::IDLE_TO_JUMP_PARAM,
        ));
        machine.add_transition(machine::Transition::new(
            "jump->run",
            jump_state,
            run_state,
            0.1,
            Self::JUMP_TO_WALK_PARAM,
        ));
        machine.add_transition(machine::Transition::new(
            "jump->idle",
            jump_state,
            idle_state,
            0.5,
            Self::JUMP_TO_IDLE_PARAM,
        ));

        machine.add_transition(machine::Transition::new(
            "run->attack",
            run_state,
            attack_state,
            0.1,
            Self::WALK_TO_ATTACK_PARAM,
        ));
        machine.add_transition(machine::Transition::new(
            "idle->attack",
            idle_state,
            attack_state,
            0.1,
            Self::IDLE_TO_ATTACK_PARAM,
        ));
        machine.add_transition(machine::Transition::new(
            "jump->attack",
            jump_state,
            attack_state,
            0.1,
            Self::JUMP_TO_ATTACK_PARAM,
        ));
        machine.add_transition(machine::Transition::new(
            "attack->run",
            attack_state,
            run_state,
            0.1,
            Self::ATTACK_TO_WALK_PARAM,
        ));
        machine.add_transition(machine::Transition::new(
            "attack->idle",
            attack_state,
            idle_state,
            0.1,
            Self::ATTACK_TO_IDLE_PARAM,
        ));
        machine.add_transition(machine::Transition::new(
            "attack->jump",
            attack_state,
            jump_state,
            0.1,
            Self::ATTACK_TO_JUMP_PARAM,
        ));
        machine.set_entry_state(idle_state);

        Self {
            animations,
            machine,
        }
    }

    pub fn apply(&mut self, scene: &mut Scene, time: GameTime, input: CharacterAnimationInput) {
        if input.just_started_jumping {
            scene.animations.get_mut(self.animations.jump).rewind();
        }

        self.machine
            .set_parameter(
                Self::IDLE_TO_WALK_PARAM,
                machine::Parameter::Rule(input.running),
            )
            .set_parameter(
                Self::WALK_TO_IDLE_PARAM,
                machine::Parameter::Rule(!input.running),
            )
            .set_parameter(
                Self::WALK_TO_JUMP_PARAM,
                machine::Parameter::Rule(input.jumping),
            )
            .set_parameter(
                Self::IDLE_TO_JUMP_PARAM,
                machine::Parameter::Rule(input.jumping),
            )
            .set_parameter(
                Self::JUMP_TO_WALK_PARAM,
                machine::Parameter::Rule(input.running && !input.jumping),
            )
            .set_parameter(
                Self::JUMP_TO_IDLE_PARAM,
                machine::Parameter::Rule(!input.running && !input.jumping),
            )
            .set_parameter(
                Self::WALK_TO_ATTACK_PARAM,
                machine::Parameter::Rule(input.attacking),
            )
            .set_parameter(
                Self::IDLE_TO_ATTACK_PARAM,
                machine::Parameter::Rule(input.attacking),
            )
            .set_parameter(
                Self::JUMP_TO_ATTACK_PARAM,
                machine::Parameter::Rule(!input.jumping && input.attacking),
            )
            .set_parameter(
                Self::ATTACK_TO_WALK_PARAM,
                machine::Parameter::Rule(!input.attacking && input.running),
            )
            .set_parameter(
                Self::ATTACK_TO_IDLE_PARAM,
                machine::Parameter::Rule(!input.attacking && !input.running),
            )
            .set_parameter(
                Self::ATTACK_TO_JUMP_PARAM,
                machine::Parameter::Rule(!input.attacking && input.jumping),
            )
            .evaluate_pose(&scene.animations, time.delta)
            .apply(&mut scene.graph);
    }
}
