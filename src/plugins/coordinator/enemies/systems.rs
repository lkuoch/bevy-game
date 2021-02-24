use crate::coordinator::enemies::{components::*, vars::*};
use bevy::prelude::*;

pub fn enemies_setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut enemies: ResMut<Enemies>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    for enemy in enemies::ENEMY_LIST.iter() {
        for texture in enemy.animation_states.iter() {
            let path = format!("{}{}", enemy.root_path.to_owned(), texture.path.to_owned());

            let texture_handle: Handle<Texture> = asset_server.load(&path[..]);
            let handle = texture_atlases.add(TextureAtlas::from_grid(
                texture_handle,
                Vec2::new(32.0, 32.0),
                texture.frames,
                1,
            ));

            match enemy.ty {
                EnemyType::AngryPig => {
                    enemies.textures.insert(
                        EnemySpriteMapKey::State(EnemyTypeKey::AngryPig(texture.state)),
                        EnemySpriteKV::Handle(handle.clone()),
                    );

                    enemies.textures.insert(
                        EnemySpriteKV::Handle(handle.clone()),
                        EnemySpriteKV::State(texture.state),
                    );
                }
                EnemyType::Bat => {}
                EnemyType::Bee => {}
                EnemyType::BlueBird => {}
                EnemyType::Bunny => {}
                EnemyType::Chameleon => {}
                EnemyType::Chicken => {}
                EnemyType::Duck => {}
                EnemyType::FatBird => {}
                EnemyType::Ghost => {}
                EnemyType::Mushroom => {}
                EnemyType::Plant => {}
                EnemyType::Radish => {}
                EnemyType::Rino => {}
                EnemyType::Rocks => {}
                EnemyType::Skull => {}
                EnemyType::Slime => {}
                EnemyType::Snail => {}
                EnemyType::Trunk => {}
                EnemyType::Turtle => {}
            }
        }
    }

    // Let's just spawn angry pig
    if let Some(EnemySpriteKV::Handle(enemy)) =
        enemies
            .textures
            .get(&EnemySpriteKV::State(EnemyTypeKey::AngryPig(
                enemies::States::Idle,
            )))
    {
        commands.spawn(SpriteSheetBundle {
            texture_atlas: enemy.clone(),
            transform: Transform::from_xyz(150., 0., 0.)
                .mul_transform(Transform::from_scale(Vec3::splat(2.5))),
            ..Default::default()
        });
    }
}
