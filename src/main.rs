use bevy::prelude::*;
pub mod animation;
use std::collections::HashMap;
fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
    .add_system(animation::animate_sprite)
    .add_startup_system(setup)
    .run();
}
pub fn create_player_anim_hashmap()->HashMap<String,animation::Animation>
{
    let mut hash_map = HashMap::new();

    hash_map.insert("Walk".to_string(),animation::Animation{start:1,end:3,looping:true,cooldown:0.1});

    hash_map.insert("Idle".to_string(),animation::Animation{start:1,end:1,looping:true,cooldown:0.1});

    return hash_map;
}
pub fn setup(mut commands: Commands, asset_server : Res<AssetServer>, mut texture_atlases : ResMut<Assets<TextureAtlas>>)
{
    let texture_handle = asset_server.load("player.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(8.0+1.0,9.0+1.0),
        3,
        1,
        Some(Vec2::new(1.,1.)),
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteSheetBundle{
        texture_atlas:texture_atlas_handle,
        transform:Transform::from_scale(Vec3::splat(5.0)),
        ..default()})
    .insert(animation::Animator{
        timer:0.,
        cooldown:0.05,
        last_animation : "Walk".to_string(),
        current_animation : "Walk".to_string(),
        animation_bank: create_player_anim_hashmap(),
    });
}