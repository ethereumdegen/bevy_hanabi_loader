/*

  let x: MyStruct = ron::from_str("(boolean: true, float: 1.23)").unwrap();

    println!("RON: {}", ron::to_string(&x).unwrap());
    
    

*/


use serde::{Serialize, Deserialize};
use bevy::{prelude::* };


use bevy::platform::collections::hash_map::HashMap ;

use bevy_hanabi::prelude::*;
 

use crate::particle_types::{portal::PortalEffectBuilder, billboard::BillboardEffectBuilder};


#[derive(  Clone )]
pub struct BuiltHanabiEffect {

    pub effect_asset: EffectAsset,
    pub effect_material: Option<EffectMaterial>
}

#[derive(Asset, Debug, Serialize, Deserialize)]
pub  enum HanabiEffectBuilder {
    Portal(PortalEffectBuilder),
    Billboard(BillboardEffectBuilder)
}


impl HanabiEffectBuilder{
    
    pub fn build(&self,
        image_handle_map: &HashMap<String, Handle<Image>>
        ) -> Option< BuiltHanabiEffect > {
            
        
        match self {
            HanabiEffectBuilder::Portal(portal) => portal.build( ),
            HanabiEffectBuilder::Billboard(billboard) => billboard.build(image_handle_map)    
        }
        
        
    }

       
    pub fn get_name(&self ) -> &String { 
        
        match self {
            HanabiEffectBuilder::Portal(portal) => &portal.name,
            HanabiEffectBuilder::Billboard(billboard) => &billboard.name   
        }
        
        
    }
}

impl TypePath for HanabiEffectBuilder {
    
      fn short_type_path() -> &'static str {
        "hvfx.ron"
       }
       fn type_path() -> &'static str {
        "hvfx.ron"
       }
    
}

