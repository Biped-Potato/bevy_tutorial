use bevy::prelude::*;

use crate::{animation::Animator, cursor_info::OffsetedCursorPosition, gun::GunController};

#[derive(Component)]
pub struct PlayerMovement {
    pub speed: f32,
}

pub fn move_player(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&PlayerMovement, &mut Transform, &mut Animator)>,
    mut gun_query: Query<&mut TextureAtlasSprite, (With<GunController>, Without<PlayerMovement>)>,
    cursor_res: ResMut<OffsetedCursorPosition>,
) {
    for (player_movement, mut transform, mut animator) in query.iter_mut() {
        animator.current_animation = "Idle".to_string();
        if keys.pressed(KeyCode::W) {
            animator.current_animation = "Walk".to_string();
            transform.translation.y += player_movement.speed * time.delta_seconds();
        }
        if keys.pressed(KeyCode::A) {
            transform.rotation = Quat::from_rotation_y(std::f32::consts::PI);
            animator.current_animation = "Walk".to_string();
            transform.translation.x -= player_movement.speed * time.delta_seconds();
        }
        if keys.pressed(KeyCode::S) {
            animator.current_animation = "Walk".to_string();
            transform.translation.y -= player_movement.speed * time.delta_seconds();
        }
        if keys.pressed(KeyCode::D) {
            transform.rotation = Quat::default();
            animator.current_animation = "Walk".to_string();
            transform.translation.x += player_movement.speed * time.delta_seconds();
        }
        for mut sprite in gun_query.iter_mut() {
            if cursor_res.x - transform.translation.x >= 0. {
                sprite.flip_y = false;
            } else {
                sprite.flip_y = true;
            }
        }
    }
}
