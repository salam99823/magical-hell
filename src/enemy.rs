use bevy::math::vec3;
use bevy::utils::Duration;
use bevy::{prelude::*, time::common_conditions::on_timer};
use bevy_rapier2d::prelude::*;
use rand::Rng;
use std::f32::consts::PI;

use crate::animation::AnimationTimer;
use crate::player::Player;
use crate::resources::GlobalTextureAtlas;
use crate::world::GameEntity;
use crate::*;

pub struct EnemyPlugin;

#[derive(Component)]
pub struct Enemy {
    pub health: f32,
}

#[derive(Component)]
pub enum EnemyType {
    Green,
    Red,
    Skin,
}

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_enemies.run_if(on_timer(Duration::from_secs_f32(ENEMY_SPAWN_INTERVAL))),
                update_enemy_transform,
                despawn_dead_enemies,
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}

type WithEnemy = (With<Enemy>, Without<Player>);

fn despawn_dead_enemies(mut commands: Commands, enemy_query: Query<(&Enemy, Entity), WithEnemy>) {
    if enemy_query.is_empty() {
        return;
    }

    for (enemy, entity) in enemy_query.iter() {
        if enemy.health <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn update_enemy_transform(
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<(&mut KinematicCharacterController, &Transform), WithEnemy>,
) {
    if player_query.is_empty() || enemy_query.is_empty() {
        return;
    }

    let player_pos = player_query.single().translation;
    for (mut controller, transform) in enemy_query.iter_mut() {
        let dir = (player_pos - transform.translation).normalize() * ENEMY_SPEED;
        controller.translation = Some(Vec2::new(dir.x, dir.y));
    }
}

fn spawn_enemies(
    mut commands: Commands,
    handle: Res<GlobalTextureAtlas>,
    player_query: Query<&Transform, With<Player>>,
    enemy_query: Query<&Transform, (With<Enemy>, Without<Player>)>,
) {
    let num_enemies = enemy_query.iter().len();
    let enemy_spawn_count = (MAX_NUM_ENEMIES - num_enemies).min(SPAWN_RATE_PER_SECOND);

    if num_enemies >= MAX_NUM_ENEMIES || player_query.is_empty() {
        return;
    }

    let player_pos = player_query.single().translation.truncate();
    for _ in 0..enemy_spawn_count {
        let (x, y) = get_random_position_around(player_pos);
        let enemy_type = EnemyType::get_rand_enemy();
        commands.spawn((
            RigidBody::Dynamic,
            Collider::cuboid(
                TILE_H as f32 * SPRITE_SCALE_FACTOR / 6.0,
                TILE_W as f32 * SPRITE_SCALE_FACTOR / 6.0,
            ),
            LockedAxes::ROTATION_LOCKED,
            CollisionGroups::new(Group::GROUP_2, Group::GROUP_2 | Group::GROUP_1),
            KinematicCharacterController::default(),
            SpriteBundle {
                texture: handle.image.clone().unwrap(),
                transform: Transform::from_translation(vec3(x, y, 1.0))
                    .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
                ..default()
            },
            TextureAtlas {
                layout: handle.layout.clone().unwrap(),
                index: enemy_type.get_base_sprite_index(),
            },
            Enemy::default(),
            enemy_type,
            AnimationTimer(Timer::from_seconds(0.08, TimerMode::Repeating)),
            GameEntity,
        ));
    }
}

fn get_random_position_around(pos: Vec2) -> (f32, f32) {
    let mut rng = rand::thread_rng();
    let angle = rng.gen_range(0.0..PI * 2.0);
    let dist = rng.gen_range(500.0..750.0);

    let offset_x = angle.cos() * dist;
    let offset_y = angle.sin() * dist;

    let random_x = pos.x + offset_x;
    let random_y = pos.y + offset_y;

    (random_x, random_y)
}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            health: ENEMY_HEALTH,
        }
    }
}

impl EnemyType {
    fn get_rand_enemy() -> Self {
        let mut rng = rand::thread_rng();
        let rand_index = rng.gen_range(0..3);
        match rand_index {
            0 => Self::Green,
            1 => Self::Red,
            _ => Self::Skin,
        }
    }

    pub fn get_base_sprite_index(&self) -> usize {
        match self {
            EnemyType::Green => 8,
            EnemyType::Red => 12,
            EnemyType::Skin => 20,
        }
    }
}
