use crate::plugins::animation::{components::*, systems::*};
use bevy::prelude::*;

pub struct AnimationPlugin;
impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<AnimEvent<Handle<TextureAtlas>>>()
            .add_system(animate_sprite_system);
    }
}
