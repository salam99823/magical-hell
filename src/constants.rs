// Window
pub const WW: f32 = 1200.0;
pub const WH: f32 = 900.0;

// Sprites
pub const SPRITE_SHEET_PATH: &str = "images/assets.png";
pub const SPRITE_SCALE_FACTOR: f32 = 2.0;
pub const TILE_W: u32 = 16;
pub const TILE_H: u32 = 16;
pub const SPRITE_SHEET_W: u32 = 8;
pub const SPRITE_SHEET_H: u32 = 8;

// World
pub const NUM_WORLD_DECORATIONS: u32 = 1;
pub const WORLD_W: f32 = 250.0;
pub const WORLD_H: f32 = 250.0;

// Player
pub const PLAYER_SPEED: f32 = 5.0;
pub const PLAYER_HEALTH: u32 = 100;

// Enemy
pub const MAX_NUM_ENEMIES: usize = 5;
pub const ENEMY_DAMAGE: u32 = 1;
pub const SPAWN_RATE_PER_SECOND: usize = 500;
pub const ENEMY_HEALTH: u32 = 100;
pub const ENEMY_SPAWN_INTERVAL: f32 = 1.0;
pub const ENEMY_SPEED: f32 = 1.0;

// Gun
pub const BULLET_SPAWN_INTERVAL: f32 = 0.2;
pub const BULLET_TIME_SECS: f32 = 1.0;
pub const BULLET_SPEED: f32 = 200.0;
pub const BULLET_DAMAGE: f32 = 15.0;

pub const NUM_BULLETS_PER_SHOT: u32 = 5;

// Colors
pub const BG_COLOR: (u8, u8, u8) = (197, 204, 184);
