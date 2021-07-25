use bevy::prelude::*;

use crate::{
    core::state_machine::Machine,
    plugins::{
        animation::{
            components::{EntSpriteKV, EntTypeKey},
            traits::*,
        },
        coordinator::{components::*, enemies::components::*},
        resource_manager::components::ResourceManager,
    },
};

impl Animatable<EnemyTypeStates, EnemyAnimationStates> for FromEnemy {
    fn get_texture_handle_from_state(
        &self,
        handle: &Handle<TextureAtlas>,
        resource_manager: &ResourceManager,
    ) -> Option<EnemyAnimationStates> {
        if let Some(x) = resource_manager
            .textures
            .enemies
            .get(&EntSpriteKV::Handle(handle.clone()))
        {
            return match x {
                EntSpriteKV::State(s) => Some(*s),
                _ => None,
            };
        }

        None
    }

    fn get_state_from_texture_handle(
        &self,
        entity_state: &EnemyTypeStates,
        animation_state: &EnemyAnimationStates,
        resource_manager: &ResourceManager,
    ) -> Option<Handle<TextureAtlas>> {
        if let Some(x) = resource_manager
            .textures
            .enemies
            .get(&EntSpriteKV::State(EntTypeKey {
                ty: entity_state.clone(),
                anim_ty: animation_state.clone(),
            }))
        {
            return match x {
                EntSpriteKV::Handle(h) => Some(h.clone()),
                _ => None,
            };
        }

        None
    }
}

impl EnemyTypeState {
    pub fn get(&self) -> EnemyTypeStates {
        self.machine.state
    }

    pub fn enqueue(&mut self, command: CoordinatorCommands) {
        if let Some(new_state) =
            EnemyTypeState::transformation_state_logic(&self.machine.state, &command)
        {
            self.machine.state = new_state;
        }
    }

    fn transformation_state_logic(
        current_state: &EnemyTypeStates,
        command: &CoordinatorCommands,
    ) -> Option<EnemyTypeStates> {
        let transform = || -> EnemyTypeStates {
            match current_state {
                EnemyTypeStates::AngryPig => EnemyTypeStates::Bat,
                EnemyTypeStates::Bat => EnemyTypeStates::Bee,
                EnemyTypeStates::Bee => EnemyTypeStates::BlueBird,
                EnemyTypeStates::BlueBird => EnemyTypeStates::Bunny,
                EnemyTypeStates::Bunny => EnemyTypeStates::Chameleon,
                EnemyTypeStates::Chameleon => EnemyTypeStates::Chicken,
                EnemyTypeStates::Chicken => EnemyTypeStates::Duck,
                EnemyTypeStates::Duck => EnemyTypeStates::FatBird,
                EnemyTypeStates::FatBird => EnemyTypeStates::Ghost,
                EnemyTypeStates::Ghost => EnemyTypeStates::Mushroom,
                EnemyTypeStates::Mushroom => EnemyTypeStates::Plant,
                EnemyTypeStates::Plant => EnemyTypeStates::Radish,
                EnemyTypeStates::Radish => EnemyTypeStates::Rhino,
                EnemyTypeStates::Rhino => EnemyTypeStates::Rocks1,
                EnemyTypeStates::Rocks1 => EnemyTypeStates::Rocks2,
                EnemyTypeStates::Rocks2 => EnemyTypeStates::Rocks3,
                EnemyTypeStates::Rocks3 => EnemyTypeStates::Skull,
                EnemyTypeStates::Skull => EnemyTypeStates::Slime,
                EnemyTypeStates::Slime => EnemyTypeStates::Snail,
                EnemyTypeStates::Snail => EnemyTypeStates::Trunk,
                EnemyTypeStates::Trunk => EnemyTypeStates::Turtle,
                EnemyTypeStates::Turtle => EnemyTypeStates::AngryPig,
                _ => EnemyTypeStates::AngryPig,
            }
        };

        match command {
            CoordinatorCommands::EnemyTransform(_) => Some(transform()),
            _ => None,
        }
    }
}

impl EnemyTypeStates {
    pub fn get_default_animation(enemy_type_state: &EnemyTypeStates) -> EnemyAnimationStates {
        match enemy_type_state {
            EnemyTypeStates::AngryPig => EnemyAnimationStates::Idle,
            EnemyTypeStates::Bat => EnemyAnimationStates::Idle,
            EnemyTypeStates::Bee => EnemyAnimationStates::Idle,
            EnemyTypeStates::BlueBird => EnemyAnimationStates::Flying,
            EnemyTypeStates::Bunny => EnemyAnimationStates::Idle,
            EnemyTypeStates::Chameleon => EnemyAnimationStates::Idle,
            EnemyTypeStates::Chicken => EnemyAnimationStates::Idle,
            EnemyTypeStates::Duck => EnemyAnimationStates::Idle,
            EnemyTypeStates::FatBird => EnemyAnimationStates::Idle,
            EnemyTypeStates::Ghost => EnemyAnimationStates::Idle,
            EnemyTypeStates::Mushroom => EnemyAnimationStates::Idle,
            EnemyTypeStates::Plant => EnemyAnimationStates::Idle,
            EnemyTypeStates::Radish => EnemyAnimationStates::Idle,
            EnemyTypeStates::Rhino => EnemyAnimationStates::Idle,
            EnemyTypeStates::Rocks1 => EnemyAnimationStates::Idle,
            EnemyTypeStates::Rocks2 => EnemyAnimationStates::Idle,
            EnemyTypeStates::Rocks3 => EnemyAnimationStates::Idle,
            EnemyTypeStates::Skull => EnemyAnimationStates::Idle,
            EnemyTypeStates::Slime => EnemyAnimationStates::IdleRun,
            EnemyTypeStates::Snail => EnemyAnimationStates::Idle,
            EnemyTypeStates::Trunk => EnemyAnimationStates::Idle,
            EnemyTypeStates::Turtle => EnemyAnimationStates::Idle,
            _ => EnemyAnimationStates::Idle,
        }
    }
}

impl Default for EnemyTypeState {
    fn default() -> Self {
        Self {
            machine: Machine::<EnemyTypeStates> {
                state: EnemyTypeStates::AngryPig,
            },
        }
    }
}
