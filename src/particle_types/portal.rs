
use serde::{Serialize, Deserialize};
use bevy::{prelude::*, utils::HashMap};
use bevy_hanabi::prelude::*;
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct PortalEffectBuilder {
    pub name: String,
    pub color_gradient_keys: Vec<(f32, Vec4)>,
    pub size_gradient_keys: Vec<(f32, Vec2)>,
    pub spawn_rate: f32,
    pub particle_lifetime_min: f32,
    pub particle_lifetime_max: f32,
    pub initial_velocity: Vec3,
    pub tangent_acceleration: f32,
}

impl PortalEffectBuilder {
   

    // Methods to add gradient keys, set properties, etc., could be added here

    pub fn build(
        &self,
        
        
        ) -> Option<EffectAsset> {
         
        
        let mut color_gradient = Gradient::new();
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

        let effect =  
            EffectAsset::new(vec![32768], Spawner::rate(self.spawn_rate.into()), module)
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
        

    

        Some(effect)
    }
}