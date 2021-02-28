use crate::plugins::core::{components::*, systems::*};
use bevy::prelude::*;

pub struct CorePlugin;
impl Plugin for CorePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<AnimEvent<Handle<TextureAtlas>>>()
            .add_system(animate_sprite.system());
    }
}
