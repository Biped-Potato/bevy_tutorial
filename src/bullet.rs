use bevy::prelude::*;

#[derive(Component)]
pub struct Bullet
{
    pub lifetime:f32,
    pub speed : f32,
    pub direction : Vec2,
}
pub fn update_bullets(mut bullet_query : Query<(&mut Bullet,&mut Transform,Entity)>,time:Res<Time>,mut commands : Commands)
{
    for(mut bullet, mut transform,entity) in bullet_query.iter_mut()
    {
        bullet.lifetime -=time.delta_seconds();
        let moving = bullet.speed*bullet.direction*time.delta_seconds();
        transform.translation+=Vec3::new(moving.x,moving.y,0.);
        if bullet.lifetime <=0.
        {
            commands.entity(entity).despawn();
        }
    }
}