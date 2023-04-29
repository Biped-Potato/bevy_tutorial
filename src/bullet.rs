use bevy::prelude::*;

use crate::enemy::Enemy;

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
pub struct BulletInfo
{
    pub translation : Vec2,
    pub entity:Entity,
}
pub fn update_bullet_hits(
    bullet_query: Query<(&Transform,Entity),(With<Bullet>,Without<Enemy>)>,
    mut enemy_query: Query<(&mut Enemy,&mut Transform),Without<Bullet>>,
    mut commands:Commands
)
{
    let mut bullet_list = Vec::new();
    for(transform,entity) in bullet_query.iter()
    {
        bullet_list.push(BulletInfo{translation:Vec2::new(transform.translation.x,transform.translation.y),entity:entity});
    }
    let mut bullet_len = bullet_list.len();
    for(mut enemy,transform) in enemy_query.iter_mut()
    {
        let mut i  = 0;
        while i < bullet_len
        {
            if Vec2::distance(bullet_list[i].translation,Vec2::new(transform.translation.x,transform.translation.y)) <= 36.
            {
                enemy.health-=1.;

                commands.entity(bullet_list[i].entity).despawn();

                bullet_list.remove(i);

                i-=1;

                bullet_len-=1;
            }
            i+=1;
        }
    }
}