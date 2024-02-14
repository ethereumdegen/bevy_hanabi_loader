/*

  let x: MyStruct = ron::from_str("(boolean: true, float: 1.23)").unwrap();

    println!("RON: {}", ron::to_string(&x).unwrap());
    
    

*/


use serde::{Serialize, Deserialize};
use bevy::{prelude::*, utils::HashMap};
use bevy_hanabi::prelude::*;
 

use crate::particle_types::{portal::PortalEffectBuilder, billboard::BillboardEffectBuilder};


#[derive(Asset, Debug, Serialize, Deserialize)]
pub  enum HanabiEffectBuilder {
    Portal(PortalEffectBuilder),
    Billboard(BillboardEffectBuilder)
}

impl HanabiEffectBuilder{
    
    pub fn build(&self,
        image_handle_map: &HashMap<String, Handle<Image>>
        ) -> Option<EffectAsset> {
            
        
        match self {
            HanabiEffectBuilder::Portal(portal) => portal.build( ),
            HanabiEffectBuilder::Billboard(billboard) => billboard.build(image_handle_map)    
        }
        
        
    }
}

impl TypePath for HanabiEffectBuilder {
    
      fn short_type_path() -> &'static str {
        "bvfx"
       }
       fn type_path() -> &'static str {
        "bvfx"
       }
    
}

