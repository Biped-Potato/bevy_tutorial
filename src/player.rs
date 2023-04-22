use bevy::prelude::*;

use crate::animation::Animator;

#[derive(Component)]
pub struct PlayerMovement
{
    pub speed : f32,
}

pub fn move_player(
    time : Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut query: Query<(
    &PlayerMovement,
    &mut Transform,
    &mut Animator,
)>)
{
    for(player_movement, mut transform,mut animator) in query.iter_mut()
    {
        animator.current_animation = "Idle".to_string();
        if keys.pressed(KeyCode::W)
        {
            animator.current_animation = "Walk".to_string();
            transform.translation.y+=player_movement.speed*time.delta_seconds();
        }
        if keys.pressed(KeyCode::A)
        {
            transform.rotation = Quat::from_rotation_y(std::f32::consts::PI);
            animator.current_animation = "Walk".to_string();
            transform.translation.x-=player_movement.speed*time.delta_seconds();
        }
        if keys.pressed(KeyCode::S)
        {
            animator.current_animation = "Walk".to_string();
            transform.translation.y-=player_movement.speed*time.delta_seconds();
        }
        if keys.pressed(KeyCode::D)
        {
            transform.rotation = Quat::default();
            animator.current_animation = "Walk".to_string();
            transform.translation.x+=player_movement.speed*time.delta_seconds();
        }

    }
}