
### Bevy Hanabi Loader


Define a vfx file like this as a RON file 

```

BillboardEffectBuilder(
    name: "CloudBillboard",
    texture_name: "cloud",
    
    age: Constant(0.0),
    lifetime: Constant(5.0),
      
    position_center: RandomVector( offset:(-2.0,-2.0,-2.0),multiplier:(4.0,4.0,4.0) )  ,
    position_radius: RandomScalar( offset:0.0,multiplier:8.0 ),
    
    velocity_center: RandomVector( offset:(-2.0,-0.2,0.2),multiplier:(0.0,1.0,4.0) )  ,
    velocity_speed: RandomScalar( offset:0.0,multiplier:8.0 ),
    
    
    color_base: ( 0.9, 0.1, 0.1, 1.0 ),
    color_random_multiplier: ( 0.0,0.6,0.0,0.0 ),
    
    rotation: RandomScalar (offset: 0.0, multiplier: 6.28 ) ,
    alpha_cutoff: TimeSinewave (time_scalar: 2.0, multiplier: 0.3, offset: 0.4)
)



```



Then you can load it... see src/examples/basic.rs 