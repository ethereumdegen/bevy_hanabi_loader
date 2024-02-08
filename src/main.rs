//! Portal
//!
//! An example demonstrating the use of the `TangentAccelModifier` to create a
//! kind of portal effect where particles turn around a circle and appear to be
//! ejected from it.
//!
//! The `OrientMode::AlongVelocity` of the `OrientModifier` paired with an
//! elongated particle size gives the appearance of sparks.
//!
//! The addition of some gravity and drag, combined with a careful choice of
//! lifetime, give a subtle effect of particles appearing to fall down right
//! before they disappear, like sparkles fading away.

use bevy::{
    core_pipeline::{
        bloom::BloomSettings, clear_color::ClearColorConfig, tonemapping::Tonemapping,
    },
    log::LogPlugin,
    prelude::*,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bevy_hanabi::prelude::*;
use hanabi_effect_builder::HanabiEffectBuilder;

pub mod hanabi_effect_builder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    App::default()
        .add_plugins(
            DefaultPlugins
                .set(LogPlugin {
                    level: bevy::log::Level::WARN,
                    filter: "bevy_hanabi=warn,portal=trace".to_string(),
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "🎆 Hanabi — portal".to_string(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_systems(Update, bevy::window::close_on_esc)
        .add_plugins(HanabiPlugin)
        .add_plugins(WorldInspectorPlugin::default())
        .add_systems(Startup, setup)
        .run();

    Ok(())
}

fn setup(mut commands: Commands, mut effects: ResMut<Assets<EffectAsset>>) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0., 0., 25.)),
            camera: Camera {
                hdr: true,
                ..default()
            },
            camera_3d: Camera3d {
                clear_color: ClearColorConfig::Custom(Color::BLACK),
                ..default()
            },
            tonemapping: Tonemapping::None,
            ..default()
        },
        BloomSettings::default(),
    ));

  
    
  let effect_builder = HanabiEffectBuilder {
        name: "portal".into(),
        color_gradient_keys: vec![
            (0.0, Vec4::new(4.0, 4.0, 4.0, 1.0)),
            (0.1, Vec4::new(4.0, 4.0, 0.0, 1.0)),
            (0.9, Vec4::new(4.0, 0.0, 0.0, 1.0)),
            (1.0, Vec4::new(4.0, 0.0, 0.0, 0.0)),
        ],
        size_gradient_keys: vec![
            (0.3, Vec2::new(0.2, 0.02)),
            (1.0, Vec2::splat(0.0)),
        ],
        spawn_rate: 5000.0,
        particle_lifetime_min: 0.6,
        particle_lifetime_max: 1.3,
        initial_velocity: Vec3::ZERO, // This might need to be adjusted based on specific effect requirements
        tangent_acceleration: 30.0,
    };

   
    
    let effect_handle = effects.add( effect_builder.build() );
    
    

    commands.spawn((
        Name::new("portal"),
        ParticleEffectBundle {
            effect: ParticleEffect::new(effect_handle),
            transform: Transform::IDENTITY,
            ..Default::default()
        },
    ));
}
