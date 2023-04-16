use std::collections::HashMap;

use bevy::prelude::*;
#[derive(Clone,Copy)]
pub struct Animation
{
    pub cooldown : f32,
    pub start:usize,
    pub end:usize,
    pub looping : bool,
}
#[derive(Clone, Component)]
pub struct Animator
{
    pub animation_bank:HashMap<String,Animation>,
    pub current_animation: String,
    pub last_animation:String,
    pub timer:f32,
    pub cooldown:f32,
}
impl Default for Animator
{
    fn default() -> Self { 
        Animator{
            animation_bank :create_anim_hashmap(),
            timer: 0.,
            cooldown : 0.1,
            last_animation : " ".to_string(),
            current_animation : "Idle".to_string()
        }
    }
}
pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &mut Animator,
        &mut TextureAtlasSprite,
    )>,
) {
    for (mut animator, mut sprite) in query.iter_mut() {
        let anim = animator.animation_bank[animator.current_animation.as_str()];
        if animator.last_animation!=animator.current_animation
        {
            sprite.index = anim.start-1;
        }
        animator.timer = animator.timer-time.delta().as_secs_f32();
        if animator.timer<=0.
        {
            animator.timer = anim.cooldown;
            if anim.looping == true
            {
                sprite.index = ((sprite.index + 1 -(anim.start-1)) % ( anim.end- anim.start+1)) +anim.start-1;
            }
            else if anim.looping == false
            {
                
                sprite.index +=1;
                if sprite.index > anim.end -1
                {
                    sprite.index = anim.end-1;
                }
            }
        }
        animator.last_animation = animator.current_animation.clone();
    }
}
pub fn create_anim_hashmap()->HashMap<String,Animation>
{
    let mut hash_map = HashMap::new();
    hash_map.insert("".to_string(),Animation{start:1,end:1,looping:true,cooldown: 0.1});

    return hash_map;
}