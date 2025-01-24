use std::collections::HashMap;

use bevy::prelude::*;

pub mod animation;
pub mod bullet;
pub mod cursor_info;
pub mod enemy;
pub mod enemy_spawner;
pub mod gun;
pub mod player;
pub mod player_attach;

fn main() {
    App::new()
        .insert_resource(cursor_info::OffsetedCursorPosition { x: 0., y: 0. })
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(
            Update,
            (
                animation::animate_sprite,
                player::move_player,
                gun::gun_controls,
                player_attach::attach_objects,
                bullet::update_bullets,
                bullet::update_bullet_hits,
                enemy::update_enemies,
                enemy_spawner::update_spawning,
            ),
        )
        .add_systems(Startup, setup)
        .run();
}

pub fn create_player_anim_hashmap() -> HashMap<String, animation::Animation> {
    let mut hash_map = HashMap::new();

    hash_map.insert(
        "Walk".to_string(),
        animation::Animation {
            start: 1,
            end: 3,
            looping: true,
            cooldown: 0.1,
        },
    );

    hash_map.insert(
        "Idle".to_string(),
        animation::Animation {
            start: 1,
            end: 1,
            looping: true,
            cooldown: 0.1,
        },
    );

    hash_map
}

pub fn create_gun_anim_hashmap() -> HashMap<String, animation::Animation> {
    let mut hash_map = HashMap::new();

    hash_map.insert(
        "Shoot".to_string(),
        animation::Animation {
            start: 1,
            end: 5,
            looping: false,
            cooldown: 0.1,
        },
    );

    hash_map.insert(
        "Idle".to_string(),
        animation::Animation {
            start: 1,
            end: 1,
            looping: true,
            cooldown: 0.1,
        },
    );

    hash_map
}

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let mut texture_handle = asset_server.load("player.png");
    let mut texture_atlas =
        TextureAtlasLayout::from_grid(UVec2::new(8 + 1, 9 + 1), 3, 1, Some(UVec2::new(1, 1)), None);
    let mut texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn(Camera2d);
    commands.spawn((
        Sprite::from_atlas_image(texture_handle, TextureAtlas::from(texture_atlas_handle)),
        Transform::from_scale(Vec3::splat(5.0)),
        animation::Animator {
            timer: 0.,
            cooldown: 0.05,
            last_animation: "Walk".to_string(),
            current_animation: "Walk".to_string(),
            animation_bank: create_player_anim_hashmap(),
        },
        player::PlayerMovement { speed: 100. },
    ));

    texture_handle = asset_server.load("gun.png");
    texture_atlas =
        TextureAtlasLayout::from_grid(UVec2::new(8 + 1, 8 + 1), 5, 1, Some(UVec2::new(1, 1)), None);
    texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn((
        Sprite::from_atlas_image(texture_handle, texture_atlas_handle.into()),
        Transform::from_scale(Vec3::splat(5.0)),
        animation::Animator {
            timer: 0.,
            cooldown: 0.05,
            last_animation: "Shoot".to_string(),
            current_animation: "Shoot".to_string(),
            animation_bank: create_gun_anim_hashmap(),
        },
        player_attach::PlayerAttach {
            offset: Vec2::new(15., -5.),
        },
        gun::GunController {
            shoot_cooldown: 0.3,
            shoot_timer: 0.,
        },
    ));

    commands.spawn((
        Transform::default(),
        enemy_spawner::EnemySpawner {
            cooldown: 1.,
            timer: 1.,
        },
    ));
}
