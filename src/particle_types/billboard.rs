
use serde::{Serialize, Deserialize};
use bevy::{prelude::*, render::render_resource::{Extent3d, TextureDimension, TextureFormat, TextureDescriptor, TextureUsages}, utils::HashMap};
use bevy_hanabi::prelude::*;
use std::{error::Error, str::Bytes};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct BillboardEffectBuilder {
    pub name: String,
   /* pub color_gradient_keys: Vec<(f32, Vec4)>,
    pub size_gradient_keys: Vec<(f32, Vec2)>,
    pub spawn_rate: f32,
    pub particle_lifetime_min: f32,
    pub particle_lifetime_max: f32,
    pub initial_velocity: Vec3,
    pub tangent_acceleration: f32,*/
    
   pub texture_name: String
}

impl BillboardEffectBuilder {
   

    // Methods to add gradient keys, set properties, etc., could be added here

    pub fn build(
        &self,
        image_handle_map: HashMap<String, Handle<Image>>,
        
        
        ) -> Option< EffectAsset > {
          
      
    let texture_handle  = image_handle_map.get( &self.texture_name )?;
    

    let writer = ExprWriter::new();

    let age = writer.lit(0.).expr();
    let init_age = SetAttributeModifier::new(Attribute::AGE, age);

    let lifetime = writer.lit(5.).expr();
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    let init_pos = SetPositionCircleModifier {
        center: writer.lit(Vec3::Y * 0.1).expr(),
        axis: writer.lit(Vec3::Y).expr(),
        radius: writer.lit(1.).expr(),
        dimension: ShapeDimension::Volume,
    };

    let init_vel = SetVelocityCircleModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        axis: writer.lit(Vec3::Y).expr(),
        speed: (writer.lit(0.5) + writer.lit(0.2) * writer.rand(ScalarType::Float)).expr(),
    };

    // To give some visual diversity, we initialize each spawned particle with a
    // random per-particle color. The COLOR attribute is read back in the vertex
    // shader to initialize the particle's base color, which is later modulated
    // in this example with the texture of the ParticleTextureModifier.
    // Note that the ParticleTextureModifier uses
    // ImageSampleMapping::ModulateOpacityFromR so it will override
    // the alpha component of the color. Therefore we don't need to care about
    // rand() assigning a transparent value and making the particle invisible.
    let color = writer.rand(VectorType::VEC4F).pack4x8unorm();
    let init_color = SetAttributeModifier::new(Attribute::COLOR, color.expr());

    // Use the F32_0 attribute as a per-particle rotation value, initialized on
    // spawn and constant after. The rotation angle is in radians, here randomly
    // selected in [0:2*PI].
    let rotation = (writer.rand(ScalarType::Float) * writer.lit(std::f32::consts::TAU)).expr();
    let init_rotation = SetAttributeModifier::new(Attribute::F32_0, rotation);

    // Bounce the alpha cutoff value between 0 and 1, to show its effect on the
    // alpha masking
    let alpha_cutoff =
        ((writer.time() * writer.lit(2.)).sin() * writer.lit(0.3) + writer.lit(0.4)).expr();

    // The rotation of the OrientModifier is read from the F32_0 attribute (our
    // per-particle rotation)
    let rotation_attr = writer.attr(Attribute::F32_0).expr();

    let   module = writer.finish();
        
    let effect =  
        EffectAsset::new(32768, Spawner::rate(64.0.into()), module)
            .with_name("billboard")
            .with_alpha_mode(bevy_hanabi::AlphaMode::Mask(alpha_cutoff))
            .init(init_pos)
            .init(init_vel)
            .init(init_age)
            .init(init_lifetime)
            .init(init_rotation)
            .init(init_color)
            .render(ParticleTextureModifier {
                texture: texture_handle.clone(),
                sample_mapping: ImageSampleMapping::ModulateOpacityFromR,
            })
            .render(OrientModifier {
                mode: OrientMode::FaceCameraPosition,
                rotation: Some(rotation_attr),
            })
            .render(SizeOverLifetimeModifier {
                gradient: Gradient::constant([0.2; 2].into()),
                screen_space_size: false,
            })   ;
       
       
       /* let mut color_gradient = Gradient::new();
        for (time, color) in &self.color_gradient_keys {
            color_gradient.add_key(*time, *color);
        }

        let mut size_gradient = Gradient::new();
        for (time, size) in &self.size_gradient_keys {
            size_gradient.add_key(*time, *size);
        }
  
        
        let writer = ExprWriter::new();

        let init_pos = SetPositionCircleModifier {
            center: writer.lit(Vec3::ZERO).expr(),
            axis: writer.lit(Vec3::Z).expr(),
            radius: writer.lit(4.).expr(),
            dimension: ShapeDimension::Surface,
        };
    
        let age = writer.lit(0.0).expr();
        let init_age = SetAttributeModifier::new(Attribute::AGE, age);
    
        // Give a bit of variation by randomizing the lifetime per particle
        let lifetime = writer.lit(self.particle_lifetime_min).uniform(writer.lit(self.particle_lifetime_max)).expr();
        let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);
    
        // Add drag to make particles slow down a bit after the initial acceleration
        let drag = writer.lit(2.0).expr();
        let update_drag = LinearDragModifier::new(drag);
    
        let mut module = writer.finish();
    
     
        let tangent_accel = TangentAccelModifier::constant(&mut module, self.initial_velocity, Vec3::Z, self.tangent_acceleration);
        */
      /*  let effect =  
            EffectAsset::new(32768, Spawner::rate(self.spawn_rate.into()), module)
                .with_name(self.name.clone())
                .init(init_pos)
                 .init(init_age)
                .init(init_lifetime)
                .update(update_drag)
                .update(tangent_accel)
                .render(ColorOverLifetimeModifier {
                    gradient: color_gradient,
                })
                .render(SizeOverLifetimeModifier {
                    gradient: size_gradient,
                    screen_space_size: false,
                })
                .render(OrientModifier::new(OrientMode::AlongVelocity));
        
            */
    

        Some(effect)
    }
}


 