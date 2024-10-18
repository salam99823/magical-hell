use bevy::{prelude::*, sprite::Anchor, time::Stopwatch};
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::{
    animation::AnimationTimer,
    gun::{Gun, GunTimer},
    player::{Player, PlayerState},
    resources::GlobalTextureAtlas,
    *,
};

pub struct WorldPlugin;

#[derive(Component)]
pub struct GameEntity;

#[derive(Component, Reflect)]
pub struct Health(pub u32);

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Health>()
            .add_systems(
                OnEnter(GameState::GameInit),
                (init_world, spawn_world_decorations),
            )
            .add_systems(OnExit(GameState::InGame), despawn_all_game_entities);
    }
}

fn init_world(
    mut commands: Commands,
    handle: Res<GlobalTextureAtlas>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    commands
        .spawn((
            Name::new("player"),
            RigidBody::KinematicPositionBased,
            Collider::cuboid(
                TILE_H as f32 * SPRITE_SCALE_FACTOR / 4.0,
                TILE_W as f32 * SPRITE_SCALE_FACTOR / 4.0,
            ),
            LockedAxes::ROTATION_LOCKED,
            CollisionGroups::new(Group::GROUP_1, Group::GROUP_1),
            KinematicCharacterController::default(),
            SpriteBundle {
                texture: handle.image.clone().unwrap(),
                transform: Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
                ..default()
            },
            TextureAtlas {
                layout: handle.layout.clone().unwrap(),
                index: 0,
            },
            Player,
            Health(PLAYER_HEALTH),
            PlayerState::default(),
            AnimationTimer(Timer::from_seconds(0.15, TimerMode::Repeating)),
            GameEntity,
        ))
        .with_children(|parent| {
            parent.spawn((
                Name::new("gun"),
                SpriteBundle {
                    sprite: Sprite {
                        anchor: Anchor::CenterLeft,
                        ..default()
                    },
                    texture: handle.image.clone().unwrap(),
                    ..default()
                },
                TextureAtlas {
                    layout: handle.layout.clone().unwrap(),
                    index: 17,
                },
                Gun,
                GunTimer(Stopwatch::new()),
            ));
        });

    next_state.set(GameState::InGame);
}

fn spawn_world_decorations(mut commands: Commands, handle: Res<GlobalTextureAtlas>) {
    let mut rng = rand::thread_rng();
    for _ in 0..NUM_WORLD_DECORATIONS {
        let x = rng.gen_range(-WORLD_W..WORLD_W);
        let y = rng.gen_range(-WORLD_H..WORLD_H);
        commands.spawn((
            Friction::new(1.0),
            SpriteBundle {
                texture: handle.image.clone().unwrap(),
                transform: Transform::from_xyz(x, y, -1.0)
                    .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
                ..default()
            },
            TextureAtlas {
                layout: handle.layout.clone().unwrap(),
                index: rng.gen_range(24..=25),
            },
            GameEntity,
        ));
    }
}

fn despawn_all_game_entities(
    mut commands: Commands,
    all_entities: Query<Entity, With<GameEntity>>,
) {
    for e in all_entities.iter() {
        commands.entity(e).despawn_recursive();
    }
}
