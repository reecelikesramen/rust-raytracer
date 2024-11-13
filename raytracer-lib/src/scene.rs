use crate::{camera::{OrthographicCamera, PerspectiveCamera}, geometry::Sphere, prelude::*, shader::{LambertianShader, Shader}};
use std::{convert::TryInto, str::FromStr};
use nalgebra::Vector3;
use serde::{Deserialize, Deserializer, de};
use std::collections::HashMap;
use {vec3, color};

fn parse_vec3<FloatType>(s: &str) -> Result<Vector3<FloatType>, String> 
where
    FloatType: Into<f64> + Copy + FromStr, <FloatType as FromStr>::Err: std::fmt::Display
{
    let numbers: Result<Vec<FloatType>, _> = s
        .split_whitespace()
        .map(FloatType::from_str)
        .collect();
    
    match numbers {
        Ok(nums) if nums.len() == 3 => Ok(Vector3::new(nums[0], nums[1], nums[2])),
        Ok(_) => Err("expected exactly 3 space-separated numbers".to_string()),
        Err(e) => Err(format!("failed to parse number: {}", e)),
    }
}

fn deserialize_vec3<'de, D, FloatType>(deserializer: D) -> Result<Vector3<FloatType>, D::Error>
where
    D: Deserializer<'de>,
    FloatType: Into<f64> + Copy + FromStr, <FloatType as FromStr>::Err: std::fmt::Display
{
    let s: String = String::deserialize(deserializer)?;
    parse_vec3(&s).map_err(de::Error::custom)
}


fn deserialize_optional_vec3<'de, D, FloatType>(deserializer: D) -> Result<Option<Vector3<FloatType>>, D::Error>
where
    D: Deserializer<'de>,
    FloatType: Into<f64> + Copy + FromStr, <FloatType as FromStr>::Err: std::fmt::Display
{
    // First deserialize to Option<String>
    let opt: Option<String> = Option::deserialize(deserializer)?;
    
    match opt {
        Some(s) => parse_vec3(&s).map(Some).map_err(de::Error::custom),
        None => Ok(None),
    }
}

#[derive(Deserialize, Debug)]
struct SceneFile {
    scene: SceneData,
}

#[derive(Deserialize, Debug)]
pub struct SceneData {
    camera: Vec<CameraData>,
    light: Vec<LightData>,
    shader: Vec<ShaderType>,
    shape: Vec<ShapeType>,
    #[serde(rename = "_bgColor", default)]
    background_color: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct CameraData {
    #[serde(deserialize_with = "deserialize_vec3")]
    position: Vec3,
    #[serde(rename = "viewDir", default, deserialize_with = "deserialize_optional_vec3")]
    view_dir: Option<Vec3>,
    #[serde(rename = "lookatPoint", default, deserialize_with = "deserialize_optional_vec3")]
    lookat_point: Option<Vec3>,
    #[serde(rename = "focalLength")]
    focal_length: Real,
    #[serde(rename = "imagePlaneWidth", default)]
    image_plane_width: Option<Real>,
    #[serde(rename = "_name")]
    name: String,
    #[serde(rename = "_type")]
    camera_type: String,
}

#[derive(Deserialize, Debug)]
pub struct LightData {
    #[serde(deserialize_with = "deserialize_vec3")]
    position: Vec3,
    #[serde(deserialize_with = "deserialize_vec3")]
    intensity: Color,
    #[serde(rename = "_type")]
    light_type: String,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "_type")]
pub enum ShaderType {
    #[serde(rename = "Lambertian")]
    Lambertian(LambertianShaderData),
    #[serde(rename = "BlinnPhong")]
    BlinnPhong(BlinnPhongShaderData),
    #[serde(rename = "Mirror")]
    Mirror(MirrorShaderData),
}

#[derive(Deserialize, Debug)]
pub struct LambertianShaderData {
    #[serde(deserialize_with = "deserialize_vec3")]
    diffuse: Color,
    #[serde(rename = "_name")]
    name: String,
}

#[derive(Deserialize, Debug)]
pub struct BlinnPhongShaderData {
    #[serde(deserialize_with = "deserialize_vec3")]
    diffuse: Color,
    #[serde(deserialize_with = "deserialize_vec3")]
    specular: Color,
    #[serde(rename = "phongExp")]
    phong_exp: Real,
    #[serde(rename = "_name")]
    name: String,
}

#[derive(Deserialize, Debug)]
pub struct MirrorShaderData {
    roughness: Real,
    #[serde(rename = "_name")]
    name: String,
}

