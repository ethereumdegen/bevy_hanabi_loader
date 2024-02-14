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

use std::fs::File;
use std::io::Read;


use bevy::{
    core_pipeline::{
        bloom::BloomSettings, clear_color::ClearColorConfig, tonemapping::Tonemapping,
    },
    log::LogPlugin,
    prelude::*, render::{settings::WgpuSettings, render_resource::WgpuFeatures, RenderPlugin}, utils::HashMap,
};

 

use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bevy_hanabi::prelude::*;
use hanabi_effect_builder::HanabiEffectBuilder;
use particle_types::{portal::PortalEffectBuilder, billboard::BillboardEffectBuilder};

pub mod hanabi_effect_builder;
pub mod particle_types;

pub mod util;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    //need this ..
  let mut wgpu_settings = WgpuSettings::default();
    wgpu_settings
        .features
        .set(WgpuFeatures::VERTEX_WRITABLE_STORAGE, true);
    
    
    
    App::default()
        .add_plugins(
            DefaultPlugins
                .set(LogPlugin {
                    level: bevy::log::Level::WARN,
                    filter: "bevy_hanabi=warn,portal=trace".to_string(),
                })
                
                 .set(RenderPlugin {
                    render_creation: wgpu_settings.into(),
                })  //need me for billboards !? 
                
                
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

fn setup(
    mut commands: Commands, 
    mut effects: ResMut<Assets<EffectAsset>>,
    asset_server: Res<AssetServer>
    
    )   {
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

  
    
    
    let mut image_handle_map:HashMap<String,Handle<Image>> = HashMap::new();
    image_handle_map.insert( "cloud".into(),asset_server.load("cloud.png")  );
    
    
    /*
  let effect_builder = HanabiEffectBuilder::Portal(PortalEffectBuilder {
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
    });
 
    
    
    let effect_handle = effects.add( effect_builder.build(  &image_handle_map ).unwrap() );
    
    
    commands.spawn((
        Name::new("portal"),
        ParticleEffectBundle {
            effect: ParticleEffect::new(effect_handle),
            transform: Transform::IDENTITY,
            ..Default::default()
        },
    )); 
    */
    
    
    
    
    
    let mut file = File::open("assets/cloud.bvfx").unwrap();
    let mut contents = String::new(); 
    file.read_to_string(&mut contents).unwrap();
    let effect_builder: BillboardEffectBuilder = ron::from_str(&contents).unwrap();
     
     
    let mut image_handle_map:HashMap<String,Handle<Image>> = HashMap::new();
    image_handle_map.insert( "cloud".into(),asset_server.load("cloud.png")  );
     
    let effect_handle = effects.add( effect_builder.build(  &image_handle_map ).unwrap() );
     

    commands.spawn((
        Name::new("billboard"),
        ParticleEffectBundle {
            effect: ParticleEffect::new(effect_handle),
            transform: Transform::IDENTITY,
            ..Default::default()
        },
    ));
}
