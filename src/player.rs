use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerMovement
{
    pub speed : f32,
}

pub fn move_player(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&PlayerMovement,&mut Transform)>
)
{
    for(player_movement, mut transform) in query.iter_mut()
    {
        if keys.pressed(KeyCode::W)
        {
            transform.translation.y+=player_movement.speed*time.delta_seconds();
        }
        if keys.pressed(KeyCode::A)
        {
            transform.translation.x-=player_movement.speed*time.delta_seconds();
        }
        if keys.pressed(KeyCode::S)
        {
            transform.translation.y-=player_movement.speed*time.delta_seconds();
        }
        if keys.pressed(KeyCode::D)
        {
            transform.translation.x+=player_movement.speed*time.delta_seconds();
        }

    }
}