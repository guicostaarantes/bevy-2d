use bevy::prelude::*;

use crate::plugins::animation::components::*;
use crate::plugins::movement::components::*;

pub fn animate_entities_that_are_able_to_animate(
    movables: Query<(&Children, &HasPosition, Option<&AbleToMove>)>,
    mut sprites: Query<(
        &mut AbleToAnimate,
        &mut Handle<TextureAtlas>,
        &mut TextureAtlasSprite,
    )>,
    time: Res<Time>,
) {
    movables.iter().for_each(|(children, position, movable)| {
        children.iter().for_each(|child| {
            if let Ok((mut animated, mut atlas_handle, mut sprite)) = sprites.get_mut(*child) {
                animated.timer_to_animate.tick(time.delta());
                if !animated.timer_to_animate.just_finished() {
                    return;
                }

                if animated.current_frame + 1 == animated.number_of_frames {
                    animated.current_frame = 0;
                } else {
                    animated.current_frame += 1;
                }
                sprite.index = animated.number_of_frames * position.dir.get_sprite_index()
                    + animated.current_frame;
                sprite.flip_x = position.dir.should_flip_x();

                if let Some(mov) = movable {
                    if *atlas_handle == animated.texture_atlas && mov.current_speed.length() > 0. {
                        *atlas_handle = animated.walking_texture_atlas.clone()
                    } else if *atlas_handle == animated.walking_texture_atlas
                        && mov.current_speed.length() == 0.
                    {
                        *atlas_handle = animated.texture_atlas.clone()
                    }
                }
            }
        });
    });
}
