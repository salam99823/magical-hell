use bevy::math::vec3;
use bevy::utils::Duration;
use bevy::{prelude::*, time::common_conditions::on_timer};
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::animation::AnimationTimer;
use crate::player::Player;
use crate::resources::GlobalTextureAtlas;
use crate::world::{GameEntity, Health};
use crate::*;

pub struct EnemyPlugin;

#[derive(Component)]
pub enum EnemyType {
    Green,
    Red,
    Skin,
}

#[derive(Component)]
pub struct Enemy;

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

fn despawn_dead_enemies(mut commands: Commands, enemy_query: Query<(&Health, Entity), WithEnemy>) {
    for (health, entity) in enemy_query.iter() {
        if health.0 == 0 {
            commands.entity(entity).despawn();
        }
    }
}

fn update_enemy_transform(
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<(&mut KinematicCharacterController, &Transform), WithEnemy>,
) {
    let player_transform = player_query.single();
    for (mut controller, transform) in enemy_query.iter_mut() {
        let dir = (player_transform.translation - transform.translation)
            .normalize()
            .xy();
        controller.translation = Some(dir * ENEMY_SPEED);
    }
}

fn spawn_enemies(
    mut commands: Commands,
    assets: Res<GlobalTextureAtlas>,
    player_query: Query<&Transform, With<Player>>,
    enemy_query: Query<&Transform, WithEnemy>,
) {
    let num_enemies = enemy_query.iter().len();
    let enemy_spawn_count = (MAX_NUM_ENEMIES - num_enemies).min(SPAWN_RATE_PER_SECOND);

    if num_enemies >= MAX_NUM_ENEMIES || player_query.is_empty() {
        return;
    }

    let player_pos = player_query.single().translation.truncate();
    for _ in 0..enemy_spawn_count {
        let enemy_type = EnemyType::get_rand_enemy();
        let (x, y) =
            crate::utils::math::get_random_position_around(player_pos.into(), 500.0..600.0);
        commands.spawn((
            Name::new("enemy"),
            RigidBody::KinematicPositionBased,
            Collider::cuboid(
                TILE_H as f32 * SPRITE_SCALE_FACTOR / 6.0,
                TILE_W as f32 * SPRITE_SCALE_FACTOR / 6.0,
            ),
            LockedAxes::ROTATION_LOCKED,
            CollisionGroups::new(Group::GROUP_1, Group::GROUP_1 | Group::GROUP_2),
            KinematicCharacterController::default(),
            SpriteBundle {
                texture: assets.image.clone().unwrap(),
                transform: Transform::from_translation(vec3(x, y, 1.0))
                    .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
                ..default()
            },
            TextureAtlas {
                layout: assets.layout.clone().unwrap(),
                index: enemy_type.get_base_sprite_index(),
            },
            enemy_type,
            AnimationTimer(Timer::from_seconds(0.08, TimerMode::Repeating)),
            Enemy,
            GameEntity,
        ));
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
