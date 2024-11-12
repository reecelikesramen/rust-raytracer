use std::collections::HashMap;

pub struct Scene<'a> {
    pub camera: Box<dyn crate::camera::Camera>,
    pub shapes: Vec<Box<dyn crate::geometry::Shape<'a>>>,
    pub shaders: HashMap<&'static str, Box<dyn crate::shader::Shader>>,
}

impl<'a> Scene<'a> {
    
}
