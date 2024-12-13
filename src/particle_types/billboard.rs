
use serde::{Serialize, Deserialize};
use bevy::{prelude::*, render::render_resource::{Extent3d, TextureDimension, TextureFormat, TextureDescriptor, TextureUsages}, utils::HashMap};
use bevy_hanabi::prelude::*;
use std::{error::Error, str::Bytes};

use crate::{hanabi_effect_builder::BuiltHanabiEffect, util::{AlgebraicCurve, AlgebraicVector}};


#[derive(Debug, Serialize, Deserialize)]
pub struct BillboardEffectBuilder {
    pub name: String,
    
    pub spawn_rate: f32, // default at 64.0.into() ? 
    pub age: AlgebraicCurve,
    pub lifetime: AlgebraicCurve,
     
    pub position_center: AlgebraicVector,
    pub position_radius: AlgebraicCurve,
    
    pub velocity_center: AlgebraicVector,
    pub velocity_speed: AlgebraicCurve, 
    
    pub rotation : AlgebraicCurve, 
    
    pub color_base : Vec4,  
    pub color_random_multiplier : Vec4, 
    
    pub alpha_cutoff : AlgebraicCurve, 
    
    pub texture_name: String
}


 


impl BillboardEffectBuilder   {
   

    // Methods to add gradient keys, set properties, etc., could be added here

    pub fn build(
        &self,
        image_handle_map: &HashMap<String, Handle<Image>>,
        
        
        ) -> Option<  BuiltHanabiEffect > {
          
      
    let texture_handle  = image_handle_map.get( &self.texture_name )?;
    

    let writer = ExprWriter::new();

    let age = self.age.clone().to_expr(&writer); // 0.0
    let init_age = SetAttributeModifier::new(Attribute::AGE, age);

    let lifetime = self.lifetime.clone().to_expr(&writer);   //5.0
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);
 
    
     let init_pos   =  SetPositionSphereModifier {
                    center: self.position_center.clone().to_expr(&writer),
                    radius: self.position_radius.clone().to_expr(&writer), //writer.lit(1.).expr(),
                    dimension: ShapeDimension::Volume,
                };
            
         
    
     
    
     let init_vel  =  
                SetVelocitySphereModifier {
                  center: self.velocity_center.clone().to_expr(&writer),
                  speed: self.velocity_speed.clone().to_expr(&writer)
                }     ;
 
   
    
    
   // let position = self.position.clone().to_expr(&writer) ; //(writer.rand(ScalarType::Float) * writer.lit(std::f32::consts::TAU)).expr();
   // let init_pos = SetAttributeModifier::new(Attribute::POSITION, position);
    

    // To give some visual diversity, we initialize each spawned particle with a
    // random per-particle color. The COLOR attribute is read back in the vertex
    // shader to initialize the particle's base color, which is later modulated
    // in this example with the texture of the ParticleTextureModifier.
    // Note that the ParticleTextureModifier uses
    // ImageSampleMapping::ModulateOpacityFromR so it will override
    // the alpha component of the color. Therefore we don't need to care about
    // rand() assigning a transparent value and making the particle invisible.
    let color = (writer.rand(VectorType::VEC4F) * writer.lit(self.color_random_multiplier) +  writer.lit(self.color_base) ).pack4x8unorm();
    let init_color = SetAttributeModifier::new(Attribute::COLOR, color.expr());

    // Use the F32_0 attribute as a per-particle rotation value, initialized on
    // spawn and constant after. The rotation angle is in radians, here randomly
    // selected in [0:2*PI].
    let rotation = self.rotation.clone().to_expr(&writer) ; //(writer.rand(ScalarType::Float) * writer.lit(std::f32::consts::TAU)).expr();
    let init_rotation = SetAttributeModifier::new(Attribute::F32_0, rotation);

    // Bounce the alpha cutoff value between 0 and 1, to show its effect on the
    // alpha masking
    let alpha_cutoff = self.alpha_cutoff.clone().to_expr(&writer);
       // ((writer.time() * writer.lit(2.)).sin() * writer.lit(0.3) + writer.lit(0.4)).expr();

    // The rotation of the OrientModifier is read from the F32_0 attribute (our
    // per-particle rotation)
    let rotation_attr = writer.attr(Attribute::F32_0).expr();

      let texture_slot = writer.lit(0u32).expr();

    let   mut module = writer.finish();

    module.add_texture_slot("color");


    let spawn_rate = self.spawn_rate.clone().into();

    let size_over_lifetime = Gradient::constant([0.2; 3].into()); // for now 
        
    let effect_asset =  
        EffectAsset::new( 32768 , Spawner::rate(spawn_rate), module)
            .with_name("billboard")
            .with_alpha_mode(bevy_hanabi::AlphaMode::Blend)
            .init (init_pos  )
            .init (init_vel )
            .init(init_age)
            .init(init_lifetime)
            .init(init_rotation)
            .init(init_color)
            .render(ParticleTextureModifier {
                texture_slot: texture_slot , // this is so weird !! 
                sample_mapping: ImageSampleMapping::ModulateOpacityFromR,
            })
            .render(OrientModifier {
                mode: OrientMode::FaceCameraPosition,
                rotation: Some(rotation_attr),
            })
            .render(SizeOverLifetimeModifier {
                gradient:  size_over_lifetime ,
                screen_space_size: false,
            })   ;

    let effect_material = EffectMaterial {
            images: vec![texture_handle.clone()],
        };

       
        

        Some(BuiltHanabiEffect{ effect_asset, effect_material: Some(effect_material)})
    }
}


 