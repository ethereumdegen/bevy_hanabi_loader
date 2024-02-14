
use bevy::prelude::Vec3;
use bevy_hanabi::{ExprWriter, ExprHandle, ScalarType};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AlgebraicCurve {
    Constant( f32 ),
    RandomScalar { offset: f32, multiplier: f32  }, 
    TimeLinear { time_scalar: f32, multiplier: f32, offset:f32  },
    TimeSinewave { time_scalar: f32, multiplier: f32, offset:f32 },
}
impl AlgebraicCurve {
    
    pub fn to_expr(self, writer: &ExprWriter) -> ExprHandle{
        
        match self {
            AlgebraicCurve::Constant(x) => writer.lit( x  ).expr(),
            AlgebraicCurve::RandomScalar{offset, multiplier} => (writer.lit(offset) + writer.lit(multiplier) * writer.rand(ScalarType::Float)).expr(),
            AlgebraicCurve::TimeLinear { time_scalar, multiplier, offset } => ((writer.time() * writer.lit(time_scalar))  * writer.lit(multiplier) + writer.lit(offset)).expr(),
            AlgebraicCurve::TimeSinewave { time_scalar, multiplier, offset } =>((writer.time() * writer.lit(time_scalar)).sin() * writer.lit(multiplier) + writer.lit(offset)).expr()
        }
        
    } 
}
 
 
 

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AlgebraicVector {
    Constant( Vec3 ),
    RandomScalar{offset: Vec3, multiplier: Vec3 },
}
impl AlgebraicVector {
    
    pub fn to_expr(self, writer: &ExprWriter) -> ExprHandle{
        
        match self {
           AlgebraicVector::Constant(x) => writer.lit( x  ).expr(),
           AlgebraicVector::RandomScalar { offset, multiplier } => (writer.lit(offset) + writer.lit(multiplier.x) * writer.rand(ScalarType::Float)).expr(),
        }
        
    } 
}
 
 
 