#[derive(Deserialize, Debug)]
pub struct ShaderRef {
    #[serde(rename = "_ref")]
    name: String,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "_type")]
pub enum ShapeType {
    #[serde(rename = "sphere")]
    Sphere(SphereData),
    #[serde(rename = "box")]
    Box(BoxData),
}

#[derive(Deserialize, Debug)]
pub struct SphereData {
    #[serde(rename = "shader")]
    shader: ShaderRef,
    #[serde(deserialize_with = "deserialize_vec3")]
    center: Vec3,
    radius: Real,
    #[serde(rename = "_name")]
    name: String,
}

#[derive(Deserialize, Debug)]
pub struct BoxData {
    #[serde(rename = "shader")]
    shader: ShaderRef,
    #[serde(rename = "minPt", deserialize_with = "deserialize_vec3")]
    min_point: Vec3,
    #[serde(rename = "maxPt", deserialize_with = "deserialize_vec3")]
    max_point: Vec3,
    #[serde(rename = "_name")]
    name: String,
}

pub fn load_scene(path: &str, image_width: u32, image_height: u32, aspect_ratio: Real) -> Result<Scene, Box<dyn std::error::Error>> {
    let file = std::fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);
    let scene_file: SceneFile = serde_json::from_reader(reader)?;
    let scene = scene_file.scene;

    // Check that there is exactly one camera
    if scene.camera.len() != 1 {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "scene must have exactly one camera",
        )));
    }

    // Check that view_dir or lookat_point is specified
    if scene.camera[0].view_dir.is_none() && scene.camera[0].lookat_point.is_none() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "camera must have view_dir or lookat_point specified",
        )));
    }

    // View direction or calculate from camera position and lookat point
    let view_dir = match scene.camera[0].view_dir {
        Some(v) => v,
        // calculate view dir from camera position and lookat point
        None => scene.camera[0].position - scene.camera[0].lookat_point.unwrap(),
    };

    // Create camera
    let camera: Box<dyn crate::camera::Camera> = match scene.camera[0].camera_type.as_str() {
        "Perspective" => Box::new(PerspectiveCamera::new(
            scene.camera[0].position,
            &view_dir,
            aspect_ratio,
            scene.camera[0].focal_length,
        )),
        "Orthographic" => Box::new(OrthographicCamera::new(
            scene.camera[0].position,
            &view_dir,
            aspect_ratio,
        )),
        _ => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "camera type not supported",
            )))
        }
    };

    // Create shaders
    let mut shaders: HashMap<&'static str, Box<dyn crate::shader::Shader>> = HashMap::new();
    for shader in scene.shader.iter() {
        match shader {
            ShaderType::Lambertian(lambertian) => {
                // Convert the name to a static str - this is safe as long as the names are constant
                let name = Box::leak(lambertian.name.clone().into_boxed_str());
                shaders.insert(name, Box::new(LambertianShader::new(name, lambertian.diffuse, None)));
            }
            _ => {
                unimplemented!("shader type not supported yet")
            }
        }
    }

    // Create shapes
    let mut shapes: Vec<Box<dyn crate::geometry::Shape>> = Vec::new();
    for shape in scene.shape.iter() {
        match shape {
            ShapeType::Sphere(sphere) => {
                let shader_name = Box::leak(sphere.shader.name.clone().into_boxed_str());
                let shader = match shaders.get(shader_name) {
                    Some(s) => s.as_ref(),
                    None => {
                        return Err(Box::new(std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            "shape references non-existent shader",
                        )))
                    }
                };
                let shape = Sphere::new(sphere.center, sphere.radius, shader);
                shapes.push(Box::new(shape));
            }
            // ShapeType::Box(box_shape) => {
            //     let shader = get_shader(&shaders, box_shape.shader.name.as_str())?;
            //     let shape = BoxShape::new(box_shape.min_point, box_shape.max_point, shader);
            //     shapes.push(Box::new(shape));
            // }
            _ => {
                unimplemented!("shape type not supported yet")
            }
        }
    }

    let scene = Scene {
        camera,
        shapes,
        shaders,
    };
    return Ok(scene);
}

pub struct Scene<'a> {
    pub camera: Box<dyn crate::camera::Camera>,
    pub shapes: Vec<Box<dyn crate::geometry::Shape<'a>>>,
    pub shaders: HashMap<&'static str, Box<dyn crate::shader::Shader>>,
}

// impl<'a> Scene<'a> {}
