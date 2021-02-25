use crate::debug::components::*;
use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

pub fn setup(commands: &mut Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            text: Text::with_section(
                "FPS:",
                TextStyle {
                    font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                    font_size: 30.0,
                    color: Color::WHITE,
                },
                TextAlignment {
                    ..Default::default()
                },
            ),
            ..Default::default()
        })
        .with(FPSText);
}

pub fn text_update(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FPSText>>) {
    for mut text in query.iter_mut() {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(avg) = fps.average() {
                text.sections[0].value = format!("FPS: {avg:.2}");
            }
        }
    }
}
