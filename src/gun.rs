use bevy::{prelude::*, window::PrimaryWindow};

use crate::{animation::Animator, bullet::Bullet, cursor_info::OffsetedCursorPosition};

const BULLET_LIFETIME: f32 = 10.0;

const BULLET_SPEED: f32 = 1000.;
#[derive(Component)]
pub struct GunController {
    pub shoot_cooldown: f32,
    pub shoot_timer: f32,
}

#[allow(clippy::too_many_arguments)]
pub fn gun_controls(
    mut cursor_res: ResMut<OffsetedCursorPosition>,
    mut gun_query: Query<(&mut GunController, &mut Transform, &mut Animator)>,
    mut cursor: EventReader<CursorMoved>,
    primary_query: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
    buttons: Res<ButtonInput<MouseButton>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    for (mut gun_controller, mut transform, mut animator) in gun_query.iter_mut() {
        gun_controller.shoot_timer -= time.delta_secs();
        if gun_controller.shoot_timer > 0. {
            animator.current_animation = "Shoot".to_string();
        } else {
            animator.current_animation = "Idle".to_string();
        }
        let Ok(primary) = primary_query.get_single() else {
            return;
        };
        let cursor_position = match cursor.read().last() {
            Some(cursor_moved) => {
                Vec2::new(cursor_moved.position.x, -cursor_moved.position.y)
                    + Vec2::new(-primary.width() / 2., primary.height() / 2.)
            }
            None => Vec2::new(cursor_res.x, cursor_res.y),
        };
        cursor_res.x = cursor_position.x;
        cursor_res.y = cursor_position.y;

        let diff = cursor_position - Vec2::new(transform.translation.x, transform.translation.y);
        let angle = diff.y.atan2(diff.x);
        transform.rotation = Quat::from_axis_angle(Vec3::new(0., 0., 1.), angle);
        if gun_controller.shoot_timer <= 0. && buttons.pressed(MouseButton::Left) {
            let mut spawn_transform = Transform::from_scale(Vec3::splat(5.0));
            spawn_transform.translation = transform.translation;
            spawn_transform.rotation = Quat::from_axis_angle(Vec3::new(0., 0., 1.), angle);
            gun_controller.shoot_timer = gun_controller.shoot_cooldown;
            commands.spawn((
                Sprite::from_image(asset_server.load("bullet.png")),
                spawn_transform,
                Bullet {
                    lifetime: BULLET_LIFETIME,
                    speed: BULLET_SPEED,
                    direction: diff.normalize(),
                },
            ));
        }
    }
}
