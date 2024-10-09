use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::GameState;
use crate::*;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;
#[derive(Component)]
pub struct Health(pub f32);

#[derive(Component, Default)]
pub enum PlayerState {
    #[default]
    Idle,
    Run,
}

#[derive(Event)]
pub struct PlayerEnemyCollisionEvent;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerEnemyCollisionEvent>().add_systems(
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
    mut events: EventReader<PlayerEnemyCollisionEvent>,
) {
    if player_query.is_empty() {
        return;
    }

    let mut health = player_query.single_mut();
    for _ in events.read() {
        health.0 -= ENEMY_DAMAGE;
    }
}

fn handle_player_death(
    player_query: Query<&Health, With<Player>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if player_query.is_empty() {
        return;
    }
    let health = player_query.single();
    if health.0 <= 0.0 {
        next_state.set(GameState::MainMenu);
    }
}

fn handle_player_input(
    mut player_query: Query<(&mut Velocity, &mut PlayerState), With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if player_query.is_empty() {
        return;
    }

    for (mut vel, mut player_state) in &mut player_query {
        let w_key = keyboard_input.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]) as u32 as f32;
        let d_key = keyboard_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]) as u32 as f32;
        let a_key =
            -(keyboard_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]) as u32 as f32);
        let s_key =
            -(keyboard_input.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]) as u32 as f32);

        let delta = Vec2::new(w_key + s_key, d_key + a_key).normalize();

        vel.linvel = delta * PLAYER_SPEED;

        *player_state = if vel.linvel != Vec2::ZERO {
            PlayerState::Run
        } else {
            PlayerState::Idle
        }
    }
}
