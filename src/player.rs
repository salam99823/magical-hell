use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::world::Health;
use crate::GameState;
use crate::*;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

#[derive(Component, Default)]
pub enum PlayerState {
    #[default]
    Idle,
    Run,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                handle_player_death,
                handle_player_input,
                handle_player_enemy_collision_events,
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}

fn handle_player_enemy_collision_events(
    mut player_query: Query<&mut Health, With<Player>>,
    mut events: EventReader<CollisionEvent>,
) {
    let mut health = player_query.single_mut();
    for _ in events.read() {
        health.0 -= ENEMY_DAMAGE;
    }
}

fn handle_player_death(
    player_query: Query<&Health, With<Player>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for health in player_query.iter() {
        if health.0 == 0 {
            next_state.set(GameState::MainMenu);
        }
    }
}

fn handle_player_input(
    mut player_query: Query<(&mut KinematicCharacterController, &mut PlayerState), With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if player_query.is_empty() {
        return;
    }

    for (mut controller, mut player_state) in &mut player_query {
        let w_key = keyboard_input.pressed(KeyCode::KeyW) as u32 as f32;
        let d_key = keyboard_input.pressed(KeyCode::KeyD) as u32 as f32;
        let a_key = -(keyboard_input.pressed(KeyCode::KeyA) as u32 as f32);
        let s_key = -(keyboard_input.pressed(KeyCode::KeyS) as u32 as f32);

        let delta = Vec2::new(d_key + a_key, w_key + s_key).normalize();

        controller.translation = Some(delta * PLAYER_SPEED);

        *player_state = if delta.is_finite() {
            PlayerState::Run
        } else {
            PlayerState::Idle
        }
    }
}
