use bevy::prelude::*;

use crate::{animation::Animator, cursor_info::OffsetedCursorPosition, gun::GunController};

#[derive(Component)]
pub struct PlayerMovement {
    pub speed: f32,
}

pub fn move_player(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&PlayerMovement, &mut Transform, &mut Sprite, &mut Animator)>,
    mut gun_query: Query<&mut Sprite, (With<GunController>, Without<PlayerMovement>)>,
    cursor_res: ResMut<OffsetedCursorPosition>,
) {
    for (player_movement, mut transform, mut sprite, mut animator) in query.iter_mut() {
        animator.current_animation = "Idle".to_string();
        if keys.pressed(KeyCode::KeyW) {
            animator.current_animation = "Walk".to_string();
            transform.translation.y += player_movement.speed * time.delta_seconds();
        }
        if keys.pressed(KeyCode::KeyA) {
            sprite.flip_x = true;
            animator.current_animation = "Walk".to_string();
            transform.translation.x -= player_movement.speed * time.delta_seconds();
        }
        if keys.pressed(KeyCode::KeyS) {
            animator.current_animation = "Walk".to_string();
            transform.translation.y -= player_movement.speed * time.delta_seconds();
        }
        if keys.pressed(KeyCode::KeyD) {
            sprite.flip_x = false;
            animator.current_animation = "Walk".to_string();
            transform.translation.x += player_movement.speed * time.delta_seconds();
        }
        for mut sprite in gun_query.iter_mut() {
            sprite.flip_y = cursor_res.x - transform.translation.x < 0.;
        }
    }
}
