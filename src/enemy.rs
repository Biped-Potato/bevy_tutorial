use bevy::prelude::*;

use crate::player::PlayerMovement;

#[derive(Component)]
pub struct Enemy {
    pub health: f32,
    pub speed: f32,
}

pub fn update_enemies(
    time: Res<Time>,
    mut enemy_query: Query<(&Enemy, &mut Transform, Entity), Without<PlayerMovement>>,
    player_query: Query<(&PlayerMovement, &mut Transform), Without<Enemy>>,
    mut commands: Commands,
) {
    if let Ok((_player_movement, player_transform)) = player_query.get_single() {
        for (enemy, mut transform, entity) in enemy_query.iter_mut() {
            let moving = Vec3::normalize(player_transform.translation - transform.translation)
                * enemy.speed
                * time.delta_secs();
            transform.translation += moving;

            if enemy.health <= 0. {
                commands.entity(entity).despawn();
            }
        }
    }
}
