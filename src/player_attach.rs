use bevy::prelude::*;

use crate::player::PlayerMovement;

#[derive(Component)]
pub struct PlayerAttach {
    pub offset: Vec2,
}

pub fn attach_objects(
    player_query: Query<&mut Transform, (With<PlayerMovement>, Without<PlayerAttach>)>,
    mut objects_query: Query<(&PlayerAttach, &mut Transform), Without<PlayerMovement>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (attach, mut transform) in objects_query.iter_mut() {
            transform.translation =
                player_transform.translation + Vec3::new(attach.offset.x, attach.offset.y, 0.);
        }
    }
}
