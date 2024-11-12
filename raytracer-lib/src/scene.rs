use crate::prelude::*;
use std::str::FromStr;
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
    ref_name: String,
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
    shader: ShaderRef,
    #[serde(deserialize_with = "deserialize_vec3")]
    center: Vec3,
    radius: Real,
    #[serde(rename = "_name")]
    name: String,
}

#[derive(Deserialize, Debug)]
pub struct BoxData {
    shader: ShaderRef,
    #[serde(rename = "minPt", deserialize_with = "deserialize_vec3")]
    min_point: Vec3,
    #[serde(rename = "maxPt", deserialize_with = "deserialize_vec3")]
    max_point: Vec3,
    #[serde(rename = "_name")]
    name: String,
}

pub fn load_scene(path: &str) -> Result<SceneData, Box<dyn std::error::Error>> {
    let file = std::fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);
    let scene: SceneFile = serde_json::from_reader(reader)?;

    Ok(scene.scene)
}

pub struct Scene<'a> {
    pub camera: Box<dyn crate::camera::Camera>,
    pub shapes: Vec<Box<dyn crate::geometry::Shape<'a>>>,
    pub shaders: HashMap<&'static str, Box<dyn crate::shader::Shader>>,
}

impl<'a> Scene<'a> {}
