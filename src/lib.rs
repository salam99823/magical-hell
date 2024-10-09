use animation::AnimationPlugin;
use bevy::prelude::*;
use camera::FollowCameraPlugin;
pub use constants::*;
use enemy::EnemyPlugin;
use gui::GuiPlugin;
use gun::GunPlugin;
use player::PlayerPlugin;
use resources::ResourcesPlugin;
use world::WorldPlugin;

mod animation;
mod camera;
mod constants;
mod enemy;
mod gui;
mod gun;
mod player;
mod resources;
mod world;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    Loading,
    MainMenu,
    GameInit,
    InGame,
}

pub struct Game;

impl Plugin for Game {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_state::<GameState>().add_plugins((
            FollowCameraPlugin,
            GuiPlugin,
            GunPlugin,
            PlayerPlugin,
            AnimationPlugin,
            ResourcesPlugin,
            WorldPlugin,
            EnemyPlugin,
        ));
    }
}
