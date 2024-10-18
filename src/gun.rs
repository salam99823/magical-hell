use bevy::{prelude::*, time::Stopwatch, utils::Instant};
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::{
    player::Player,
    resources::{CursorPosition, GlobalTextureAtlas},
    world::GameEntity,
    *,
};

pub struct GunPlugin;

#[derive(Component)]
pub struct Gun;
#[derive(Component)]
pub struct GunTimer(pub Stopwatch);
#[derive(Component)]
pub struct Bullet;
#[derive(Component)]
pub struct SpawnInstant(Instant);

impl Plugin for GunPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_gun_transform, handle_gun_input, despawn_old_bullets)
                .run_if(in_state(GameState::InGame)),
        );
    }
}

fn despawn_old_bullets(
    mut commands: Commands,
    bullet_query: Query<(&SpawnInstant, Entity), With<Bullet>>,
) {
    for (instant, e) in bullet_query.iter() {
        if instant.0.elapsed().as_secs_f32() > BULLET_TIME_SECS {
            commands.entity(e).despawn();
        }
    }
}

fn update_gun_transform(
    cursor_pos: Res<CursorPosition>,
    player_query: Query<&Transform, With<Player>>,
    mut gun_query: Query<&mut Transform, (With<Gun>, Without<Player>)>,
) {
    let player_pos = player_query.single().translation.truncate();
    let cursor_pos = cursor_pos.0.unwrap_or(player_pos);

    let mut gun_transform = gun_query.single_mut();

    let angle =
        (player_pos.y - cursor_pos.y).atan2(player_pos.x - cursor_pos.x) - std::f32::consts::PI;
    gun_transform.rotation = Quat::from_rotation_z(angle);

    gun_transform.translation = Vec3::new(
        7.0 * angle.cos(),
        7.0 * angle.sin(),
        gun_transform.translation.z,
    );
}

fn handle_gun_input(
    mut commands: Commands,
    time: Res<Time>,
    mut gun_query: Query<(&GlobalTransform, &mut GunTimer), With<Gun>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    assets: Res<GlobalTextureAtlas>,
) {
    let (gun_transform, mut gun_timer) = gun_query.single_mut();
    gun_timer.0.tick(time.delta());

    if !mouse_button_input.pressed(MouseButton::Left) {
        return;
    }

    let mut rng = rand::thread_rng();
    let bullet_direction = gun_transform.right().normalize();
    let gun_pos = gun_transform.translation().truncate() + bullet_direction.xy() * 10.0;
    if gun_timer.0.elapsed_secs() >= BULLET_SPAWN_INTERVAL {
        gun_timer.0.reset();

        for _ in 0..NUM_BULLETS_PER_SHOT {
            commands.spawn((
                RigidBody::KinematicVelocityBased,
                Collider::ball(2.0),
                Velocity {
                    linvel: Vec2::new(
                        bullet_direction.x + rng.gen_range(-0.2..0.2),
                        bullet_direction.y + rng.gen_range(-0.2..0.2),
                    ) * BULLET_SPEED,
                    angvel: 0.0,
                },
                CollisionGroups::new(Group::GROUP_2, Group::GROUP_2),
                SpriteBundle {
                    texture: assets.image.clone().unwrap(),
                    transform: Transform::from_xyz(gun_pos.x, gun_pos.y, 0.0)
                        .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
                    ..default()
                },
                TextureAtlas {
                    layout: assets.layout.clone().unwrap(),
                    index: 16,
                },
                Bullet,
                SpawnInstant(Instant::now()),
                GameEntity,
            ));
        }
    }
}
